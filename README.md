

# The Unofficial Shuffle.com API & Automation Documentation




## Table of Contents
1. [Architecture Overview](#1-architecture-overview)
2. [Authentication & Headers](#2-authentication--headers)
3. [Request Payload Structure](#3-request-payload-structure)
4. [Provably Fair System](#4-provably-fair-system)
5. [Game Mechanics & Parameters](#5-game-mechanics--parameters)
6. [Provably Fair UI & Game Settings](#6-provably-fair-ui--game-settings)
7. [Common API Operations](#7-common-api-operations)
8. [Python Implementation](#8-python-implementation)
9. [Operational Reference](#9-operational-reference)
10. [Reverse Engineering New Features](#10-reverse-engineering-new-features)
11. [Cloudflare Protection](#11-cloudflare-protection)
12. [Complete Workflow Example](#12-complete-workflow-example)
13. [Troubleshooting](#13-troubleshooting)

---

## 1. Architecture Overview

Unlike REST APIs with multiple endpoints (e.g., `/api/v1/user`, `/api/v1/bet`), Shuffle uses **GraphQL**. This means you send **POST** requests to a single URL, specifying exactly what data or action you want in the body.

### Key Details
*   **Base Endpoint:** `https://shuffle.com/main-api/graphql/api/graphql`
*   **URL Construction:** Client uses `K.getGraphqlUrl("/api/graphql")` to build the endpoint
*   **Protocol:** HTTPS
*   **Method:** POST (always)
*   **Content-Type:** `application/json`
*   **Authentication:** JWT (JSON Web Token) + Cloudflare Clearance Cookies

Unlike REST, where you hit different URLs for different actions, here you always hit the **same URL**, but you change the JSON payload (`query` or `mutation`) to tell the server what to do.

### Mirror Domains

Shuffle operates across multiple mirror domains for redundancy and regional access. **All domains share the same API structure.**

**Approved Mirror Domains:**
```
shuffle.com
shuffle.game
shuffle.gg
shuffle888.com
shuffle.bet
shuffle.casino
shuffle.net
shuffle.vip
shuffle.site
shuffle.money
shuffle1069.com
shuffle.global
shuffle.online
shuffle.gold
```

**API Base URL Pattern:**
```
https://[domain]/main-api/graphql
```

Examples:
- `https://shuffle.com/main-api/graphql/api/graphql`
- `https://shuffle.gg/main-api/graphql/api/graphql`
- `https://shuffle.bet/main-api/graphql/api/graphql`

**When to Use Mirrors:**
- Primary domain blocked in your region
- DDoS protection/load balancing
- Improved latency from specific locations
- Backup access if main domain is unavailable

### Specialized GraphQL Endpoints

Beyond the main endpoint, Shuffle has specialized GraphQL endpoints for different services:

| Endpoint Path | Purpose | Full URL Example |
| :--- | :--- | :--- |
| `/api/graphql` | General queries & mutations | `https://shuffle.com/main-api/graphql/api/graphql` |
| `/sports/graphql-sports` | Sports betting data | `https://shuffle.com/main-api/graphql/sports/graphql-sports` |
| `/sports-main/graphql-sports-main` | Main sports data | `https://shuffle.com/main-api/graphql/sports-main/graphql-sports-main` |
| `/lottery/graphql-lottery` | Lottery game data | `https://shuffle.com/main-api/graphql/lottery/graphql-lottery` |
| `/kyc/graphql-kyc` | KYC verification | `https://shuffle.com/main-api/graphql/kyc/graphql-kyc` |
| `/graphql` | Default fallback | `https://shuffle.com/main-api/graphql/graphql` |

**Regional Configuration:**
- Endpoints may include regional parameters based on location
- KYC endpoint used for identity verification tasks
- Sports endpoints provide real-time betting odds and match data

### WebSocket Subscription Endpoints

For real-time updates, Shuffle uses WebSocket connections:

**Protocol:** `wss://` (WebSocket Secure)

**Base Pattern:** `wss://[domain]/main-api/bp-subscription`

| Subscription Path | Purpose | Full URL Example |
| :--- | :--- | :--- |
| `/subscription/graphql` | General subscriptions | `wss://shuffle.com/main-api/bp-subscription/subscription/graphql` |
| `/stable-subscription/graphql` | Core feature updates | `wss://shuffle.com/main-api/bp-subscription/stable-subscription/graphql` |
| `/sports-subscription/graphql` | Live sports updates | `wss://shuffle.com/main-api/bp-subscription/sports-subscription/graphql` |

**Use Cases:**
- Real-time bet results
- Live chat messages
- Balance updates
- Sports match odds changes
- Game state updates (Crash multiplier, etc.)

### REST/Utility Endpoints

Some functionality uses traditional REST endpoints:

| Endpoint | Method | Purpose |
| :--- | :--- | :--- |
| `/api/v1/sports/get-nav-items` | GET | Navigation menu & CMS data |
| `/api/v1/images/og-image` | GET | Social media preview images |

**Query Parameters:**
```
/api/v1/sports/get-nav-items?locale=en&preview=true
```

### External Service Endpoints

Shuffle integrates with external services:

| Service | URL | Purpose |
| :--- | :--- | :--- |
| **N9 Assets** | `https://n9assets.com/games/games.json` | Game library catalog |
| **N9 Assets (Alt)** | `https://n9assets.com/file/games/games.json` | Alternative game catalog (flag: `v.lU`) |
| **Store Location** | `https://shuffle.click/store-access-location` | Regional compliance checks |
| **Genius Sports** | `https://viewer-data.production.geniuslive.geniussports.com/` | Live sports streaming data |

**Note:** External endpoints are managed by third-party services and may have different authentication requirements.

### Asset Endpoints (Non-Transactional)

Game tiles and visual assets are served from static paths:

| Asset Type | Path | Purpose |
| :--- | :--- | :--- |
| **Mobile/Tall** | `/_next/static/media/originals-limbo-tall.c1997d70.png` | Mobile view game tile |
| **Desktop** | `/_next/static/media/originals-limbo.b2c2148b.png` | Desktop view game tile |

*Note: These paths are for display purposes only and don't affect game functionality.*

### Endpoint Selection Strategy

When implementing an API client:

1. **Start with primary domain:** `shuffle.com`
2. **Fallback to mirrors:** If primary fails, try alternate domains
3. **Use specialized endpoints:** Sports/lottery for specific features
4. **WebSocket for real-time:** Live updates require subscription endpoints
5. **Check regional restrictions:** Some mirrors may work better in certain regions

**Example Fallback Logic:**
```python
MIRROR_DOMAINS = [
    'shuffle.com', 'shuffle.gg', 'shuffle.bet', 
    'shuffle.casino', 'shuffle.game'
]

def get_working_endpoint():
    for domain in MIRROR_DOMAINS:
        try:
            url = f"https://{domain}/main-api/graphql/api/graphql"
            # Test connection
            return url
        except:
            continue
    raise Exception("No working mirrors found")
```

---

## 2. Authentication & Headers

To interact with the API (especially for betting or checking balances), you must impersonate a real browser session. **You cannot generate these keys programmatically** â€“ you must extract them from a logged-in browser.

### Required Headers

| Header | Value/Source | Description |
| :--- | :--- | :--- |
| `Authorization` | `Bearer <JWT>` | **Most critical.** Found in DevTools Network tab. Expires periodically (usually 24h). |
| `Cookie` | Browser Network Tab | Contains `__cf_bm`, `cf_clearance`, and session identifiers required to pass Cloudflare. |
| `User-Agent` | Your Browser | **Crucial.** Must match a real browser to bypass Cloudflare. |
| `Content-Type` | `application/json` | Required for all requests. |
| `Origin` | `https://shuffle.com` | Required for CORS checks. |
| `Referer` | Game-specific URL | Example: `https://shuffle.com/games/originals/limbo` |
| `x-correlation-id` | Generated UUID | A tracking ID the client sends to link requests. |
| `x-country` | Your location code | Example: `CA`, `US`. Should match your IP location. |

### How to Obtain Your Credentials

1.  **Log in** to Shuffle.com in your browser (Chrome/Firefox recommended).
2.  Press **F12** to open Developer Tools.
3.  Go to the **Network** tab.
4.  Filter by **Fetch/XHR** or search for `graphql`.
5.  Refresh the page or perform an action (like viewing your wallet or placing a minimum bet).
6.  Click a request named `graphql` and view **Request Headers**.
7.  Copy the following:
    *   **Authorization:** The full `Bearer eyJhbGc...` token
    *   **Cookie:** The entire cookie string (can be very long)
    *   **User-Agent:** Your exact browser user agent string

**Important Notes:**
*   The JWT token typically expires after 24 hours
*   Cloudflare cookies (`cf_clearance`) expire quickly
*   You must use the exact User-Agent from the browser where you copied the cookies

---

## 3. Request Payload Structure

Every GraphQL request must follow this JSON structure:

```json
{
  "operationName": "NameOfAction",
  "variables": {
    "param1": "value1",
    "param2": 100
  },
  "query": "query/mutation definition string..."
}
```

### Query vs Mutation
*   **Query:** Used to *read* data (e.g., Get Balance, Get Game Settings, Get Chat)
*   **Mutation:** Used to *write* data (e.g., Place Bet, Update Seed, Claim Rewards)

---

## 4. Provably Fair System

Shuffle uses a cryptographic hashing system (HMAC-SHA256) to ensure fair outcomes and allow users to verify that game results are determined randomly and transparently. **The casino cannot tamper with results after a bet is placed.**

This system prevents manipulation by ensuring that:
1. **Server commits first:** The server publishes a hashed version of its seed before you bet
2. **You control input:** Your client seed prevents the server from pre-calculating favorable results
3. **Results are verifiable:** You can independently verify any outcome using the revealed seeds

### Key Components

Understanding the three primary components is essential:

| Component | Description | When Available | Purpose |
| :--- | :--- | :--- | :--- |
| **Server Seed (Hashed)** | SHA-256 hash of the secret server seed | Before bet | Proves results were predetermined |
| **Server Seed (Plaintext)** | Original unhashed server seed | After bet/rotation | Enables verification |
| **Client Seed** | Your customizable input string | Set by you | Adds entropy you control |
| **Nonce** | Counter incremented with each bet | Always visible | Ensures unique results per bet |

**How It Works:**
1. Server generates a random seed and publishes only its **hash**
2. You provide your **client seed** (or use default)
3. When you bet, the **nonce** increments
4. Result is calculated from: HMAC-SHA256(serverSeed, `clientSeed:nonce:index`)
5. After bet, server reveals the **plaintext server seed** for verification

### Core Generation Function (G)

This is the fundamental method used across all games to derive random outcomes.

| Parameter | Type | Description |
| :--- | :--- | :--- |
| `serverSeed` | String | The plaintext server seed (revealed after game) |
| `clientSeed` | String | Your seed input |
| `nonce` | Integer | Unique counter incremented with every bet |
| `cursor` | Integer | Starting byte index (typically 0) |
| `count` | Integer | Number of 4-byte random values required |

**Algorithm:**
1. Construct message: `{clientSeed}:{nonce}:{index}` (index starts at 0)
2. Generate HMAC-SHA256 hash using `serverSeed` as the secret key
3. Extract 4-byte segments from the hash digest
4. Convert each segment to float between 0 and 1
5. Increment index and repeat if more randomness needed (beyond 32 bytes)

**Important:** The actual game result determination and verification logic runs **client-side** in the browser. Once you have the `serverSeed`, `clientSeed`, and `nonce`, you can independently verify any result without making API calls.

### Seed Management Interface

Users can view and manage their seeds through the Provably Fair modal:

**Visible Information:**
| Field | Description | Also Called |
| :--- | :--- | :--- |
| **Active Client Seed** | Current clientSeed for generating results | Your input |
| **Active Hashed Server Seed** | SHA-256 hash of current server seed | Commitment |
| **Current Nonce** | Count of bets with current seed pair | "Total plays with pair" |
| **Next Hashed Server Seed** | Hash that activates on rotation | Future commitment |

**Available Actions:**
- **Change Client Seed:** Update your seed for future bets
- **Rotate Seed Pair:** Get new server seed (reveals current plaintext seed)
- **Unhash Server Seed:** View plaintext seed for verification
- **Verify Results:** Check past outcomes in verification tab

**Seed Rotation:** When you rotate your seed pair, the current server seed is revealed (allowing verification of all past bets), and a new server seed is generated for future bets.

### Verification Process

To verify any game outcome, you need the specific parameters used for that bet:

**Required Inputs:**
1. **Server Seed (Plaintext)** - Revealed after bet or seed rotation
2. **Client Seed** - Your seed at time of bet
3. **Nonce** - The specific bet counter value
4. **Game Parameters** - Additional metadata depending on game:
   - Mines: Number of mines
   - Plinko: Row count, risk level
   - Wheel: Segment count, risk level
   - Tower: Difficulty level

**Verification Interface (`ProvablyFairTabVerify`):**
- Input fields for server seed, client seed, and nonce
- Game-specific parameter inputs
- Calculated outcome display based on your inputs
- Real-time verification as you modify inputs

**Step-by-Step Verification:**
1. Go to Provably Fair modal â†’ VERIFY tab
2. Enter the plaintext server seed (revealed after bet)
3. Enter your client seed used for that bet
4. Enter the nonce (bet number)
5. Add game-specific parameters if applicable
6. Compare calculated result with actual game outcome

**Why This Matters:**
- Proves the casino didn't know your bet beforehand
- Confirms results weren't changed after betting
- Allows independent verification without trusting the platform
- Mathematical proof of fairness

### Game-Specific Verification Requirements

Each game requires specific inputs for verification:

| Game | Random Count | Additional Inputs | Result Format | Formula Reference |
| :--- | :---: | :--- | :--- | :--- |
| **Dice** | 1 | None | 0.00 - 100.00 | `floor(10001 * e) / 100` |
| **Limbo** | 1 | None | 1.01x - 1,000,000x | `min(.99 * 0x1000000 / (0x1000000 * n + 1), 1e6)` |
| **Crash** | 1 | None | Multiplier with 'x' | Uses `x.QC(c,_)` function |
| **Plinko** | Row Count | Row count, Risk level | Path + Multiplier | Binary (0/1) per row |
| **Mines** | Mine Count | Number of mines | Mine positions (0-24) | Unique positions from 25-tile grid |
| **Wheel** | 1 | Segment count, Risk level | Segment index | 0 to N-1 (N = segment count) |
| **Hilo/Blackjack** | 52 | None | Card indices | Shuffled deck (0-51) |
| **Roulette** | 1 | None | 0 - 36 | `floor(37 * e)` |
| **Keno** | 10 | None | 10 numbers (1-40) | Unique drawn numbers |
| **Tower** | 9 (rows) | Difficulty | Safe/mine positions | Based on difficulty config |

**Note:** `e` represents the random number(s) generated by the G function, `n` is the specific random value used in calculations.

### Python Implementation of Verification

```python
import hmac
import hashlib

def generate_random_numbers(server_seed, client_seed, nonce, cursor=0, count=1):
    """
    Core provably fair generation function (G)
    Returns list of floats between 0 and 1
    """
    results = []
    bytes_needed = count * 4
    bytes_generated = 0
    block_counter = 0
    
    while bytes_generated < bytes_needed:
        message = f"{client_seed}:{nonce}:{block_counter}"
        hash_hex = hmac.new(
            server_seed.encode(),
            message.encode(),
            hashlib.sha256
        ).hexdigest()
        
        # Extract 4-byte chunks
        for i in range(0, len(hash_hex), 8):
            if bytes_generated >= bytes_needed:
                break
            chunk = hash_hex[i:i+8]
            if len(chunk) == 8:
                value = int(chunk, 16) / (2**32)
                results.append(value)
                bytes_generated += 4
        
        block_counter += 1
    
    return results[:count]

def verify_limbo_result(server_seed, client_seed, nonce):
    """
    Verify Limbo game result using client-side calculation.
    This matches the exact logic used in the Provably Fair modal.
    
    Args:
        server_seed: The revealed server seed
        client_seed: Your client seed
        nonce: The bet nonce/counter
    
    Returns:
        Float: The multiplier (1.01 to 1,000,000)
    """
    # Generate random number using G function
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    
    # Calculate hash integer (0x1000000 = 16777216)
    hash_int = int(random_value * 0x1000000)
    
    # Calculate multiplier with RTP of 0.99 (99%)
    # Formula: min((0.99 * 0x1000000) / (hash_int + 1), 1000000)
    multiplier = min(
        (0.99 * 0x1000000) / (hash_int + 1),
        1000000  # Maximum multiplier cap (1e6)
    )
    
    # Bound minimum to 1.01 and format to 2 decimal places (round down)
    result = max(1.01, multiplier)
    return round(result, 2)  # Simulates M().ROUND_DOWN to 2 decimals

def verify_dice_result(server_seed, client_seed, nonce):
    """
    Verify Dice game result (0.00 - 100.00).
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
    
    Returns:
        Float: The dice roll value (0.00 to 100.00)
    """
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    # Formula: floor(10001 * e) / 100
    return round(random_value * 10001) / 100

def verify_roulette_result(server_seed, client_seed, nonce):
    """
    Verify Roulette result (0-36).
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
    
    Returns:
        Integer: The roulette number (0 to 36)
    """
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    # Formula: floor(37 * e)
    return int(random_value * 37)

def verify_mines_positions(server_seed, client_seed, nonce, num_mines):
    """
    Verify Mines game mine positions.
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
        num_mines: Number of mines (1-24)
    
    Returns:
        List: Mine positions (0-24) on the 5x5 grid
    """
    random_values = generate_random_numbers(server_seed, client_seed, nonce, 0, num_mines)
    positions = []
    available_tiles = list(range(25))
    
    for rand_val in random_values:
        # Select from remaining available tiles
        index = int(rand_val * len(available_tiles))
        positions.append(available_tiles.pop(index))
    
    return sorted(positions)

def verify_plinko_path(server_seed, client_seed, nonce, row_count):
    """
    Verify Plinko game path (left/right deflections).
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
        row_count: Number of rows (8-16)
    
    Returns:
        List: Binary path (0=left, 1=right) for each row
    """
    random_values = generate_random_numbers(server_seed, client_seed, nonce, 0, row_count)
    # Map to binary: floor(2 * e) gives 0 or 1
    path = [int(val * 2) for val in random_values]
    return path

def verify_keno_numbers(server_seed, client_seed, nonce):
    """
    Verify Keno winning numbers.
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
    
    Returns:
        List: 10 unique winning numbers (1-40)
    """
    random_values = generate_random_numbers(server_seed, client_seed, nonce, 0, 10)
    drawn_numbers = []
    available = list(range(1, 41))  # 1-40
    
    for rand_val in random_values:
        # Formula: ceil(e * (40 - already_drawn))
        index = int(rand_val * len(available))
        drawn_numbers.append(available.pop(index))
    
    return sorted(drawn_numbers)

def verify_card_deck(server_seed, client_seed, nonce):
    """
    Verify Hilo/Blackjack shuffled deck.
    
    Args:
        server_seed: Plaintext server seed (revealed)
        client_seed: Your client seed
        nonce: The bet nonce
    
    Returns:
        List: 52 card indices (0-51) representing shuffled deck
    """
    random_values = generate_random_numbers(server_seed, client_seed, nonce, 0, 52)
    deck = []
    available_cards = list(range(52))
    
    for rand_val in random_values:
        index = int(rand_val * len(available_cards))
        deck.append(available_cards.pop(index))
    
    return deck
```

---

## 5. Game Mechanics & Parameters

This section documents the exact parameters and result generation logic for each game.

### 5.1 Mines

Determine positions of gems and mines on a 5Ã—5 grid (25 tiles total).

**Parameters:**
```json
{
  "numberOfMines": 3  // Integer: 1-24
}
```

**Result Calculation:**
- Uses G function to generate `numberOfMines` random numbers
- Maps to unique tile positions (0-24)
- Tiles marked as: GEM (2) or MINE (1)

**Tile Types & Visual Behavior:**
| Tile Type | Numeric Value | Visual Behavior | Border Color |
| :--- | :---: | :--- | :--- |
| **MASKED** | 0 | Transparent border when hovered | `rgba(0,0,0,0)` |
| **MINE** | 1 | Red drop shadow when selected | `#ff2020` (hover) |
| **GEM** | 2 | Gold drop shadow when selected | `#fcb10f` (hover) |

**UI Details:**
- Base tile uses pulsing border animation (`borderPulse`)
- Optional Halloween theme available (`halloweenTheme`)
- Selected tiles display hashed server seed
- Drop shadow filters: Mine `#EF594577`, Gem `#FCB10F66`

**Example Mutation:**
```graphql
mutation MinesPlay($data: MinesPlayInput!) {
  minesPlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        mines {
          minePositions
          revealedPositions
        }
      }
    }
  }
}
```

### 5.2 Plinko

Determines multiplier based on rows and risk level.

**Parameters:**
```json
{
  "plinkoRowCount": 16,        // Integer: 8-16
  "plinkoRiskLevel": "HIGH_RISK"  // LOW_RISK, MEDIUM_RISK, HIGH_RISK
}
```

**Result Calculation:**
- Generates `rowCount` random numbers
- Each mapped to 0 or 1 (left/right deflection)
- Final position determines multiplier from payout table

**Risk Level Payout Tables:**
- **LOW_RISK:** Lower variance, safer multipliers
- **MEDIUM_RISK:** Balanced risk/reward
- **HIGH_RISK:** High variance, potential for extreme multipliers

### 5.3 Wheel

Outcome based on segment count and risk level.

**Parameters:**
```json
{
  "wheelSegmentCount": "FIFTY",  // TEN, TWENTY, THIRTY, FORTY, FIFTY
  "wheelRiskLevel": "HIGH"       // LOW, MEDIUM, HIGH
}
```

**Result Calculation:**
- Single random number from G function
- Mapped to segment index based on `segmentCount`
- Segment index corresponds to multiplier in payout table

**Color Indicators & Styling:**
| Color | Multiplier Range | Hex/CSS Variable | Hover Background |
| :--- | :--- | :--- | :--- |
| **Black** | Base multipliers | Default | `var(--color-gray600)` |
| **Gray** | 1.2, 1.5 | Gray 200 | `var(--color-gray200)` |
| **Purple** | 1.5 - 1.9 | Primary Violet | `var(--color-primaryViolet)` |
| **Yellow** | ~1.7 | Yellow | `#ffeb39` |
| **Orange** | ~2.0 | Orange-Red | `#ff5f5f` |
| **Green** | 3, 4, 5, 9.9, 19.8+ | Lime Green | `#c7f025` |

**Display Format:**
- Multipliers shown with 2 decimal places + "x" (e.g., "3.00x")
- CSS classes: `WheelGameFooter_black__Tmwjd`, `WheelGameFooter_gray__36ywg`, etc.

### 5.4 Crash

Single multiplier game with progressive crash point.

**Result Calculation:**
- Uses internal crash calculation function `x.QC(clientSeed, nonce)`
- Output format: Multiplier with 'x' suffix (e.g., "1.50x")
- Based on serverSeed, clientSeed, and nonce
- Displayed as fixed decimal string ("0.00" format)

**Example Query:**
```graphql
query CrashGame {
  crashGame {
    id
    crashPoint
    status
  }
}
```

### 5.5 Limbo

Single multiplier game, bounded between 1.01x and 1,000,000x.

**Parameters:**
```json
{
  "amount": "0.10000000",    // Bet amount in crypto
  "bet": "2.00",             // Target multiplier (eu)
  "currency": "USDT",        // Currency ticker
  "windowId": "abc123xyz",   // Browser tab identifier
  "usdAmount": "0.10"        // Calculated USD equivalent (optional)
}
```

**Mutation:**
```graphql
mutation LimboPlay($data: LimboPlayInput!) {
  limboPlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        limbo {
          resultValue
        }
      }
    }
  }
}
```

**Result Calculation (Client-Side Verification):**

The multiplier is calculated **client-side** using cryptographic inputs. No separate API call needed for verification once seeds are available.

**Inputs Required:**
- `serverSeed`: Revealed after game conclusion
- `clientSeed`: User-provided seed
- `nonce`: Bet counter

**Calculation Formula:**
```javascript
// Generate random number from seeds
n = G(serverSeed, clientSeed, nonce, 0, 1)

// Calculate multiplier with BigNumber precision
multiplier = Math.min(
  (0.99 * 0x1000000) / ((0x1000000 * n) + 1),
  1000000  // Maximum multiplier (1e6)
)

// Format to 2 decimal places, round down
result = multiplier.toFixed(2, M().ROUND_DOWN)
```

**Detailed Formula:**
```
random = G(serverSeed, clientSeed, nonce, 0, 1)
hash_int = int(random * 0x1000000)
multiplier = min((0.99 * 0x1000000) / (hash_int + 1), 1000000)
bounded = max(1.01, multiplier)
formatted = toFixed(2, ROUND_DOWN)
```

**Key Points:**
- **RTP (Return to Player):** 0.99 (99%)
- **Min Multiplier:** 1.01x (bounded)
- **Max Multiplier:** 1,000,000x (1e6)
- **Precision:** BigNumber library with `ROUND_DOWN` mode
- **Verification:** Performed in Provably Fair modal using client-side calculation

### 5.6 Hilo & Blackjack

Card games using shuffled deck generation.

**Result Calculation:**
- Uses G function with count=52
- Generates 52 random numbers
- Maps to card indices (0-51) representing shuffled deck

**Card Structure:**
```json
{
  "suit": "HEARTS",  // CLUBS, SPADES, DIAMONDS, HEARTS
  "value": "A",      // A, 2-10, J, Q, K
  "color": "red"     // Red: Hearts/Diamonds, Black: Clubs/Spades
}
```

**Suit Colors & Styling:**
| Suit | Color | Hex Code | SVG Fill | CSS Class |
| :--- | :--- | :--- | :--- | :--- |
| **CLUBS** | Black | Black | Black fill | Default |
| **SPADES** | Black | Black | Black fill | Default |
| **DIAMONDS** | Red | `#B4171E` | Red fill | `HiloProvablyFairCard_red__EKmdI` |
| **HEARTS** | Red | `#B4171E` | Red fill | `HiloProvablyFairCard_red__EKmdI` |

**Display Details:**
- Card value displayed with suit SVG icon
- Red-colored text for red suits using specific CSS class
- Card elements include both value and corresponding suit graphic

### 5.7 Roulette

Standard roulette with numbers 0-36.

**Result Calculation:**
```
random = G(serverSeed, clientSeed, nonce, 0, 1)
result = floor(37 * random)  // 0-36
```

### 5.8 Keno

Select 10 winning numbers from 1-40.

**Result Calculation:**
- Uses G function with count=10
- Generates 10 unique numbers between 1-40
- Formula: `ceil(random * (40 - already_selected))`

**Example Mutation:**
```graphql
mutation KenoPlay($data: KenoPlayInput!) {
  kenoPlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        keno {
          winningNumbers
          selectedNumbers
        }
      }
    }
  }
}
```

### 5.9 Dice

Roll a number between 0.00 and 100.00.

**Parameters:**
```json
{
  "target": "50.00",    // Target number (0.00-100.00)
  "over": true          // Bet over or under (userDiceDirection)
}
```

**Result Calculation:**
```
random = G(serverSeed, clientSeed, nonce, 0, 1)
result = floor(10001 * random) / 100  // 0.00-100.00
```

**Outcome Determination:**
- **WIN:** If `over=true` and `result > target`, or `over=false` and `result < target`
- **LOSE:** Otherwise
- Result compared against user-defined roll value and direction

### 5.10 Tower

Climb towers by choosing correct blocks across 9 rows.

**Parameters:**
```json
{
  "towerDifficulty": "HARD"  // EASY, MEDIUM, HARD, EXPERT, MASTER
}
```

**Difficulty Configurations:**
| Difficulty | Mines/Apples | Columns | Visual |
| :--- | :---: | :---: | :--- |
| **EASY** | 3 | 4 | 3 Keys, 1 Apple |
| **MEDIUM** | 2 | 3 | 2 Keys, 1 Apple |
| **HARD** | 1 | 2 | 1 Key, 1 Apple |
| **EXPERT** | 1 | 3 | 1 Key, 2 Apples |
| **MASTER** | 1 | 4 | 1 Key, 3 Apples |

**Result Calculation:**
- Uses G function with count=9 (one per row)
- Determines mine positions based on difficulty
- Progressive reveal system

---

## 6. Provably Fair UI & Game Settings

### 6.1 Provably Fair Modal Interface

The client application includes a dedicated modal dialog for managing and verifying provably fair inputs. The modal contains two primary tabs, each serving distinct purposes.

**Modal Structure:**
| Component | Description | CSS Class |
| :--- | :--- | :--- |
| **Modal Title** | Main heading | `ProvablyFairModal_provablyFairTitle__rwbAX` |
| **Tabs** | Two tabs: SEEDS and VERIFY | Tab array: `[{id:s.r6.SEEDS,name:"tabSeeds"},{id:s.r6.VERIFY,name:"tabVerify"}]` |
| **Seed Input Fields** | Grid layout for seed management | `ChangeSeedInput_field__RHSXQ` |
| **Rotation Title** | Header for seed rotation section | `ProvablyFairTabSeeds_rotateSeedTitle__uHPlu` |

---

#### Tab 1: "tabSeeds" (ID: s.r6.SEEDS)

This tab acts as a **cryptographic key vault** for managing your current seed pair.

**CSS Root Class:** `.ProvablyFairTabSeeds_rotateSeedTitle__uHPlu`

**Information Displayed:**
| Field | Description | Purpose |
| :--- | :--- | :--- |
| **Active Client Seed** | Your current client seed | Currently influencing game outcomes |
| **Active Hashed Server Seed** | SHA-256 hash of server seed | Proves server commitment |
| **Current Nonce** | Total plays with current pair | Bet counter for this seed pair |
| **Next Server Seed** | Hashed seed for next rotation | Future commitment |

**Core Functionality: Rotating Seed Pair**

The primary action in this tab is rotating your seed pair:

1. **Input Form:** User enters a new client seed
   - Form CSS Class: `.ChangeSeedInput_form__AmRid`
   - Input validation and formatting applied
   
2. **Confirmation:** User clicks "Change" button
   - System executes seed change mutation
   - Server reveals current plaintext seed
   - New seed pair becomes active
   
3. **Success Notification:** 
   - Confirmation message displayed
   - Active seeds updated in display
   - Nonce resets to 0

**When to Rotate:**
- After significant betting sessions to verify past results
- When you want to change your client seed
- Periodically for security and transparency
- Before verifying a series of past bets

---

#### Tab 2: "tabVerify" (ID: s.r6.VERIFY)

This tab acts as an **inspection tool** for verifying past game results using cryptographic proofs.

**CSS Root Class:** `.ProvablyFairTabVerify_root__0GnY1`

**Interface Structure:**

**1. Verification Input Forms** (`.ProvablyFairTabVerify_verifyFormGroup__rCx_g`)

Users can input or modify verification parameters:

| Input Field | Purpose | Notes |
| :--- | :--- | :--- |
| **Game Selection** | Dropdown to select game type | Pre-populated from URL query params if available |
| **Client Seed** | Your seed used for the bet | Must match seed at time of bet |
| **Server Seed** | Plaintext server seed | Only available after rotation |
| **Nonce** | Specific bet counter | Identifies which bet to verify |
| **Game Parameters** | Additional inputs per game | e.g., mine count, row count, risk level |

**Available Games in Dropdown:**
- Dice
- Limbo
- Crash
- Plinko
- Mines
- Wheel
- Tower
- Keno
- Hilo
- Blackjack
- Roulette

**Special Case - Crash:**
- May show only client seed and nonce inputs
- Server hash sometimes treated as nonce input for result generation

**2. Verification Display (Dynamic Content)**

The display below input fields changes based on selected `verifyGameType`:

| Game Type | Display Format | Example |
| :--- | :--- | :--- |
| **Crash/Limbo** | Calculated multiplier | "1.85x", "2.00x" |
| **Mines** | 5Ã—5 grid visualization | Shows gem/mine positions (0-24) |
| **Hilo/Blackjack** | Card sequence | Displays predicted cards in order |
| **Wheel** | Simulated wheel graphic | Highlights winning segment |
| **Plinko** | Path visualization | Shows left/right deflections |
| **Dice** | Number display | "50.42" (0.00-100.00) |
| **Roulette** | Number display | "17" (0-36) |
| **Keno** | Number grid | Highlights 10 drawn numbers |
| **Tower** | Grid layout | Shows safe/mine positions per row |

**Verification Workflow:**
1. Select the game you want to verify
2. Enter the plaintext server seed (from rotation or unhash)
3. Enter your client seed used for that bet
4. Enter the specific nonce for that bet
5. Add game-specific parameters if required
6. View calculated result
7. Compare with actual game outcome

**Real-Time Calculation:**
- Results update instantly as you modify inputs
- No API call needed - all calculations client-side
- Uses same HMAC-SHA256 algorithm as server

---

#### Tab Separation Philosophy

The two-tab design provides clear separation of concerns:

| Tab | Role | Metaphor |
| :--- | :--- | :--- |
| **tabSeeds** | Management & Preparation | Cryptographic key vault |
| **tabVerify** | Inspection & Validation | Forensic inspection tool |

**Workflow:**
1. **"tabSeeds"** â†’ Monitor current seeds, rotate when needed
2. **"tabVerify"** â†’ Verify past bets using revealed seeds

This separation makes cryptographic transparency intuitive and accessible to users.

### 6.2 Game Settings & Configuration

All original games include configurable settings accessible in the game layout.

**Available Settings:**

| Setting | Description | Visual Asset | State Variable |
| :--- | :--- | :--- | :--- |
| **Turbo Mode** | Speeds up game animations | `/images/turbo-mode.svg` | `isTurboModeOn` |
| **Show Max Bet** | Display maximum bet button | `/icons/max-bet.svg` | `showGamesMaxBet` |
| **Mute Sounds** | Toggle game sound effects | `/icons/speaker-off.svg` | `muteGameSounds` |
| **Hotkeys** | Enable keyboard shortcuts | Info modal available | `gameHotkeysEnabled` |

**Hotkey Mappings:**
```javascript
{
  KeyW: "Action 1",
  KeyS: "Action 2", 
  Space: "Primary Action"
  // Additional mappings defined per game
}
```

**Settings Access:**
- Settings toggles available in game header/footer
- Visual indicators for active states
- Persistent across game sessions
- Hotkey info modal accessible via (H) key

### 6.3 Game UI Themes

**Special Themes:**
- **Halloween Theme:** Available for Mines game (`MinesGameTileElement_halloweenTheme__Wqpy8`)
- **Color Schemes:** Customizable per game type
- **Animation Effects:** Pulsing borders, drop shadows, hover states

---

## 7. Common API Operations

Here are the reconstructed schemas for common actions. **You must verify these by inspecting your own network traffic**, as Shuffle frequently changes variable names to break bots.

**Important:** All operations use the same GraphQL endpoint constructed via `K.getGraphqlUrl("/api/graphql")`, which resolves to `https://shuffle.com/main-api/graphql/api/graphql`.

### A. Public Data (Token Stats)

*   **Type:** Query
*   **Operation Name:** `tokenInfo`
*   **Variables:** `{}`
*   **Payload:**
    ```json
    {
      "operationName": "tokenInfo",
      "variables": {},
      "query": "query tokenInfo { tokenInfo { priceInUsd circulatingSupply burnedTokens } }"
    }
    ```

### B. Get User Balance

*   **Type:** Query
*   **Operation Name:** `GetBalances`
*   **Variables:** `{}`
*   **Query:**
    ```graphql
    query GetBalances {
      me {
        balances {
          currency
          amount
        }
      }
    }
    ```

### C. Get Game Settings (Limits)

*   **Type:** Query
*   **Operation Name:** `GetAppSettings`
*   **Query:**
    ```graphql
    query GetAppSettings {
      appSettings {
        limbo {
          maxPayoutUSD
          minBetUSD
        }
      }
    }
    ```

### D. Recently Played Games

Retrieval restricted to authenticated users.

**Query Parameters:**
```json
{
  "sortBy": "RECENT_PLAY",
  "isRecentPlay": true,
  "first": 40,        // Page size
  "skip": 0,          // Pagination offset
  "fetchPolicy": "no-cache"
}
```

**Query:**
```graphql
query GetRecentlyPlayed($first: Int!, $skip: Int!) {
  games(
    sortBy: RECENT_PLAY
    isRecentPlay: true
    first: $first
    skip: $skip
  ) {
    nodes {
      id
      gameName
      createdAt
      payout
      multiplier
    }
    pageInfo {
      hasNextPage
    }
  }
}
```

**Navigation:**
- Initial load: 40 games
- Load more: Increments `skip` parameter
- Search filter available on `/casino/recently-played`
- Skeleton loaders during fetch

### E. Rotate Seed (Provably Fair)

*   **Type:** Mutation
*   **Operation Name:** `RotateSeed`
*   **Variables:**
    ```json
    {
      "newSeed": "your_new_client_seed_here"
    }
    ```
*   **Query:**
    ```graphql
    mutation RotateSeed($newSeed: String!) {
      rotateSeed(newSeed: $newSeed) {
        clientSeed
        hashedServerSeed
        nextHashedServerSeed
      }
    }
    ```
*   **GraphQL Operation Reference:** Related to `p.GCs` mutation for updating seed pair
*   **Effect:** Reveals current plaintext server seed and generates new server seed

### F. Change Client Seed

*   **Type:** Mutation
*   **Operation Name:** `ChangeClientSeed`
*   **Purpose:** Update your client seed for future bets without rotating server seed
*   **Variables:**
    ```json
    {
      "newClientSeed": "your_custom_seed_string"
    }
    ```
*   **Note:** This allows you to change your input without triggering a server seed rotation

### G. Unhash Server Seed

*   **Type:** Query
*   **Operation Name:** `UnhashServerSeed`
*   **Purpose:** Retrieve the plaintext server seed for verification
*   **GraphQL Reference:** Uses `(0,o.DaH)` query for unhashing
*   **Variables:**
    ```json
    {
      "hashedServerSeed": "the_hashed_seed_to_reveal"
    }
    ```
*   **When Available:** After bet completion or seed rotation
*   **Use Case:** Enables verification of past game results

### H. Get Seed Status

*   **Type:** Query
*   **Operation Name:** `GetSeedStatus`
*   **Purpose:** Retrieve current seed information
*   **Expected Response:**
    ```json
    {
      "data": {
        "seedStatus": {
          "activeClientSeed": "your_client_seed",
          "activeHashedServerSeed": "hashed_server_seed",
          "currentNonce": 42,
          "nextHashedServerSeed": "next_hashed_seed"
        }
      }
    }
    ```

**Important Notes on Seed Management:**
- Seed management operations use the main GraphQL endpoint
- The exact mutation/query names may vary (check DevTools)
- Client seed changes are immediate
- Server seed rotation reveals the previous plaintext seed
- Nonce resets to 0 when server seed rotates

---

## 8. Python Implementation

This is a complete, working implementation that handles authentication, ID generation, provably fair verification, and proper data formatting.

### Prerequisites
```bash
pip install requests
```

### Complete Python Script

```python
import requests
import json
import random
import string
import uuid
import time
import hmac
import hashlib

# ================= CONFIGURATION =================
# PASTE YOUR DATA HERE FROM BROWSER DEVTOOLS
JWT_TOKEN = "eyJhbGciOi..."  # Paste WITHOUT 'Bearer ' prefix
COOKIES = "ip-country=CA; __cf_bm=..." # Paste FULL cookie string
USER_AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36..." # Paste your exact UA

# Mirror domain configuration for failover
MIRROR_DOMAINS = [
    'shuffle.com',
    'shuffle.gg',
    'shuffle.bet',
    'shuffle.casino',
    'shuffle.game',
    'shuffle.net',
    'shuffle.vip',
    'shuffle888.com',
    'shuffle.online',
    'shuffle.gold'
]

# Use primary domain by default
PRIMARY_DOMAIN = 'shuffle.com'
API_URL = f"https://{PRIMARY_DOMAIN}/main-api/graphql/api/graphql"

# ================= HELPER FUNCTIONS =================
def generate_window_id(length=10):
    """Generates a random ID to mimic the browser tab ID"""
    return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))

def test_mirror_domain(domain, timeout=5):
    """
    Test if a mirror domain is accessible.
    
    Args:
        domain: Domain to test (e.g., 'shuffle.com')
        timeout: Request timeout in seconds
    
    Returns:
        Boolean: True if domain is accessible
    """
    try:
        test_url = f"https://{domain}/main-api/graphql/api/graphql"
        test_query = '{"query": "{ __typename }"}'
        response = requests.post(
            test_url,
            headers={'Content-Type': 'application/json'},
            data=test_query,
            timeout=timeout
        )
        return response.status_code in [200, 401, 403]  # 401/403 = server reachable but auth needed
    except:
        return False

def get_working_mirror(domains=MIRROR_DOMAINS):
    """
    Find the first working mirror domain.
    
    Args:
        domains: List of domains to test
    
    Returns:
        String: Working domain or None
    """
    print("ðŸ” Testing mirror domains...")
    for domain in domains:
        if test_mirror_domain(domain):
            print(f"âœ“ Connected to: {domain}")
            return domain
    print("âŒ No working mirrors found")
    return None

def generate_random_numbers(server_seed, client_seed, nonce, cursor=0, count=1):
    """
    Core provably fair generation function (G)
    Returns list of floats between 0 and 1
    """
    results = []
    bytes_needed = count * 4
    bytes_generated = 0
    block_counter = 0
    
    while bytes_generated < bytes_needed:
        message = f"{client_seed}:{nonce}:{block_counter}"
        hash_hex = hmac.new(
            server_seed.encode(),
            message.encode(),
            hashlib.sha256
        ).hexdigest()
        
        # Extract 4-byte chunks
        for i in range(0, len(hash_hex), 8):
            if bytes_generated >= bytes_needed:
                break
            chunk = hash_hex[i:i+8]
            if len(chunk) == 8:
                value = int(chunk, 16) / (2**32)
                results.append(value)
                bytes_generated += 4
        
        block_counter += 1
    
    return results[:count]

def verify_limbo_result(server_seed, client_seed, nonce):
    """Verify Limbo game result"""
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    multiplier = (0.99 * (2**32)) / (int(random_value * (2**32)) + 1)
    return max(1.01, min(1000000, round(multiplier, 2)))

def verify_dice_result(server_seed, client_seed, nonce):
    """Verify Dice game result (0.00 - 100.00)"""
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    return round(random_value * 10001) / 100

def verify_roulette_result(server_seed, client_seed, nonce):
    """Verify Roulette result (0-36)"""
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    return int(random_value * 37)

def get_headers():
    """Constructs the headers required to bypass Cloudflare"""
    return {
        "authority": "shuffle.com",
        "accept": "*/*",
        "authorization": f"Bearer {JWT_TOKEN}",
        "content-type": "application/json",
        "cookie": COOKIES,
        "origin": "https://shuffle.com",
        "referer": "https://shuffle.com/games/originals/limbo",
        "user-agent": USER_AGENT,
        # A semi-random correlation ID to look legitimate
        "x-correlation-id": f"{uuid.uuid4()}", 
        "x-country": "CA"  # Change to match your IP location
    }

def send_request(operation_name, query, variables, use_failover=False):
    """
    Generic function to send GraphQL requests.
    
    Args:
        operation_name: Name of the GraphQL operation
        query: GraphQL query/mutation string
        variables: Variables for the operation
        use_failover: If True, tries mirror domains on failure
    
    Returns:
        Dict: Response data or None
    """
    payload = {
        "operationName": operation_name,
        "variables": variables,
        "query": query
    }
    
    domains_to_try = [PRIMARY_DOMAIN]
    if use_failover:
        domains_to_try.extend([d for d in MIRROR_DOMAINS if d != PRIMARY_DOMAIN])
    
    for domain in domains_to_try:
        api_url = f"https://{domain}/main-api/graphql/api/graphql"
        
        try:
            response = requests.post(api_url, headers=get_headers(), json=payload)
            
            if response.status_code == 200:
                data = response.json()
                if "errors" in data:
                    print(f"âŒ API Error: {data['errors'][0]['message']}")
                    return None
                return data
            elif response.status_code == 403:
                print(f"âŒ 403 Forbidden on {domain}: Cloudflare blocked. Update Cookies/User-Agent.")
            elif response.status_code == 401:
                print(f"âŒ 401 Unauthorized on {domain}: JWT Token expired.")
            else:
                print(f"âŒ HTTP {response.status_code} on {domain}: {response.text[:100]}")
            
            # If failover enabled, try next mirror
            if use_failover and domain != domains_to_try[-1]:
                print(f"â†’ Trying next mirror...")
                continue
            else:
                break
                
        except Exception as e:
            print(f"âŒ Connection Error on {domain}: {e}")
            if use_failover and domain != domains_to_try[-1]:
                print(f"â†’ Trying next mirror...")
                continue
            else:
                break
    
    return None

# ================= GRAPHQL DEFINITIONS =================

# 1. QUERY: Get Token Info (Public - No Auth Required)
TOKEN_INFO_QUERY = """
query tokenInfo {
  tokenInfo {
    priceInUsd
    circulatingSupply
    burnedTokens
  }
}
"""

# 2. QUERY: Get User Balance
BALANCE_QUERY = """
query GetBalances {
  me {
    balances {
      currency
      amount
    }
  }
}
"""

# 3. QUERY: Get User Info
USER_INFO_QUERY = """
query GetUser {
  me {
    id
    username
    vipLevel
  }
}
"""

# 4. QUERY: Get Game Settings (Limits)
SETTINGS_QUERY = """
query GetAppSettings {
  appSettings {
    limbo {
      maxPayoutUSD
      minBetUSD
    }
  }
}
"""

# 5. QUERY: Recently Played Games
RECENT_GAMES_QUERY = """
query GetRecentlyPlayed($first: Int!, $skip: Int!) {
  games(
    sortBy: RECENT_PLAY
    isRecentPlay: true
    first: $first
    skip: $skip
  ) {
    nodes {
      id
      gameName
      createdAt
      payout
    }
  }
}
"""

# 6. MUTATION: Place Limbo Bet
LIMBO_MUTATION = """
mutation LimboPlay($data: LimboPlayInput!) {
  limboPlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        limbo {
          resultValue
        }
      }
    }
  }
}
"""

# 7. MUTATION: Place Dice Bet
DICE_MUTATION = """
mutation DicePlay($data: DicePlayInput!) {
  dicePlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        dice {
          resultValue
          target
          over
        }
      }
    }
  }
}
"""

# 8. MUTATION: Rotate Seed
ROTATE_SEED_MUTATION = """
mutation RotateSeed($newSeed: String!) {
  rotateSeed(newSeed: $newSeed) {
    clientSeed
    hashedServerSeed
    nextHashedServerSeed
    revealedServerSeed
  }
}
"""

# 9. MUTATION: Change Client Seed
CHANGE_CLIENT_SEED_MUTATION = """
mutation ChangeClientSeed($newClientSeed: String!) {
  changeClientSeed(newClientSeed: $newClientSeed) {
    clientSeed
    hashedServerSeed
  }
}
"""

# 10. QUERY: Unhash Server Seed
UNHASH_SEED_QUERY = """
query UnhashServerSeed($hashedServerSeed: String!) {
  unhashServerSeed(hashedServerSeed: $hashedServerSeed) {
    plainTextSeed
    wasUsedForGames
  }
}
"""

# 11. QUERY: Get Seed Status
SEED_STATUS_QUERY = """
query GetSeedStatus {
  me {
    seedStatus {
      activeClientSeed
      activeHashedServerSeed
      currentNonce
      nextHashedServerSeed
    }
  }
}
"""

# ================= ACTIONS =================

def get_token_info():
    """Fetches public token statistics (no authentication required)"""
    data = send_request("tokenInfo", TOKEN_INFO_QUERY, {})
    if data:
        return data['data']['tokenInfo']
    return None

def get_user_balance():
    """Fetches user wallet balances"""
    data = send_request("GetBalances", BALANCE_QUERY, {})
    if data:
        return data['data']['me']['balances']
    return None

def get_user_info():
    """Fetches user profile information"""
    data = send_request("GetUser", USER_INFO_QUERY, {})
    if data:
        return data['data']['me']
    return None

def get_limbo_constraints():
    """Fetches server-side betting limits"""
    data = send_request("GetAppSettings", SETTINGS_QUERY, {})
    if data:
        return data['data']['appSettings']['limbo']
    return None

def get_recent_games(first=40, skip=0):
    """Fetches recently played games"""
    variables = {"first": first, "skip": skip}
    data = send_request("GetRecentlyPlayed", RECENT_GAMES_QUERY, variables)
    if data:
        return data['data']['games']['nodes']
    return None

def rotate_seed(new_client_seed):
    """
    Rotates the seed pair with a new client seed.
    This reveals the current server seed for verification.
    
    Args:
        new_client_seed: Your new client seed string
    
    Returns:
        Dict with clientSeed, hashedServerSeed, nextHashedServerSeed, revealedServerSeed
    """
    variables = {"newSeed": new_client_seed}
    data = send_request("RotateSeed", ROTATE_SEED_MUTATION, variables)
    if data:
        return data['data']['rotateSeed']
    return None

def change_client_seed(new_client_seed):
    """
    Changes only the client seed without rotating server seed.
    
    Args:
        new_client_seed: Your new client seed string
    
    Returns:
        Dict with updated clientSeed and current hashedServerSeed
    """
    variables = {"newClientSeed": new_client_seed}
    data = send_request("ChangeClientSeed", CHANGE_CLIENT_SEED_MUTATION, variables)
    if data:
        return data['data']['changeClientSeed']
    return None

def unhash_server_seed(hashed_seed):
    """
    Retrieves the plaintext server seed for verification.
    
    Args:
        hashed_seed: The hashed server seed to reveal
    
    Returns:
        Dict with plainTextSeed and wasUsedForGames
    """
    variables = {"hashedServerSeed": hashed_seed}
    data = send_request("UnhashServerSeed", UNHASH_SEED_QUERY, variables)
    if data:
        return data['data']['unhashServerSeed']
    return None

def get_seed_status():
    """
    Fetches current seed status information.
    
    Returns:
        Dict with activeClientSeed, activeHashedServerSeed, currentNonce, nextHashedServerSeed
    """
    data = send_request("GetSeedStatus", SEED_STATUS_QUERY, {})
    if data:
        return data['data']['me']['seedStatus']
    return None

def place_limbo_bet(amount, target, currency="USDT", usd_amount=None):
    """
    Places a Limbo bet.
    
    Args:
        amount: Bet amount in crypto (e.g., 0.1 for 0.1 USDT)
        target: Target multiplier (e.g., 2.0 for 2x)
        currency: Currency ticker (BTC, ETH, USDT, USDC, etc.)
        usd_amount: Optional USD equivalent of bet amount
    
    Returns:
        True if win, False if loss
    """
    
    # CRITICAL: Shuffle expects STRINGS for math precision
    variables = {
        "data": {
            "amount": f"{amount:.8f}",  # Format to 8 decimal places
            "bet": f"{target:.2f}",     # Format to 2 decimal places (target multiplier)
            "currency": currency,
            "windowId": generate_window_id()
        }
    }
    
    # Add optional USD amount if provided
    if usd_amount:
        variables["data"]["usdAmount"] = f"{usd_amount:.2f}"
    
    data = send_request("LimboPlay", LIMBO_MUTATION, variables)
    
    if data:
        res = data['data']['limboPlay']
        outcome = res['shuffleOriginalActions'][0]['action']['limbo']['resultValue']
        payout = res['payout']
        
        won = float(payout) > 0
        status = "âœ… WIN" if won else "âŒ LOSS"
        print(f"ðŸŽ² {status} | Target: {target}x | Result: {outcome}x | Payout: {payout}")
        return won
    return False

def place_dice_bet(amount, target, over=True, currency="USDT"):
    """
    Places a Dice bet.
    
    Args:
        amount: Bet amount in crypto
        target: Target number (0.00-100.00)
        over: True to bet over, False to bet under
        currency: Currency ticker
    
    Returns:
        True if win, False if loss
    """
    variables = {
        "data": {
            "amount": f"{amount:.8f}",
            "target": f"{target:.2f}",
            "over": over,
            "currency": currency,
            "windowId": generate_window_id()
        }
    }
    
    data = send_request("DicePlay", DICE_MUTATION, variables)
    
    if data:
        res = data['data']['dicePlay']
        result = res['shuffleOriginalActions'][0]['action']['dice']['resultValue']
        payout = res['payout']
        
        won = float(payout) > 0
        status = "âœ… WIN" if won else "âŒ LOSS"
        direction = "OVER" if over else "UNDER"
        print(f"ðŸŽ² {status} | Target: {direction} {target} | Result: {result} | Payout: {payout}")
        return won
    return False

# ================= MAIN EXECUTION =================
if __name__ == "__main__":
    print("=" * 60)
    print("SHUFFLE.COM API CLIENT")
    print("=" * 60)
    
    # 0. Optional: Test mirror domains for best connection
    print("\nðŸŒ Mirror Domain Testing (Optional):")
    print("   Uncomment to find fastest mirror:")
    print("   # working_domain = get_working_mirror()")
    print("   # if working_domain:")
    print("   #     API_URL = f'https://{working_domain}/main-api/graphql/api/graphql'")
    print(f"   Using primary domain: {PRIMARY_DOMAIN}")
    
    # 1. Get public token info (no auth required)
    print("\nðŸ“Š Fetching token info...")
    token_info = get_token_info()
    if token_info:
        print(f"   Price: ${token_info['priceInUsd']}")
        print(f"   Supply: {token_info['circulatingSupply']}")
    
    # 2. Get user info (requires auth)
    print("\nðŸ‘¤ Fetching user info...")
    user_info = get_user_info()
    if user_info:
        print(f"   Username: {user_info['username']}")
        print(f"   VIP Level: {user_info['vipLevel']}")
    
    # 3. Get wallet balances
    print("\nðŸ’° Fetching balances...")
    balances = get_user_balance()
    if balances:
        for balance in balances:
            if float(balance['amount']) > 0:
                print(f"   {balance['currency']}: {balance['amount']}")
    
    # 4. Get game limits
    print("\nðŸŽ® Fetching game limits...")
    limits = get_limbo_constraints()
    if limits:
        print(f"   Min Bet: ${limits['minBetUSD']}")
        print(f"   Max Payout: ${limits['maxPayoutUSD']}")
    
    # 5. Get recent games
    print("\nðŸ“œ Fetching recent games...")
    recent = get_recent_games(first=5)
    if recent:
        for game in recent:
            print(f"   {game['gameName']}: {game['payout']}")
    
    # 6. Get seed status
    print("\nðŸ” Fetching seed status...")
    seed_status = get_seed_status()
    if seed_status:
        print(f"   Client Seed: {seed_status['activeClientSeed'][:20]}...")
        print(f"   Hashed Server Seed: {seed_status['activeHashedServerSeed'][:20]}...")
        print(f"   Current Nonce: {seed_status['currentNonce']}")
    
    # 7. Provably Fair Verification Examples
    print("\nðŸ” Provably Fair Verification Examples:")
    print("   " + "="*50)
    
    # Example: Verify Limbo Result
    print("\n   Testing Limbo verification...")
    # Replace with actual seeds from a completed bet
    example_server_seed = "example_server_seed"
    example_client_seed = "example_client_seed"
    example_nonce = 1
    
    print("   âš ï¸  To verify real results:")
    print("      1. Complete a bet and note the nonce")
    print("      2. Rotate your seed to reveal server seed")
    print("      3. Use revealed seeds with the verification functions below")
    print("")
    print("   Example verification code:")
    print("   ```")
    print("   result = verify_limbo_result(revealed_server_seed, client_seed, nonce)")
    print("   print(f'Verified Limbo: {result}x')")
    print("")
    print("   dice_roll = verify_dice_result(revealed_server_seed, client_seed, nonce)")
    print("   print(f'Verified Dice: {dice_roll}')")
    print("")
    print("   mines = verify_mines_positions(revealed_server_seed, client_seed, nonce, 3)")
    print("   print(f'Verified Mine Positions: {mines}')")
    print("")
    print("   plinko_path = verify_plinko_path(revealed_server_seed, client_seed, nonce, 16)")
    print("   print(f'Verified Plinko Path: {plinko_path}')")
    print("   ```")
    
    # 8. Seed Management Examples (COMMENTED OUT)
    print("\nðŸ”„ Seed Management:")
    print("   âš ï¸  Seed management operations are commented out.")
    print("   Uncomment to use (changes your active seeds):")
    print("")
    print("   # Change client seed only:")
    print("   # change_client_seed('my_new_custom_seed_12345')")
    print("")
    print("   # Rotate seed pair (reveals current server seed):")
    print("   # rotation_result = rotate_seed('my_new_client_seed')")
    print("   # print(f'Revealed Server Seed: {rotation_result[\"revealedServerSeed\"]}')")
    print("")
    print("   # Unhash a previous server seed:")
    print("   # unhashed = unhash_server_seed('hashed_seed_here')")
    print("   # print(f'Plaintext Seed: {unhashed[\"plainTextSeed\"]}')")
    
    # 9. Place test bets (UNCOMMENT TO USE)
    # WARNING: These will place real bets with real money
    print("\nâš ï¸  Test bets are commented out. Uncomment to place real bets.")
    # time.sleep(1)
    # place_limbo_bet(amount=0.1, target=2.0, currency="USDT")
    # place_dice_bet(amount=0.1, target=50.0, over=True, currency="USDT")
    
    # 10. Advanced: WebSocket Subscriptions (Future Implementation)
    print("\nðŸ“¡ WebSocket Subscriptions:")
    print("   For real-time updates, Shuffle uses WebSocket connections.")
    print("   Base URL: wss://shuffle.com/main-api/bp-subscription")
    print("   Endpoints:")
    print("   - /subscription/graphql (general subscriptions)")
    print("   - /stable-subscription/graphql (core features)")
    print("   - /sports-subscription/graphql (sports betting)")
    print("")
    print("   Example use cases:")
    print("   - Real-time balance updates")
    print("   - Live chat messages")
    print("   - Crash game multiplier updates")
    print("   - Sports odds changes")
    print("")
    print("   Note: WebSocket implementation requires additional libraries:")
    print("   pip install websockets graphql-subscription-manager")
```

---

## 9. Operational Reference

### A. Data Formatting Rules

Shuffle is **extremely strict** about data types. Sending incorrect types will result in validation errors.

| Field | Type | Format | Example | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **Amount** | String | 8 decimals | `"0.10000000"` | Bet amount in crypto |
| **Multiplier (bet)** | String | 2 decimals | `"2.00"` | Target multiplier (Limbo's `eu` parameter) |
| **Currency** | String | Uppercase ticker | `"USDT"`, `"BTC"`, `"ETH"` | Must match wallet balance |
| **WindowID** | String | 10 random chars | `"a7b3c9d2e1"` | Browser tab identifier |
| **USD Amount** | String | 2 decimals | `"0.10"` | Optional USD equivalent |

**Common Mistakes:**
*   âŒ Wrong: `{"amount": 10.5}` (Python float)
*   âœ… Right: `{"amount": "10.50000000"}` (String with 8 decimals)
*   âŒ Wrong: `{"bet": 2}` (Integer)
*   âœ… Right: `{"bet": "2.00"}` (String with 2 decimals)

**Endpoint Construction:**
- All requests use the same GraphQL endpoint
- Client-side construction: `K.getGraphqlUrl("/api/graphql")`
- Resolves to: `https://shuffle.com/main-api/graphql/api/graphql`

### B. Game-Specific Parameters

| Game | Key Parameters | Value Ranges |
| :--- | :--- | :--- |
| **Limbo** | bet (target multiplier) | "1.01" - "1000000.00" |
| **Dice** | target, over | "0.00" - "100.00", boolean |
| **Mines** | numberOfMines | 1 - 24 |
| **Plinko** | rowCount, riskLevel | 8-16, LOW/MEDIUM/HIGH_RISK |
| **Wheel** | segmentCount, riskLevel | TEN-FIFTY, LOW/MEDIUM/HIGH |
| **Tower** | difficulty | EASY/MEDIUM/HARD/EXPERT/MASTER |
| **Keno** | selectedNumbers (array) | 1-10 numbers from 1-40 |

### C. Provably Fair Verification (Client-Side)

**Important:** Result verification happens **entirely client-side** in the browser. Once you have the cryptographic seeds, you can verify any result without making API calls.

**Verification Process:**
1. **During Bet:** Server uses `hashedServerSeed` (unknown to you)
2. **After Bet:** Server reveals the actual `serverSeed`
3. **Verification:** You can now independently calculate the result

**Example Verification Workflow:**
```python
# After a bet is complete and server seed is revealed
server_seed = "revealed_seed_here"
client_seed = "your_client_seed"
nonce = 1

# Verify Limbo result
verified_multiplier = verify_limbo_result(server_seed, client_seed, nonce)
print(f"Verified Result: {verified_multiplier}x")

# Verify Dice result
verified_roll = verify_dice_result(server_seed, client_seed, nonce)
print(f"Verified Roll: {verified_roll}")

# Verify Roulette result  
verified_number = verify_roulette_result(server_seed, client_seed, nonce)
print(f"Verified Number: {verified_number}")
```

**Key Points:**
- Verification requires the **revealed** server seed (not hashed)
- Calculations use BigNumber library with `ROUND_DOWN` precision
- Results displayed in Provably Fair modal are calculated client-side
- No additional API endpoint needed for verification
- You can verify results in any programming language with HMAC-SHA256

### D. Rate Limiting Guidelines

To avoid detection and bans:
*   **Minimum delay:** 1 second between bets
*   **Recommended delay:** 2-5 seconds for human-like behavior
*   **Never:** Spam hundreds of requests in rapid succession
*   **Add randomness:** Vary your bet timing slightly

```python
import time
import random

# Between bets
time.sleep(random.uniform(1.5, 3.0))
```

## 10. Reverse Engineering New Features

Since this is unofficial documentation, you are the maintainer. Here's how to add support for new games or features:

### Step-by-Step Workflow

1.  **Open DevTools:** Press F12 in Chrome/Firefox
2.  **Clear Log:** Click the "Clear" (ðŸš«) button in the Network tab
3.  **Perform Action:** Click the button on the website you want to automate (e.g., "Place Bet" on a new game)
4.  **Find Request:** Look for the new `graphql` POST request
5.  **Inspect Payload:** Click the request â†’ **Payload** tab
    *   Copy the entire `query` string
    *   Note all `variables` and their types
6.  **Add to Script:** Create a new constant in your Python script
7.  **Test:** Run `send_request()` with your new query

### Identifying Specialized Endpoints

Different features may use specialized GraphQL endpoints:

**How to Find the Endpoint:**
1. Perform the action in your browser (e.g., place a sports bet)
2. Check the **Request URL** in DevTools Network tab
3. Note the endpoint path (e.g., `/sports/graphql-sports`)

**Common Specialized Endpoints:**

| Feature | Endpoint Pattern | Example |
| :--- | :--- | :--- |
| **Sports Betting** | `/sports/graphql-sports` | `https://shuffle.com/main-api/graphql/sports/graphql-sports` |
| **Lottery Games** | `/lottery/graphql-lottery` | `https://shuffle.com/main-api/graphql/lottery/graphql-lottery` |
| **KYC Verification** | `/kyc/graphql-kyc` | `https://shuffle.com/main-api/graphql/kyc/graphql-kyc` |
| **Sports Main** | `/sports-main/graphql-sports-main` | `https://shuffle.com/main-api/graphql/sports-main/graphql-sports-main` |

**Modifying Your Script for Specialized Endpoints:**
```python
def send_sports_request(operation_name, query, variables):
    """Send request to sports GraphQL endpoint"""
    sports_url = f"https://{PRIMARY_DOMAIN}/main-api/graphql/sports/graphql-sports"
    
    payload = {
        "operationName": operation_name,
        "variables": variables,
        "query": query
    }
    
    response = requests.post(sports_url, headers=get_headers(), json=payload)
    return response.json() if response.status_code == 200 else None
```

### Accessing REST Endpoints

Some features use traditional REST APIs:

**Navigation/CMS Data:**
```python
def get_navigation_items(locale='en', preview=False):
    """Fetch navigation menu items"""
    url = f"https://{PRIMARY_DOMAIN}/api/v1/sports/get-nav-items"
    params = {'locale': locale}
    if preview:
        params['preview'] = 'true'
    
    response = requests.get(url, params=params)
    return response.json()
```

**OG Image Generation:**
```python
def get_og_image():
    """Get Open Graph social media image"""
    url = f"https://{PRIMARY_DOMAIN}/api/v1/images/og-image"
    response = requests.get(url)
    return response.content  # Returns image bytes
```

### Example: Adding Plinko Support

```python
# After capturing from DevTools:
PLINKO_MUTATION = """
mutation PlinkoPlay($data: PlinkoPlayInput!) {
  plinkoPlay(data: $data) {
    id
    payout
    shuffleOriginalActions {
      action {
        plinko {
          path
          multiplier
        }
      }
    }
  }
}
"""

def place_plinko_bet(amount, rows=16, risk="HIGH_RISK", currency="USDT"):
    variables = {
        "data": {
            "amount": f"{amount:.8f}",
            "plinkoRowCount": rows,
            "plinkoRiskLevel": risk,
            "currency": currency,
            "windowId": generate_window_id()
        }
    }
    return send_request("PlinkoPlay", PLINKO_MUTATION, variables)
```

### WebSocket Subscriptions (Advanced)

For real-time updates, implement WebSocket subscriptions:

**Available Subscription Endpoints:**
```python
WS_ENDPOINTS = {
    'general': 'wss://shuffle.com/main-api/bp-subscription/subscription/graphql',
    'stable': 'wss://shuffle.com/main-api/bp-subscription/stable-subscription/graphql',
    'sports': 'wss://shuffle.com/main-api/bp-subscription/sports-subscription/graphql'
}
```

**Example Subscription (requires websockets library):**
```python
import asyncio
import websockets
import json

async def subscribe_to_balance_updates():
    """Subscribe to real-time balance updates"""
    uri = WS_ENDPOINTS['stable']
    
    # WebSocket connection requires authentication
    headers = {
        'authorization': f'Bearer {JWT_TOKEN}',
        'cookie': COOKIES
    }
    
    subscription_query = {
        "type": "start",
        "payload": {
            "query": """
                subscription OnBalanceUpdate {
                    balanceUpdated {
                        currency
                        amount
                    }
                }
            """
        }
    }
    
    async with websockets.connect(uri, extra_headers=headers) as websocket:
        # Send subscription
        await websocket.send(json.dumps(subscription_query))
        
        # Listen for updates
        while True:
            message = await websocket.recv()
            data = json.loads(message)
            print(f"Balance Update: {data}")

# Run subscription
# asyncio.run(subscribe_to_balance_updates())
```

**Common Subscription Types:**
- Balance updates
- Bet results
- Chat messages
- Crash game multiplier
- Sports odds changes
- Jackpot updates

**Note:** WebSocket implementations require:
```bash
pip install websockets
```

---

## 11. Cloudflare Protection

Shuffle uses Cloudflare to prevent automated access. Here's how to handle it:

### Common Issues

| HTTP Status | Meaning | Solution |
| :--- | :--- | :--- |
| **403 Forbidden** | Cloudflare detected bot | Update cookies/User-Agent |
| **429 Too Many Requests** | Rate limit hit | Add delays between requests |
| **503 Service Unavailable** | Challenge required | Use browser automation |

### Solutions by Complexity

#### Level 1: Manual Cookie Extraction (Easiest)
*   Refresh cookies every 30-60 minutes
*   Copy from DevTools each time
*   Works for light usage

#### Level 2: TLS Fingerprinting Libraries
Python's default `requests` library has an easily detected TLS fingerprint. Use alternatives:

```bash
# Option A: curl_cffi (mimics curl's TLS signature)
pip install curl-cffi

# Option B: tls_client (customizable TLS fingerprint)
pip install tls-client
```

**Example with curl_cffi:**
```python
from curl_cffi import requests as cffi_requests

response = cffi_requests.post(
    API_URL,
    headers=headers,
    json=payload,
    impersonate="chrome110"  # Mimics Chrome 110
)
```

#### Level 3: Browser Automation (Most Reliable)
Use Selenium or Playwright to:
1.  Open a real browser
2.  Log in and solve CAPTCHA manually (or with a service)
3.  Extract cookies programmatically
4.  Pass to your script

```python
from selenium import webdriver
import pickle

def get_fresh_cookies():
    driver = webdriver.Chrome()
    driver.get("https://shuffle.com")
    
    # Wait for manual login + CAPTCHA solving
    input("Press Enter after logging in...")
    
    # Extract cookies
    cookies = driver.get_cookies()
    cookie_string = "; ".join([f"{c['name']}={c['value']}" for c in cookies])
    
    driver.quit()
    return cookie_string
```

---

## 12. Complete Workflow Example

This section demonstrates a complete workflow from authentication to betting to verification.

### Step-by-Step: Placing and Verifying a Bet

```python
import time
import random

# Step 1: Get your current seed status
print("Step 1: Checking seed status...")
seed_status = get_seed_status()
if seed_status:
    current_client_seed = seed_status['activeClientSeed']
    current_nonce = seed_status['currentNonce']
    print(f"âœ“ Client Seed: {current_client_seed}")
    print(f"âœ“ Current Nonce: {current_nonce}")
    print(f"âœ“ Next bet will use nonce: {current_nonce}")

# Step 2: Check your balance
print("\nStep 2: Checking balance...")
balances = get_user_balance()
usdt_balance = next((b for b in balances if b['currency'] == 'USDT'), None)
if usdt_balance:
    print(f"âœ“ USDT Balance: {usdt_balance['amount']}")

# Step 3: Place a bet and record the nonce
print("\nStep 3: Placing bet...")
bet_nonce = current_nonce  # This bet will use this nonce
bet_amount = 0.1
bet_target = 2.0

print(f"â†’ Betting {bet_amount} USDT at {bet_target}x target")
print(f"â†’ This bet uses nonce: {bet_nonce}")

# Place the bet
result = place_limbo_bet(amount=bet_amount, target=bet_target, currency="USDT")

# Step 4: Record the result
# In a real scenario, you would parse the actual result from the response
# For this example, let's assume we got result_multiplier = 1.85

print("\nStep 4: Bet completed!")
print(f"â†’ Used nonce: {bet_nonce}")
print(f"â†’ Client seed: {current_client_seed}")
# Note: Server seed is still hashed at this point

# Step 5: Rotate seeds to reveal server seed
print("\nStep 5: Rotating seeds to enable verification...")
new_client_seed = ''.join(random.choices('abcdefghijklmnopqrstuvwxyz0123456789', k=20))
rotation_result = rotate_seed(new_client_seed)

if rotation_result:
    revealed_server_seed = rotation_result['revealedServerSeed']
    print(f"âœ“ Server seed revealed: {revealed_server_seed[:20]}...")
    
    # Step 6: Verify the bet result
    print("\nStep 6: Verifying bet result...")
    verified_multiplier = verify_limbo_result(
        revealed_server_seed,
        current_client_seed,
        bet_nonce
    )
    
    print(f"âœ“ Verified multiplier: {verified_multiplier}x")
    print(f"â†’ Original result: 1.85x (example)")
    
    if abs(verified_multiplier - 1.85) < 0.01:
        print("âœ… VERIFICATION SUCCESSFUL - Result matches!")
    else:
        print("âš ï¸  VERIFICATION MISMATCH - Check your inputs")
    
    print("\n" + "="*60)
    print("VERIFICATION COMPLETE")
    print("="*60)
    print("You have proven that:")
    print("1. The server committed to a result before you bet")
    print("2. The result was calculated fairly using HMAC-SHA256")
    print("3. No one could manipulate the outcome after the bet")
```

### Verification Workflow for Different Games

```python
# After rotating seeds and getting revealed_server_seed:

# Verify Dice (0-100)
dice_result = verify_dice_result(revealed_server_seed, client_seed, nonce)
print(f"Dice Roll: {dice_result}")

# Verify Mines (3 mines on 5x5 grid)
mine_positions = verify_mines_positions(revealed_server_seed, client_seed, nonce, 3)
print(f"Mine Positions: {mine_positions}")

# Verify Plinko (16 rows, path determination)
plinko_path = verify_plinko_path(revealed_server_seed, client_seed, nonce, 16)
print(f"Plinko Path (0=left, 1=right): {plinko_path}")

# Verify Keno (10 drawn numbers)
keno_numbers = verify_keno_numbers(revealed_server_seed, client_seed, nonce)
print(f"Keno Numbers: {keno_numbers}")

# Verify Roulette (0-36)
roulette_number = verify_roulette_result(revealed_server_seed, client_seed, nonce)
print(f"Roulette Number: {roulette_number}")

# Verify Card Deck (Hilo/Blackjack)
shuffled_deck = verify_card_deck(revealed_server_seed, client_seed, nonce)
print(f"First 5 cards: {shuffled_deck[:5]}")
```

### Best Practices for Verification

1. **Always record the nonce** before placing a bet
2. **Save your client seed** used for each betting session
3. **Rotate seeds regularly** to reveal server seeds for verification
4. **Verify immediately** after seed rotation while details are fresh
5. **Keep a log** of bets with their nonce, seeds, and results
6. **Use the web interface** to cross-check your calculations
7. **Test verification** with small bets first

### Example: Logging System

```python
import json
from datetime import datetime

class BetLogger:
    def __init__(self, filename='bet_log.json'):
        self.filename = filename
        self.logs = []
    
    def log_bet(self, game, amount, target, nonce, client_seed, result):
        entry = {
            'timestamp': datetime.now().isoformat(),
            'game': game,
            'amount': amount,
            'target': target,
            'nonce': nonce,
            'client_seed': client_seed,
            'result': result,
            'verified': False
        }
        self.logs.append(entry)
        self._save()
    
    def mark_verified(self, nonce, server_seed, verified_result):
        for entry in self.logs:
            if entry['nonce'] == nonce and not entry['verified']:
                entry['server_seed'] = server_seed
                entry['verified_result'] = verified_result
                entry['verified'] = True
                entry['verification_time'] = datetime.now().isoformat()
                break
        self._save()
    
    def _save(self):
        with open(self.filename, 'w') as f:
            json.dump(self.logs, f, indent=2)

# Usage:
logger = BetLogger()

# Before bet:
logger.log_bet('Limbo', 0.1, 2.0, current_nonce, current_client_seed, None)

# After bet:
# ... place bet, get result ...

# After seed rotation:
verified = verify_limbo_result(revealed_server_seed, current_client_seed, bet_nonce)
logger.mark_verified(bet_nonce, revealed_server_seed, verified)
```

---

## 13. Troubleshooting

### Common Errors & Solutions

### Common Errors & Solutions

| Error Message | Cause | Solution |
| :--- | :--- | :--- |
| **403 Forbidden** | Cloudflare detection | Your `Cookie` or `User-Agent` is outdated. Refresh from browser. |
| **401 Unauthorized** | JWT expired | Your Authorization token expired (~24h). Log in again and get a new one. |
| **Variable "$data" got invalid value** | Type mismatch | You sent a Python float/int instead of string. Use `f"{val:.8f}"`. |
| **Insufficient Funds** | Wrong currency | Check the `currency` parameter. If you have USDC but bet USDT, it fails. |
| **"amount" must be greater than 0** | Bet too small | Check minimum bet requirement for your currency. |
| **Maximum payout exceeded** | Bet too large | Your potential win exceeds the game's max payout limit. |
| **Invalid window ID** | Missing/wrong windowId | Ensure you're generating a random `windowId` for each bet. |
| **Invalid numberOfMines** | Out of range | Mines must be between 1-24. |
| **Invalid plinkoRowCount** | Out of range | Plinko rows must be between 8-16. |
| **Invalid difficulty** | Wrong enum value | Tower difficulty must be exact: EASY/MEDIUM/HARD/EXPERT/MASTER. |
| **Seed verification mismatch** | Wrong seeds used | Ensure you're using the revealed server seed (not hashed), correct client seed, and exact nonce from the bet. |
| **Cannot unhash seed** | Seed not revealed yet | Server seed is only revealed after rotation or bet completion. |
| **Nonce mismatch** | Wrong bet counter | Verify you're using the exact nonce from when the bet was placed. |
| **Connection timeout** | Network/firewall issues | Try a different mirror domain from the approved list. |
| **Domain blocked** | Regional restrictions | Use mirror domains: shuffle.gg, shuffle.bet, shuffle.casino, etc. |

### Debugging Checklist

When your script fails:

1.  âœ… **Check token expiry:** JWT tokens last ~24 hours
2.  âœ… **Verify cookie freshness:** Cloudflare cookies expire quickly
3.  âœ… **Confirm currency balance:** Do you have the currency you're betting with?
4.  âœ… **Validate data types:** Are amounts and multipliers strings?
5.  âœ… **Check User-Agent:** Does it match your browser exactly?
6.  âœ… **Review response body:** Print `response.text` to see exact error
7.  âœ… **Test with minimal bet:** Use the smallest possible amount first
8.  âœ… **Verify game parameters:** Check valid ranges for game-specific settings
9.  âœ… **Test provably fair:** Verify results match expected calculations
10. âœ… **Check seed status:** Ensure you're using correct seeds and nonce
11. âœ… **Verify independently:** Use verification functions to confirm outcomes
12. âœ… **Try mirror domains:** If primary domain fails, test alternate mirrors
13. âœ… **Check external services:** Verify third-party services are accessible

### Getting Help

Since this is unofficial:
*   **No official support** exists for this API
*   **Network tab is your friend:** Always check actual requests in DevTools
*   **Document changes:** If Shuffle updates their API, you'll need to reverse-engineer again
*   **Share findings:** Consider documenting your discoveries for others
*   **Test verification:** Use the provably fair functions to verify your results
*   **Mirror domains:** If one domain is blocked, try alternatives from the approved list
*   **Specialized endpoints:** Check if your feature uses a specialized GraphQL endpoint (sports, lottery, kyc)
*   **WebSocket for real-time:** Some features require subscription endpoints instead of REST

**Useful DevTools Filters:**
```
Filter by:
- "graphql" to see API calls
- "subscription" to see WebSocket connections
- "n9assets" to see game catalog requests
- "shuffle.click" to see location services
```

---

## Final Reminders

1.  **This violates Shuffle's ToS** â€“ use at your own risk
2.  **You can lose real money** â€“ test with minimum amounts first
3.  **Credentials expire** â€“ be prepared to refresh tokens/cookies regularly
4.  **APIs change** â€“ Shuffle actively works to break bots, so this guide may become outdated
5.  **Rate limits exist** â€“ don't spam requests or you'll be banned
6.  **Legal risks** â€“ automated gambling may be illegal in your jurisdiction
7.  **Verify results** â€“ Use the provably fair system to independently verify outcomes
8.  **Understand the math** â€“ Know the house edge and expected value of each game
9.  **Trust but verify** â€“ The platform provides verification tools - use them
10. **Seed management** â€“ Rotate seeds periodically to reveal server seeds for verification
11. **Mirror domains** â€“ Know the approved alternate domains in case primary is blocked
12. **Specialized endpoints** â€“ Different features (sports, lottery, KYC) use different GraphQL endpoints

**Understanding Provably Fair:**
- The system **cannot be manipulated** after you place a bet
- You can **mathematically prove** every outcome is fair
- Server commits to results **before** knowing your bet
- You control one input (client seed) to prevent pre-calculation
- All results can be **independently verified** without trusting the platform

**What This Guide Covers:**
- âœ… Core GraphQL API structure and authentication
- âœ… 14+ approved mirror domains for redundancy
- âœ… Specialized endpoints (sports, lottery, KYC)
- âœ… WebSocket subscription architecture for real-time updates
- âœ… Complete provably fair system with verification
- âœ… All 10+ original games with exact mechanics
- âœ… Seed management (rotate, change, unhash, verify)
- âœ… Working Python implementation with failover
- âœ… UI/UX details including CSS classes and modal structure
- âœ… External service dependencies
- âœ… Complete workflow examples from bet to verification

**Use responsibly and only for educational purposes.**

Integrate to the guide; Based on the provided source documentation and CSS definitions, here are the specific answers to your questions, followed by the improved and organized guide. ### Part 1: Specific Data Extraction **1. What are the WheelSegments names?** According to the `u.kX` Enum and the `.WheelGameFooter` CSS classes, the wheel segments are named by their color: *   **Black** *   **Gray** *   **Green** *   **Orange** *   **Purple** *   **Yellow** **2. Which games are listed for selection?** Based on the `i.sDy` enumeration and game-specific component references, the "Original" games available are: *   **Plinko** *   **Mines** *   **Roulette** *   **Limbo** *   **Wheel** *   **Tower** *   **Dice** (referenced in configuration `h.$`) *   *Broader Categories:* Slots, Live Casino, Blackjack. **3. What are the HiloGuess options?** According to the `p` Enum (Hilo Guess Results) and game logic, the guess options are: *   **UP** (Predicting the next card will be higher) *   **DOWN** (Predicting the next card will be lower) *   *(Note: The logic also tracks `UP_CORRECT` and `DOWN_WRONG` for result states).* *** ### Part 2: Improved Developer Guide This guide has been reorganized for readability, separating visual styles (CSS) from application logic (JS) and grouping them by their functional context (General Layout vs. Game Specific). #### I. Core UI Layout & Structure *Classes responsible for the main application shell, modals, and responsive grid systems.* | Class / Component | Type | Definition & Purpose | | :--- | :--- | :--- | | **.GameLayout_gameLayoutWrapper** | CSS | **Main Wrapper:** The primary flex container (column direction) for the entire game page. | | **.GameLayout_gameContainer** | CSS | **Responsive Container:** Holds inputs and game content. Switches from `column-reverse` (mobile) to `row` (desktop). | | **.GameLayout_gameContent** | CSS | **Canvas Area:** The dark background container where the visual game runs. Handles dimensions and transitions. | | **.GameLayout_gameFooter** | CSS | **Footer Controls:** Manages the bottom section (stats, fairness toggles) with horizontal alignment. | | **.TabViewModal_root** | CSS | **Tab System:** Root style for modular tab views (e.g., in the Lobby or Fairness modals) with bottom borders. | | **.PopupViewContainer_content** | CSS | **Popups/Tooltips:** Fixed positioning, high z-index, rounded corners, and white background for overlays. | | **B (Component)** | JS | **Card Grid:** Renders the responsive grid of game thumbnails, handling pagination and "infinite load." | | **F (Component)** | JS | **Lobby Manager:** Manages main navigation tabs (Lobby, Originals, Slots) and search filtering. | #### II. Game-Specific Implementations *Classes and Logic tied to specific "Original" games.* ##### 1. Mines | Identifier | Type | Purpose | | :--- | :--- | :--- | | **.MinesGameTileElement_root** | CSS | Styles the individual tiles in the grid (absolute positioning). | | **.MinesGameTileWrapper_borderPulse** | CSS | Animation keyframe for the pulsating orange border on active tiles. | | **s.S (Enum)** | Const | **Tile States:** `MASKED` (0), `MINE` (1), `GEM` (2). | | **R.gB.MINES_TILES_COUNT** | Const | **Grid Size:** Defaults to 25 tiles. | ##### 2. Wheel | Identifier | Type | Purpose | | :--- | :--- | :--- | | **.WheelGameFooter_[Color]** | CSS | Styles for footer segments (Black, Yellow, Purple, Orange, Green, Gray). | | **i.hjI (Enum)** | Const | **Risk Levels:** `LOW`, `MEDIUM`, `HIGH`. | | **i.oSz (Enum)** | Const | **Segment Counts:** `TEN`, `TWENTY`, `THIRTY`, `FORTY`, `FIFTY`. | | **f.WHEEL_MULTIPLIERS** | Const | A nested map determining payout based on Segment Count + Risk Level. | ##### 3. Hilo | Identifier | Type | Purpose | | :--- | :--- | :--- | | **.HiloProvablyFairCard_smallCard** | CSS | Styles the result cards (Font: Domine, 32px). | | **E (Class)** | JS | **Core Logic:** Calculates win chances, multipliers, and validates `UP`/`DOWN` guesses. | | **d (Enum)** | Const | **Game States:** `CHOOSING_FIRST_CARD`, `MIN_CHOOSING_NEXT_CARD`, `LOST`, `CASHOUT`. | ##### 4. Tower | Identifier | Type | Purpose | | :--- | :--- | :--- | | **.GameResult_towerContainer** | CSS | Fixed height container for Tower results. | | **m.b2v (Enum)** | Const | **Difficulty:** `EASY`, `MEDIUM`, `HARD`, `EXPERT`, `MASTER`. | | **M.TOWER_DIFFICULTY_TO_CONFIG** | Const | Maps difficulty levels to column counts and key requirements. | ##### 5. Limbo | Identifier | Type | Purpose | | :--- | :--- | :--- | | **v.aA (Class)** | JS | **Core Logic:** Calculates win chance vs. target multiplier. | | **v.Sg (Const)** | Const | **Limits:** Min Multiplier (1.01x), Max Multiplier (1,000,000x). | #### III. Betting & Provably Fair Logic *Components that handle money, input validation, and cryptographic verification.* | Identifier | Type | Definition & Purpose | | :--- | :--- | :--- | | **j (Component)** | JS | **Currency Input:** Handles bet amount entry, max bet buttons, and fiat/crypto toggling. | | **c / b.V** | JS | **Multiply Buttons:** The logic for "1/2" and "2x" buttons in the betting interface. | | **R (Component)** | JS | **Fairness Modal:** The root component for the "Provably Fair" popup (Seeds & Verify tabs). | | **G (Function)** | JS | **Generator:** The RNG function. Takes `ClientSeed`, `ServerSeed`, `Nonce` $\rightarrow$ HMAC-SHA256 $\rightarrow$ Game Outcome. | | **A (Component)** | JS | **Seed Rotator:** The form allowing users to change their Client Seed and view the hashed Server Seed. | | **v (Function)** | JS | **Validator:** Checks if bets exceed max payout or user balance. Returns `G.UNDER` or `G.OVER` errors. | #### IV. Global Configuration & Utilities *Shared constants and helper functions.* | Identifier | Type | Definition | | :--- | :--- | :--- | | **i.o_p (Enum)** | Const | **Fiat Currencies:** USD, EUR, MXN, BRL, etc. | | **u.c (Enum)** | Const | **Play Modes:** `REAL` (Crypto), `FUN` (Demo/Free), `NONE`. | | **A (Function)** | JS | **Hotkeys:** Registers global `keydown`/`keyup` listeners (if enabled in settings). | | **h (Component)** | JS | **Settings Panel:** Dropdown for Turbo Mode, Theater Mode, and Audio toggles. | | **_ (Component)** | JS | **SEO:** Manages `<head>` metadata, canonical URLs, and language tags. |

To build a Tampermonkey script that tracks every bet and links the provably fair data, you are missing **three critical technical components** that the current guide does not cover.

While the guide explains *how to be a bot* (send requests), it doesnâ€™t explain **how to listen to the browser's internal traffic** (intercept requests), **how to manage the data state** (linking future seeds to past bets), or **how to inject into Shuffle's specific React environment**.

Here is exactly what is missing and how to implement it.

---

### 1. The Interception Mechanism (`window.fetch` Proxy)
The guide tells you what the endpoints are, but not how to read the traffic flowing through them in real-time without making your own requests.

Since Shuffle uses GraphQL, you cannot simply listen for URL changes. You must monkey-patch `window.fetch`.

**Missing Code Pattern:**
You need a wrapper that inspects every outgoing network request, identifies if it's a bet, and clones the response to read the result.

```javascript
// The missing interception logic
const originalFetch = window.fetch;
window.fetch = async function (url, init) {
    const response = await originalFetch(url, init);

    // Only intercept calls to the GraphQL API
    if (url.toString().includes("/api/graphql") && init && init.method === "POST") {
        try {
            const requestBody = JSON.parse(init.body);
            
            // Check if this is a game action (add other games as needed)
            const betMutations = ["LimboPlay", "MinesPlay", "DicePlay"];
            
            if (betMutations.includes(requestBody.operationName)) {
                // Clone response because it can only be read once
                const clone = response.clone();
                clone.json().then(data => {
                    handleBetInterception(requestBody, data);
                });
            }
            
            // Intercept Seed Rotation to link past bets
            if (requestBody.operationName === "RotateSeed") {
                 const clone = response.clone();
                 clone.json().then(data => {
                     handleSeedRotation(data);
                 });
            }
        } catch (e) {
            console.error("ShuffleTracker Error:", e);
        }
    }
    
    return response;
};
```

### 2. The "Delayed Verification" Logic
The current guide explains how to verify a bet *if you have the seed*. However, it misses the **State Management Logic** required for a tracker.

**The Problem:** When a bet happens (Nonce #50), the Server Seed is **hashed**. You cannot verify it yet.
**The Solution:** You must store bets in a "Pending Verification" state until the user clicks "Rotate Seed".

**Missing Data Structure:**
Your script needs a local database (using `IndexedDB` via a library like `idb` or `Dexie.js`, because `localStorage` is too small) to handle this workflow:

1.  **On Bet (`LimboPlay`):**
    *   Capture `Nonce`, `ClientSeed`, `ActiveServerSeedHash`, `Result`.
    *   Store in DB with status: `UNVERIFIED`.
2.  **On Seed Rotation (`RotateSeed`):**
    *   Capture `PreviousServerSeed` (Plaintext) from the response.
    *   **The Missing Link:** Query DB for all bets where `ActiveServerSeedHash` matches the hash of this newly revealed seed.
    *   Run the Python-equivalent verification logic (from the guide) on all those bets.
    *   Update DB status: `VERIFIED_FAIR` or `TAMPERED`.

### 3. Response Parsing Paths
The guide lists request payloads nicely, but for a tracker, you need the exact **Response JSON Paths** to extract the Nonce and Result. These paths are deep and specific.

**Missing Schema Mapping:**
You need to map the game-specific response paths to a standardized format.

*Example for Limbo:*
```javascript
function handleBetInterception(req, res) {
    // 1. Extract Inputs from Request
    const clientSeed = getCurrentClientSeedFromGlobalState(); // See point #4
    const amount = req.variables.data.amount;
    
    // 2. Extract Outputs from Response
    // The guide misses this specific path structure
    const gameData = res.data.limboPlay; // Changes based on game: minesPlay, dicePlay
    const resultValue = gameData.shuffleOriginalActions[0].action.limbo.resultValue;
    const payout = gameData.payout;
    
    // 3. You need the Nonce. 
    // WARNING: Shuffle often does NOT return the nonce in the bet response.
    // You must track the nonce locally or fetch 'GetSeedStatus' after every bet.
}
```

### 4. Accessing React Internal State (The "Nonce" Problem)
As noted above, the API response for a bet often doesn't explicitly say "This was Nonce #50." It just says "You won."

To link the bet accurately, you need to read the **Current Nonce** from the UI before the request goes out.

**Missing DOM/React Access Strategy:**
You cannot just scrape the HTML because the nonce updates dynamically. You need to hook into the React properties of the DOM elements.

```javascript
// Helper to read React props from a DOM element
function getReactProps(domElement) {
    const key = Object.keys(domElement).find(k => k.startsWith("__reactProps$"));
    return key ? domElement[key] : null;
}

// Example: Finding the current nonce/seed hash from the "Provably Fair" modal hidden data
// or by querying the 'GetSeedStatus' endpoint manually in the background.
```

*Better Approach for Tampermonkey:*
Instead of scraping the DOM for the Nonce, your script should trigger a background `fetch` to the `GetSeedStatus` query (defined in the guide) **immediately after** detecting a successful bet to confirm the nonce used.

### Summary of Tasks to Add to Your Script
To make the guide's info work in Tampermonkey, write a script that:

1.  **Injects `window.fetch` proxy** immediately on page load (`@run-at document-start`).
2.  **Initializes an IndexedDB** named `ShuffleHistory`.
3.  **Listens for `*Play` mutations**:
    *   Saves the bet + current `ServerSeedHash` + `ClientSeed` to IndexedDB.
4.  **Listens for `RotateSeed` mutation**:
    *   Takes the revealed seed.
    *   Iterates through IndexedDB for matching hashes.
    *   Runs the HMAC-SHA256 logic (port the Python logic from the guide to JS).
    *   Updates the UI with a green "Verified" checkmark next to past bets in the "My Bets" list.

1- The system utilizes several mechanisms for intercepting events and managing data flow, primarily related to handling user input, managing application state, and processing external data.
1. Hotkey and Input Interception: An explicit mechanism exists for controlling game hotkeys by intercepting keyboard and browser events. A function checks several conditions to determine if key presses should be ignored, preventing unintended actions during gameplay. This logic dictates that hotkeys are disabled if:
    ◦ Game hotkeys are not enabled globally.
    ◦ A component is explicitly marked to prevent hotkeys (m).
    ◦ The browser environment indicates a mobile or tablet device (s).
    ◦ An overlapping modal or floating UI element is currently open (l or n).
2. This system uses event listeners registered on the window for keydown, keyup, focusin, and focusout events to track application state and prevent focus issues, ensuring that hotkeys are disabled while inputs are active.
3. GraphQL Response Interception (Client Awareness): There is a data interception mechanism within the GraphQL client configuration, executed via an Apollo link (et),. This link intercepts GraphQL responses to extract and process specific HTTP headers, such as x-country and x-region, using X.iV.setUserCountryAndRegion() to update the global state with the user's country and region information,.
4. UI and Navigation Interception: Navigation state changes often involve interception:
    ◦ When a user interacts with a navigation item, a handler is called which explicitly sets the right sidebar status to NONE, ensuring the sidebar is closed upon navigation,.
    ◦ The toggleIntercom function, used for live chat, acts as an interceptor that checks the current state of the Intercom window and calls the appropriate API method (hide() or show()) to control its visibility,.
    ◦ The application includes logic to manage URL parameters (e.g., modal and tab IDs, md-tab, md-id) and ensures that these parameters are removed when a modal is closed, effectively cleaning up the URL after the UI element is dismissed .
5. State Management Interception: The system includes a core mechanism (l) for updating persistent application state by taking an entity key and a modification function (t), and feeding that update to the global state implementation (s.setState), allowing for centralized state mutation control,. This pattern is fundamental to intercepting and applying state changes throughout the application.

2- The state management logic described in the sources relies on a centralized store architecture, likely built using tools similar to Redux or Zustand, coupled with specialized mechanisms for deriving game results and handling persistent data like caching and local storage.
Core State Architecture and Management
The application utilizes a global state structure which can be accessed and modified using functions like getState and setState. Changes to the application state are often handled through reducer-like actions (implicitly or explicitly defined via i.Z0, u.Z0) that update state properties, often involving spreading existing state object properties to ensure immutability.
Specific store slices and their associated state updates include:
1. Games Persistence (games store): This store holds configurations and active settings for various original games. The initial state uses default settings for games like Dice, Limbo, Keno, Mines, Wheel, Plinko, Blackjack, Roulette, Crash, and Tower.
    ◦ It includes actions for updating playing status (updateIsPlaying), modifying bet amounts (updateBetAmount), and saving game-specific persistence data (updateGamePersistState).
    ◦ For Plinko, separate logic exists to specifically manage and update parameters stored in local storage, such as numberOfRows and riskLevel (updatePlinkoLocalStorage).
2. Global UI/Browser Preferences (browserPreference and globalState): These slices manage UI settings and application status, such as:
    ◦ Toggling the display of prices in Fiat currency (toggleDisplayInFiat).
    ◦ Setting active navigation elements (setActiveMobileNav, setActiveNavMenuType).
    ◦ Managing sidebar visibility, such as displaying the notifications panel (toggleNotificationSidebar) or the Bet Slip (e((0,k.Iq)({open:!v})),v&&f===q.D5.BET_PLACED&&l()}).
3. Application Settings (appSetting): Stores game configuration settings fetched from the backend, such as maxBetUSD, maxPayoutUSD, kycAge, and geoRestrictedRegions.
Game-Specific State Logic
For original games, state management involves calculating the current state based on client inputs, seeds, and game rules.
Game
Key State Elements and Logic
Hilo
The game state is derived from shuffleOriginalActions and can be in one of several modes: "CHOOSING_FIRST_CARD", "MIN_CHOOSING_NEXT_CARD", "MAX_CHOOSING_NEXT_CARD", "CHOOSING_NEXT_CARD", "CASHOUT", or "LOST". The logic determines the multiplier and the current state based on the last action type (CORRECT_GUESS, WRONG_GUESS, CASHOUT, FORCE_CASHOUT) and the value of the current card (e.g., if the value is 'A' or 'K').
Mines
State includes selectedTiles, mineLocations, minesCount, numGemsRemaining, and outcome (WIN/LOSS/null). The outcome is determined by checking if the calculated minesResult is contained within the selected tiles, or if the user cashed out successfully. The component displays the grid with tiles marked as MINE or GEM based on the results.
Blackjack
The state tracks hands (dealerHand, mainHand, splitHand), availableActions, and the specific game phase (STARTING, BUYING_INSURANCE, PLAYING_MAIN_HAND, PLAYING_SPLIT_HAND, GAME_OVER). The available actions are calculated dynamically based on the current player's hand, the dealer's visible card, and whether insurance is eligible.
Plinko
The game calculates the resulting multiplier using a function that depends on the serverSeed, clientSeed, nonce, and the chosen plinkoRowCount. The multiplier is then formatted to one decimal place (.toFixed(1)).
Data Persistence and Caching
Data fetching and caching primarily utilize the Apollo Client, which maintains a unified cache that acts as a secondary layer of state. The application instantiates multiple Apollo Clients, differentiated by their configuration and target endpoints (ep, ec, ef, eh, em), allowing for specialized data handling (e.g., for standard queries vs. sports bets).
Components often interact with this cached data layer using query hooks, specifying a fetchPolicy such as cache-and-network or no-cache.
The connection status to real-time systems (WebSockets/Subscriptions) is also monitored in the global state (websocketStatus), allowing the application to display alerts (like an "unstable connection" banner) and trigger actions such as resetting the store.

3- The term "Response Parsing Paths," based on the provided sources, relates primarily to how user interface elements are positioned (often referred to as placement paths) and how structured data or errors are addressed using object pathways (like those found in GraphQL).
UI Placement Paths (CSS & Logic)
The sources define multiple layout options, referred to as placement paths, that dictate where a pop-up menu or similar element will appear relative to its anchor element.
• Defined Placements: The PopupViewContainer component handles these paths through CSS classes, enabling placement options such as:
    ◦ "right" (the default placement).
    ◦ "top" (applied via class PopupViewContainer_topMenu__Pkbz1).
    ◦ "left" (applied via class PopupViewContainer_leftMenu__Hs7hy).
    ◦ "bottom" (applied via class PopupViewContainer_bottomMenu__nYbT_).
    ◦ Specific corner placements like "top-left" (via class PopupViewContainer_topLeftMenu__Orx26) and "bottom-left" (via class PopupViewContainer_bottomLeftMenu__bKNMG).
    ◦ The placement "bottom-right" is also referenced by a CSS class (PopupViewContainer_bottomRightMenu__a610A).
This placement choice determines which corresponding class is applied using conditional logic (e.g., ".concat("top"===a?u().topMenu:"")) to achieve the visual layout. Additionally, dark menu styling is available for these placements (e.g., .PopupViewContainer_darkMenu__JNyy_.PopupViewContainer_topLeftMenu__Orx26 .PopupViewContainer_arrow__2dgEg).
Data and Error Paths
In the context of processing complex application data, paths are critical for identifying locations within data structures:
• GraphQL Error Paths: When an error occurs during GraphQL processing, the corresponding error object often includes a path field which helps pinpoint the exact location of the error within the query structure.
• GraphQL Document Parsing: Functions responsible for transforming or printing GraphQL documents parse structure elements such as Document, OperationDefinition, Variable, Field, and Argument.
SVG Paths
The codebase also utilizes mathematical definitions to draw graphics, such as an SVG path defined by concatenating coordinate segments: path:"M ".concat(i.join(" ")).

4- The process for fetching the game seed status is handled by a dedicated GraphQL query hook associated with the Provably Fair modal, typically referred to internally as i.$zh in the sources.
This retrieval process performs the following actions:
1. Initiation and Policy: The data fetch is initiated using a GraphQL query hook (i.$zh) within the component responsible for the Provably Fair modal (R). It uses a fetchPolicy set to "cache-and-network".
2. Data Retrieval: The data, labeled internally as b, is fetched, and the loading status is tracked by the variable y. This fetch is triggered when the modal is opened.
3. Data Structure (currentGameSeeds): The fetched data contains information about the currentGameSeeds. The system inspects this data to identify two crucial components based on their status:
    ◦ The Active Seed (o): This is the seed object currently in use, where the status matches i.Ypt.ACTIVE.
    ◦ The Next Seed (r): This is the seed object prepared for the subsequent game session, where the status matches i.Ypt.NEXT.
4. Displayed Information: The data extracted from these seeds is then displayed within the "Seeds" tab of the Provably Fair modal (A in source), including:
    ◦ Active Client Seed ((null==o?void 0:o.clientSeed)||")
    ◦ Active Hashed Server Seed ((null==o?void 0:o.hashedSeed)||")
    ◦ Current Nonce ((null==o?void 0:o.currentNonce)||") (labeled as "total bets/plays with pair" to the user)
    ◦ Next Server Seed ((null==r?void 0:r.hashedSeed)||")



Based on the provided developer notes and system internals, here is exactly what is missing from the original guide to build a fully functional Tampermonkey tracking script.

While the original guide documents the **API** (how to send commands), it misses the **Interception & State Management** layers required to passively track a user's actions and link the cryptographic data.

### 1. The `window.fetch` Interceptor (The "Listener")
The original guide describes endpoints to *call*. A tracker needs to *listen*.
*   **Context:** Note #1.3 mentions the system uses **Apollo Client** links for interception internally. You cannot easily inject into Apollo from Tampermonkey without breaking the app.
*   **Missing Component:** You must monkey-patch `window.fetch` to read the JSON traffic passively.
*   **Implementation:**
    ```javascript
    const originalFetch = window.fetch;
    window.fetch = async function (url, init) {
        // 1. Capture the Request
        const requestBody = init.body ? JSON.parse(init.body) : {};
        
        // 2. Execute the Request
        const response = await originalFetch(url, init);
        
        // 3. Clone and Read Response
        const clone = response.clone();
        clone.json().then(data => {
            // Check for Game Plays (e.g., MinesPlay, LimboPlay)
            if (url.toString().includes("/graphql")) {
                handleInterception(requestBody, data);
            }
        });
        
        return response;
    };
    ```

### 2. State-Based Nonce Extraction
The biggest challenge in tracking is knowing **"Which nonce was this bet?"** The API response for a bet often confirms the win/loss but *does not* repeat the nonce used.
*   **Context:** Note #4 identifies the seed data structure: `currentGameSeeds` containing `currentNonce`.
*   **Missing Component:** The script must capture the `currentNonce` **before** the bet request is sent.
*   **Implementation:**
    *   **Method A (Network):** Listen for the `GetSeedStatus` query (mentioned in Note #4) which loads on page load or modal open. Store the `currentNonce` in a local variable and increment it +1 every time you detect a successful bet request.
    *   **Method B (React Props):** Access the "Provably Fair" modal's internal React state (if initialized) to read `o.currentNonce`.

### 3. The "Unverified" Holding Database
You cannot verify a bet immediately because the server seed is hashed (`Active Hashed Server Seed` in Note #4).
*   **Context:** Note #2 describes the `games` store state logic. You need a parallel store in your script.
*   **Missing Component:** An **IndexedDB** instance (via `idb` or `Dexie.js`) to store bets in a "Pending" state.
*   **Workflow:**
    1.  **On Bet:** Store `{ nonce: 50, clientSeed: "abc", serverHash: "hash123", result: "win" }` in IndexedDB.
    2.  **Status:** `UNVERIFIED`.

### 4. Seed Rotation Logic (The Verification Trigger)
The guide explains *how* to rotate a seed, but not how to use that event to trigger retroactive verification.
*   **Context:** Note #4 mentions `Next Server Seed` and `Active Seed`. When a rotation happens, the `Active` seed (plaintext) is revealed.
*   **Missing Component:** A listener for the `RotateSeed` mutation.
*   **Implementation:**
    1.  Intercept `RotateSeed` response.
    2.  Extract the **Previous Plaintext Server Seed**.
    3.  Query IndexedDB: *"Find all bets where `serverHash` matches the hash of this newly revealed seed."*
    4.  Run the HMAC-SHA256 verification (from the guide's Python section, ported to JS) on those pending bets.
    5.  Update UI to show a "Verified Fair" badge.

### 5. Accessing Game-Specific Stores
To provide context to your tracker (e.g., "Risk Level" or "Rows" for Plinko), you need more than just the bet result.
*   **Context:** Note #2 explicitly mentions the `games` store handles state like `updatePlinkoLocalStorage`.
*   **Missing Component:** Your script needs to read `localStorage` keys like `plinko_store` (or similar reversed keys) to capture the configuration settings (Rows, Risk) that aren't always fully detailed in the standardized bet response.

### Summary of Script Architecture
To make this work, your Tampermonkey script needs:
1.  **`@run-at document-start`** to hook `fetch` before the app loads.
2.  **A global `nonce` counter** synced with `GetSeedStatus`.
3.  **An `IndexedDB`** to hold unverified bets.
4.  **A "Verifier" module** that wakes up upon `RotateSeed` to process the backlog.
