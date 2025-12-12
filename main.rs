use clap::Parser;
use graphql_parser::query::{parse_query, Definition, Selection, OperationDefinition};
use regex::Regex;
use reqwest::header;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::sync::OnceLock;
use std::time::Instant;
use url::Url;

//
// ============================ CLI ============================
//

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    url: String,
}

//
// ============================ MODELS ============================
//

#[derive(Clone)]
struct GqlOperation {
    name: String,
    op_type: String,
    body: String,
    source: String, // Added to track where it came from
}

#[derive(Clone)]
struct ObjectType {
    name: String,
    fields: HashSet<String>,
}

#[derive(Clone)]
struct InterfaceType {
    name: String,
    fields: HashSet<String>,
}

//
// ============================ REGEX ============================
//

static RE_OP: OnceLock<Regex> = OnceLock::new();
static RE_JS: OnceLock<Regex> = OnceLock::new();
static RE_ENUM: OnceLock<Regex> = OnceLock::new();

fn re_op() -> &'static Regex {
    RE_OP.get_or_init(|| {
        Regex::new(r"(query|mutation|subscription)\s+([A-Za-z0-9_]+)").unwrap()
    })
}

fn re_js() -> &'static Regex {
    RE_JS.get_or_init(|| Regex::new(r#"['"](/[^'"]+\.js)['"]"#).unwrap())
}

fn re_enum() -> &'static Regex {
    RE_ENUM.get_or_init(|| Regex::new(r#""([A-Z][A-Z0-9_]{2,})""#).unwrap())
}

//
// ============================ EXTRACTION ============================
//

fn extract_ops(js: &str, source_url: &str) -> Vec<GqlOperation> {
    let mut ops = Vec::new();

    for cap in re_op().captures_iter(js) {
        let name = cap[2].to_string();
        let start = cap.get(0).unwrap().start();
        // Grab a chunk of text that likely contains the full query
        let slice = &js[start..js.len().min(start + 6000)];

        // Simple heuristic to find end of brace
        if let Some(end) = slice.rfind('}') {
            // Cleanup: ensure newlines are respected if they were escaped
            let clean_body = slice[..=end]
                .replace("\\n", "\n")
                .replace("\\r", "")
                .replace("\\t", "  ")
                .replace("\\\"", "\"");

            ops.push(GqlOperation {
                name,
                op_type: cap[1].to_string(),
                body: clean_body,
                source: source_url.to_string(),
            });
        }
    }
    ops
}

//
// ============================ ENUM INFERENCE ============================
//

fn infer_enums(ops: &[GqlOperation]) -> HashMap<String, HashSet<String>> {
    let mut buckets: HashMap<String, HashSet<String>> = HashMap::new();

    // Regex definitions for specific categories
    let re_vip = Regex::new(r"^(BRONZE|SILVER|GOLD|PLATINUM|DIAMOND|RUBY|SAPPHIRE|JADE|OPAL|DRAGON|MYTHIC)(_\d+)?$").unwrap();
    let re_state = Regex::new(r"^(ALABAMA|ALASKA|ARIZONA|ARKANSAS|CALIFORNIA|COLORADO|CONNECTICUT|DELAWARE|FLORIDA|GEORGIA|HAWAII|IDAHO|ILLINOIS|INDIANA|IOWA|KANSAS|KENTUCKY|LOUISIANA|MAINE|MARYLAND|MASSACHUSETTS|MICHIGAN|MINNESOTA|MISSISSIPPI|MISSOURI|MONTANA|NEBRASKA|NEVADA|NEW_HAMPSHIRE|NEW_JERSEY|NEW_MEXICO|NEW_YORK|NORTH_CAROLINA|NORTH_DAKOTA|OHIO|OKLAHOMA|OREGON|PENNSYLVANIA|RHODE_ISLAND|SOUTH_CAROLINA|SOUTH_DAKOTA|TENNESSEE|TEXAS|UTAH|VERMONT|VIRGINIA|WASHINGTON|WEST_VIRGINIA|WISCONSIN|WYOMING)$").unwrap();
    let re_risk = Regex::new(r"^(LOW|MEDIUM|HIGH)_RISK$").unwrap();
    let re_game_action = Regex::new(r"^(HIT|STAND|SPLIT|DOUBLE_DOWN|CASHOUT|BET|NEXT|START)$").unwrap();
    let re_kyc = Regex::new(r"^KYC_\d+$").unwrap();

    for op in ops {
        for cap in re_enum().captures_iter(&op.body) {
            let val = cap[1].to_string();
            
            let key = if re_vip.is_match(&val) {
                "VipLevel"
            } else if re_state.is_match(&val) {
                "UsState"
            } else if re_risk.is_match(&val) {
                "RiskLevel"
            } else if re_game_action.is_match(&val) {
                "GameAction"
            } else if re_kyc.is_match(&val) {
                "KycLevel"
            } else if val.contains("BTC") || val.contains("ETH") || val.contains("USD") || val.contains("SOL") || val.contains("LTC") {
                "CurrencyCode"
            } else if val.contains("WON") || val.contains("LOST") {
                "BetStatus"
            } else {
                "GenericEnum"
            };
            
            buckets.entry(key.into()).or_default().insert(val);
        }
    }

    buckets.retain(|_, v| v.len() >= 2);
    buckets
}

//
// ============================ RESPONSE SHAPE ============================
//

fn infer_objects(ops: &[GqlOperation]) -> Vec<ObjectType> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for op in ops {
        if let Ok(doc) = parse_query::<&str>(&op.body) {
            for def in doc.definitions {
                if let Definition::Operation(opdef) = def {
                    let selection_set = match opdef {
                        OperationDefinition::SelectionSet(s) => s,
                        OperationDefinition::Query(q) => q.selection_set,
                        OperationDefinition::Mutation(m) => m.selection_set,
                        OperationDefinition::Subscription(s) => s.selection_set,
                    };

                    for sel in selection_set.items {
                        collect_fields(&sel, &op.name, &mut map);
                    }
                }
            }
        }
    }

    map.into_iter()
        .map(|(n, f)| ObjectType { name: n, fields: f })
        .collect()
}

fn collect_fields<'a>(
    sel: &Selection<'a, &'a str>,
    root: &str,
    out: &mut HashMap<String, HashSet<String>>,
) {
    if let Selection::Field(f) = sel {
        let type_name = format!("{}Result", root);
        out.entry(type_name)
            .or_default()
            .insert(f.name.to_string());

        for s in &f.selection_set.items {
            collect_fields(s, root, out);
        }
    }
}

//
// ============================ INTERFACE INFERENCE ============================
//

fn infer_interfaces(objects: &[ObjectType]) -> Vec<InterfaceType> {
    let mut out = Vec::new();

    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let a = &objects[i];
            let b = &objects[j];
            let shared: HashSet<_> =
                a.fields.intersection(&b.fields).cloned().collect();

            if shared.len() * 10 >= a.fields.len().min(b.fields.len()) * 7 {
                out.push(InterfaceType {
                    name: "GameResult".into(),
                    fields: shared,
                });
            }
        }
    }
    out
}

//
// ============================ EMISSION ============================
//

// NEW: Generates the extracted_operations.graphql file
fn emit_raw_operations(ops: &[GqlOperation]) {
    let mut s = String::new();
    
    // Sort alphabetically by name to keep output deterministic
    let mut sorted_ops = ops.to_vec();
    sorted_ops.sort_by(|a, b| a.name.cmp(&b.name));

    s.push_str("# Auto-generated from scan\n");
    s.push_str(&format!("# Total Operations: {}\n\n", sorted_ops.len()));

    for op in sorted_ops {
        s.push_str(&format!("# Source: {}\n", op.source));
        s.push_str(&op.body);
        s.push_str("\n\n");
    }

    fs::write("extracted_operations.graphql", s).unwrap();
}

fn emit_schema(
    enums: &HashMap<String, HashSet<String>>,
    objects: &[ObjectType],
    interfaces: &[InterfaceType],
) {
    let mut s = String::new();

    s.push_str("schema { query: Query mutation: Mutation subscription: Subscription }\n\n");
    s.push_str("scalar Decimal\nscalar DateTime\nscalar UUID\nscalar JSON\n\n");

    for (name, vals) in enums {
        s.push_str(&format!("enum {} {{\n", name));
        for v in vals {
            s.push_str(&format!("  {}\n", v));
        }
        s.push_str("}\n\n");
    }

    for iface in interfaces {
        s.push_str(&format!("interface {} {{\n", iface.name));
        for f in &iface.fields {
            s.push_str(&format!("  {}: JSON!\n", f));
        }
        s.push_str("}\n\n");
    }

    for obj in objects {
        s.push_str(&format!("type {} implements GameResult {{\n", obj.name));
        for f in &obj.fields {
            s.push_str(&format!("  {}: JSON!\n", f));
        }
        s.push_str("}\n\n");
    }

    fs::write("schema.graphql", s).unwrap();
}

fn emit_sdk() {
    fs::write(
        "sdk.ts",
        r#"
export async function gql<T>(query: string, variables?: any): Promise<T> {
  const res = await fetch('/graphql', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ query, variables })
  });
  const json = await res.json();
  if (json.errors) throw json.errors[0];
  return json.data;
}
"#,
    )
    .unwrap();
}

//
// ============================ MAIN ============================
//

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start = Instant::now();

    let client = reqwest::Client::builder()
        .default_headers({
            let mut h = header::HeaderMap::new();
            h.insert(
                header::USER_AGENT,
                header::HeaderValue::from_static("Mozilla/5.0"),
            );
            h
        })
        .build()?;

    let html = client.get(&args.url).send().await?.text().await?;
    let base = Url::parse(&args.url)?;

    let mut ops = Vec::new();
    
    // Brute force JS extraction
    for cap in re_js().captures_iter(&html) {
        let js_url_str = &cap[1];
        let js_url = if js_url_str.starts_with("http") {
             Url::parse(js_url_str)?
        } else {
             base.join(js_url_str)?
        };
        
        if let Ok(resp) = client.get(js_url.clone()).send().await {
            if let Ok(txt) = resp.text().await {
                // Pass the source URL so we can write it in comments later
                ops.extend(extract_ops(&txt, js_url.as_str()));
            }
        }
    }

    let enums = infer_enums(&ops);
    let objects = infer_objects(&ops);
    let interfaces = infer_interfaces(&objects);

    emit_schema(&enums, &objects, &interfaces);
    emit_raw_operations(&ops); // Generate the raw operations file
    emit_sdk();

    println!("Completed in {:?}!", start.elapsed());
    println!("Generated: schema.graphql, sdk.ts, extracted_operations.graphql");
    Ok(())
}