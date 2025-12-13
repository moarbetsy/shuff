use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use graphql_parser::query::{parse_query, Definition, OperationDefinition, Selection};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::Regex;
use reqwest::{Client, Url};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Serialize;
use sourcemap::SourceMap;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

// ==============================================================================
// CONFIG & CONSTANTS
// ==============================================================================

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) Helios/2.1 SecurityScan";
const MIN_ENTROPY: f32 = 4.6;

static RE_NEXT_BUILD: OnceLock<Regex> = OnceLock::new();
static RE_ASSET_URL: OnceLock<Regex> = OnceLock::new();
static RE_SECRET: OnceLock<Regex> = OnceLock::new();
static RE_GQL_OP: OnceLock<Regex> = OnceLock::new();
static RE_ENUM_CAP: OnceLock<Regex> = OnceLock::new();
static RE_API_ENDPOINT: OnceLock<Regex> = OnceLock::new();

fn init_regexes() {
    RE_NEXT_BUILD.get_or_init(|| Regex::new(r#"/_next/static/([^/]+)/_buildManifest\.js"#).unwrap());
    RE_ASSET_URL.get_or_init(|| Regex::new(r#"(https?://[^"']+\.js|/[^"']+\.js)"#).unwrap());
    RE_SECRET.get_or_init(|| Regex::new(r#"(?i)(key|api|token|secret|auth|pass|pwd)[a-z0-9_.\-]*\s*[:=]\s*["']([a-zA-Z0-9_\-]{16,})["']"#).unwrap());
    RE_GQL_OP.get_or_init(|| Regex::new(r"(query|mutation|subscription)\s+([A-Za-z0-9_]+)").unwrap());
    RE_ENUM_CAP.get_or_init(|| Regex::new(r#""([A-Z][A-Z0-9_]{2,})""#).unwrap());
    RE_API_ENDPOINT.get_or_init(|| Regex::new(r#"["']((?:/api/|/v[0-9]/|https?://api\.)[^"'\s]+)["']"#).unwrap());
}

// ==============================================================================
// DATA MODELS
// ==============================================================================

#[derive(Parser, Debug)]
#[command(author, version, about = "Advanced GraphQL Reconstruction & Secret Scanner")]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value_t = 20)]
    concurrency: usize,

    /// Output directory for results
    #[arg(short, long, default_value = "helios_output", alias = "out-dir")]
    output: String,
}

#[derive(Debug, Clone, Serialize)]
struct GqlOperation {
    name: String,
    op_type: String,
    body: String,
    source_file: String,
}

#[derive(Debug, Clone, Serialize, Hash, Eq, PartialEq)]
enum FindingCategory {
    Secret,
    Endpoint,
}

#[derive(Debug, Clone, Serialize)]
struct Finding {
    category: FindingCategory,
    severity: String,
    match_val: String,
    context: String,
    source: String,
}

#[derive(Clone)]
struct ObjectType {
    name: String,
    fields: HashSet<String>,
}

struct AppState {
    client: ClientWithMiddleware,
    base_url: Url,
    visited: Arc<Mutex<HashSet<String>>>,
    findings: Arc<Mutex<Vec<Finding>>>,
    gql_ops: Arc<Mutex<Vec<GqlOperation>>>,
    #[allow(dead_code)] // Kept for future UI extensions
    progress: MultiProgress,
}

// ==============================================================================
// MAIN LOGIC
// ==============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    init_regexes();
    let args = Args::parse();
    let start_time = Instant::now();

    // 1. Setup Client
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(
        Client::builder()
            .user_agent(USER_AGENT)
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(10))
            .build()?
    )
    .with(RetryTransientMiddleware::new_with_policy(retry_policy))
    .build();

    let base_url = Url::parse(&args.url).context("Invalid URL")?;
    let mp = MultiProgress::new();

    let state = Arc::new(AppState {
        client,
        base_url: base_url.clone(),
        visited: Arc::new(Mutex::new(HashSet::new())),
        findings: Arc::new(Mutex::new(Vec::new())),
        gql_ops: Arc::new(Mutex::new(Vec::new())),
        progress: mp.clone(),
    });

    println!("{}", "\n    🔥 HELIOS RECON v2.1 🔥".bold().red());
    println!("{}", "    Hybrid GraphQL Reconstruction & Entropy Engine\n".dimmed());

    fs::create_dir_all(&args.output)?;

    // 2. Discovery Phase
    let spinner = mp.add(ProgressBar::new_spinner());
    spinner.set_message("Scanning for assets...");
    
    let target_str = args.url.clone();
    let root_html = state.client.get(&target_str).send().await?.text().await?;
    
    let mut script_urls = HashSet::new();
    
    // Basic regex scrape for .js files
    for cap in RE_ASSET_URL.get().unwrap().captures_iter(&root_html) {
        if let Ok(joined) = state.base_url.join(&cap[1]) {
            script_urls.insert(joined.to_string());
        }
    }

    // Next.js Intelligence
    if let Some(caps) = RE_NEXT_BUILD.get().unwrap().captures(&root_html) {
        let build_id = &caps[1];
        spinner.set_message(format!("Detected Next.js Build: {}", build_id));
        let manifest = state.base_url.join(&format!("/_next/static/{}/_buildManifest.js", build_id))?;
        if let Ok(res) = state.client.get(manifest).send().await {
            let txt = res.text().await.unwrap_or_default();
            let re_chunk = Regex::new(r#""(static/chunks/[^"]+\.js)""#).unwrap();
            for chunk in re_chunk.captures_iter(&txt) {
                if let Ok(u) = state.base_url.join(&format!("/_next/{}", &chunk[1])) {
                    script_urls.insert(u.to_string());
                }
            }
        }
    }
    spinner.finish_with_message(format!("Found {} JS bundles", script_urls.len()));

    // 3. Extraction Phase
    let bar = mp.add(ProgressBar::new(script_urls.len() as u64));
    bar.set_style(ProgressStyle::default_bar().template("{spinner:.green} {bar:40.cyan/blue} {pos}/{len} {msg}")?);

    let urls: Vec<String> = script_urls.into_iter().collect();
    
    stream::iter(urls)
        .map(|url| {
            let s = state.clone();
            let b = bar.clone();
            async move {
                process_bundle(&s, url).await;
                b.inc(1);
            }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;

    bar.finish();

    // 4. Analysis & Reconstruction Phase
    println!("\n{}", "⚙️  Reconstructing GraphQL Schema...".yellow());
    let ops_guard = state.gql_ops.lock().unwrap();
    let ops = ops_guard.clone();
    drop(ops_guard); // Release lock early

    let enums = infer_enums(&ops);
    let objects = infer_objects(&ops);
    let interfaces = infer_interfaces(&objects);
    let findings = state.findings.lock().unwrap();

    // 5. Artifact Generation
    generate_outputs(&args.output, &ops, &enums, &objects, &interfaces, &findings)?;

    println!("{}", format!("\n✅ Mission Complete in {:?}!", start_time.elapsed()).green().bold());
    println!("📂 Artifacts saved to: {}", args.output.cyan());
    println!("   └─ extracted_operations.graphql");
    println!("   └─ schema_reconstructed.graphql");
    println!("   └─ findings.json ({} items)", findings.len());

    Ok(())
}

// ==============================================================================
// EXTRACTION ENGINE
// ==============================================================================

async fn process_bundle(state: &Arc<AppState>, url: String) {
    // Dedup
    {
        let mut v = state.visited.lock().unwrap();
        if v.contains(&url) { return; }
        v.insert(url.clone());
    }

    // Try Source Map first
    let map_url = format!("{}.map", url);
    let mut used_sourcemap = false;

    if let Ok(res) = state.client.get(&map_url).send().await {
        if res.status().is_success() {
            if let Ok(bytes) = res.bytes().await {
                if let Ok(sm) = SourceMap::from_slice(&bytes) {
                    used_sourcemap = true;
                    for (id, content) in sm.sources().zip(sm.source_contents()) {
                        if let Some(code) = content {
                            if !id.contains("node_modules") {
                                analyze_text(code, id, state);
                            }
                        }
                    }
                }
            }
        }
    }

    // Fallback to raw JS
    if !used_sourcemap {
        if let Ok(res) = state.client.get(&url).send().await {
            if let Ok(text) = res.text().await {
                analyze_text(&text, &url, state);
            }
        }
    }
}

fn analyze_text(text: &str, source: &str, state: &Arc<AppState>) {
    let mut ops_buf = Vec::new();
    let mut findings_buf = Vec::new();

    // A. GraphQL Extraction
    for cap in RE_GQL_OP.get().unwrap().captures_iter(text) {
        let op_type = cap[1].to_string();
        let name = cap[2].to_string();
        
        // Context aware extraction: find the brace block
        let start = cap.get(0).unwrap().start();
        // Look ahead 5000 chars max
        let slice = &text[start..text.len().min(start + 5000)];
        
        // Basic brace counter to find end of query
        let mut depth = 0;
        let mut end_idx = 0;
        let mut found_brace = false;

        for (i, c) in slice.char_indices() {
            if c == '{' {
                depth += 1;
                found_brace = true;
            } else if c == '}' {
                depth -= 1;
            }
            if found_brace && depth == 0 {
                end_idx = i;
                break;
            }
        }

        if end_idx > 0 {
            let body = &slice[..=end_idx]
                .replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\t", " ");
            
            ops_buf.push(GqlOperation {
                name,
                op_type,
                body: body.to_string(),
                source_file: source.to_string(),
            });
        }
    }
    
    // B. Secret Extraction (High Entropy)
    for cap in RE_SECRET.get().unwrap().captures_iter(text) {
        let val = &cap[2];
        if shannon_entropy(val) > MIN_ENTROPY {
            findings_buf.push(Finding {
                category: FindingCategory::Secret,
                severity: "HIGH".to_string(),
                match_val: format!("{}: {}", &cap[1], val),
                context: format!("Found in {}", source),
                source: source.to_string(),
            });
        }
    }

    // C. Endpoint Extraction
    for cap in RE_API_ENDPOINT.get().unwrap().captures_iter(text) {
        findings_buf.push(Finding {
            category: FindingCategory::Endpoint,
            severity: "INFO".to_string(),
            match_val: cap[1].to_string(),
            context: format!("Endpoint in {}", source),
            source: source.to_string(),
        });
    }

    if !ops_buf.is_empty() {
        state.gql_ops.lock().unwrap().extend(ops_buf);
    }
    if !findings_buf.is_empty() {
        state.findings.lock().unwrap().extend(findings_buf);
    }
}

fn shannon_entropy(s: &str) -> f32 {
    let mut map = HashMap::new();
    for ch in s.chars() { *map.entry(ch).or_insert(0) += 1; }
    let len = s.len() as f32;
    map.values().fold(0.0, |acc, &count| {
        let p = count as f32 / len;
        acc - p * p.log2()
    })
}

// ==============================================================================
// SCHEMA INFERENCE
// ==============================================================================

fn infer_enums(ops: &[GqlOperation]) -> HashMap<String, HashSet<String>> {
    let mut buckets: HashMap<String, HashSet<String>> = HashMap::new();
    
    // Heuristics for common enums
    let patterns = vec![
        (r"^(BRONZE|SILVER|GOLD|PLATINUM|DIAMOND)$", "VipLevel"),
        (r"^(ALABAMA|CALIFORNIA|NEW_YORK|TEXAS)$", "Region"),
        (r"^(BTC|ETH|USD|EUR|GBP)$", "Currency"),
        (r"^(LOW|MEDIUM|HIGH)_RISK$", "Risk"),
        (r"^(ACTIVE|PENDING|BANNED)$", "UserStatus"),
    ];
    let compiled_patterns: Vec<(Regex, &str)> = patterns.iter()
        .map(|(r, n)| (Regex::new(r).unwrap(), *n))
        .collect();

    for op in ops {
        for cap in RE_ENUM_CAP.get().unwrap().captures_iter(&op.body) {
            let val = cap[1].to_string();
            // Filter noise (too short or numbers)
            if val.len() < 3 || val.chars().all(|c| c.is_numeric()) { continue; }

            let mut key = "UnknownEnum";
            for (re, name) in &compiled_patterns {
                if re.is_match(&val) { key = name; break; }
            }
            buckets.entry(key.into()).or_default().insert(val);
        }
    }
    // Filter out singleton enums (likely false positives)
    buckets.retain(|_, v| v.len() > 1);
    buckets
}

fn infer_objects(ops: &[GqlOperation]) -> Vec<ObjectType> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for op in ops {
        // Attempt strict parsing
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
                        recursive_field_walk(&sel, &op.name, &mut map);
                    }
                }
            }
        }
    }

    map.into_iter().map(|(n, f)| ObjectType { name: n, fields: f }).collect()
}

fn recursive_field_walk<'a>(sel: &Selection<'a, &'a str>, root: &str, out: &mut HashMap<String, HashSet<String>>) {
    if let Selection::Field(f) = sel {
        // We guess the type name based on the parent field
        let type_name = format!("{}Type", root); 
        out.entry(type_name).or_default().insert(f.name.to_string());
        
        // If this field has sub-selections, it's an object, recurse
        for s in &f.selection_set.items {
            recursive_field_walk(s, &f.name, out); // Use current field name as new root
        }
    }
}

fn infer_interfaces(objects: &[ObjectType]) -> Vec<ObjectType> {
    // Detect shared fields to create "interfaces"
    let mut ifaces = Vec::new();
    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            let a = &objects[i];
            let b = &objects[j];
            let shared: HashSet<_> = a.fields.intersection(&b.fields).cloned().collect();
            // If they share > 50% fields, assume an interface
            if shared.len() >= 3 && shared.len() > (a.fields.len().max(b.fields.len()) / 2) {
                 ifaces.push(ObjectType {
                     name: format!("I{}{}", a.name, b.name), // Crude naming
                     fields: shared,
                 });
            }
        }
    }
    ifaces
}

// ==============================================================================
// OUTPUT
// ==============================================================================

fn generate_outputs(
    dir: &str, 
    ops: &[GqlOperation], 
    enums: &HashMap<String, HashSet<String>>, 
    objs: &[ObjectType],
    ifaces: &[ObjectType],
    findings: &[Finding]
) -> Result<()> {
    
    // 1. Raw Operations
    let mut op_str = String::new();
    op_str.push_str("# Extracted GraphQL Operations\n\n");
    for op in ops {
        op_str.push_str(&format!("# Source: {}\n{}\n\n", op.source_file, op.body));
    }
    fs::write(format!("{}/extracted_operations.graphql", dir), op_str)?;

    // 2. Reconstructed Schema
    let mut schema = String::new();
    schema.push_str("schema { query: Query mutation: Mutation }\nscalar JSON\n\n");
    
    for (name, vals) in enums {
        schema.push_str(&format!("enum {} {{\n", name));
        for v in vals { schema.push_str(&format!("  {}\n", v)); }
        schema.push_str("}\n\n");
    }
    
    for obj in objs {
        schema.push_str(&format!("type {} {{\n", obj.name));
        for f in &obj.fields { schema.push_str(&format!("  {}: JSON\n", f)); }
        schema.push_str("}\n\n");
    }

    // Interfaces (appended as types for visibility)
    for iface in ifaces {
        schema.push_str(&format!("interface {}Common {{\n", iface.name));
        for f in &iface.fields { schema.push_str(&format!("  {}: JSON\n", f)); }
        schema.push_str("}\n\n");
    }
    fs::write(format!("{}/schema_reconstructed.graphql", dir), schema)?;

    // 3. Secrets Report
    let file = File::create(format!("{}/findings.json", dir))?;
    serde_json::to_writer_pretty(file, findings)?;

    Ok(())
}
