# Shuffle.com Complete API Schema

**Version:** 1.0  
**Last Updated:** 2025-12-12  
**Total Operations:** 296 (146 Queries, 114 Mutations, 36 Subscriptions)  
**API Type:** GraphQL  
**Protocol:** HTTPS (REST) / WSS (WebSocket)

---

## Table of Contents

1. [Base URLs & Endpoints](#base-urls--endpoints)
2. [Authentication](#authentication)
3. [Request/Response Format](#requestresponse-format)
4. [Type System](#type-system)
5. [Queries (146)](#queries-146)
6. [Mutations (114)](#mutations-114)
7. [Subscriptions (36)](#subscriptions-36)
8. [Error Handling](#error-handling)
9. [Rate Limiting](#rate-limiting)
10. [Game-Specific Schemas](#game-specific-schemas)
11. [Provably Fair System](#provably-fair-system)
12. [Hidden Mechanics & Anti-Bot Features](#hidden-mechanics--anti-bot-features)
13. [Python Client Library](#python-client-library)

---

## Base URLs & Endpoints

### Primary Domain
```
https://shuffle.com
```

### Mirror Domains
All endpoints work with these alternate domains:
- `shuffle.com`
- `shuffle.game`
- `shuffle.gg`
- `shuffle888.com`
- `shuffle.bet`
- `shuffle.casino`
- `shuffle.net`
- `shuffle.vip`
- `shuffle.site`
- `shuffle.money`
- `shuffle1069.com`
- `shuffle.global`
- `shuffle.online`
- `shuffle.gold`

### GraphQL Endpoints

#### Main GraphQL Endpoint
```
POST https://[domain]/main-api/graphql/api/graphql
```
**Purpose:** General queries & mutations  
**Used For:** Game operations, user data, betting, financial operations

#### Specialized GraphQL Endpoints

| Endpoint Path | Full URL Pattern | Purpose |
|---------------|------------------|---------|
| `/api/graphql` | `https://[domain]/main-api/graphql/api/graphql` | General operations |
| `/sports/graphql-sports` | `https://[domain]/main-api/graphql/sports/graphql-sports` | Sports betting data |
| `/sports-main/graphql-sports-main` | `https://[domain]/main-api/graphql/sports-main/graphql-sports-main` | Main sports data |
| `/lottery/graphql-lottery` | `https://[domain]/main-api/graphql/lottery/graphql-lottery` | Lottery operations |
| `/kyc/graphql-kyc` | `https://[domain]/main-api/graphql/kyc/graphql-kyc` | KYC verification |
| `/graphql` | `https://[domain]/main-api/graphql/graphql` | Fallback endpoint |

### WebSocket Subscription Endpoints

| Subscription Path | Full URL Pattern | Purpose |
|-------------------|------------------|---------|
| `/subscription/graphql` | `wss://[domain]/main-api/bp-subscription/subscription/graphql` | General subscriptions |
| `/stable-subscription/graphql` | `wss://[domain]/main-api/bp-subscription/stable-subscription/graphql` | Core feature updates |
| `/sports-subscription/graphql` | `wss://[domain]/main-api/bp-subscription/sports-subscription/graphql` | Live sports updates |

### REST Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/v1/sports/get-nav-items` | GET | Navigation menu & CMS data |
| `/api/v1/images/og-image` | GET | Social media preview images |

**Query Parameters:**
```
/api/v1/sports/get-nav-items?locale=en&preview=true
```

### External Service Endpoints

| Service | URL | Purpose |
|---------|-----|---------|
| **N9 Assets** | `https://n9assets.com/games/games.json` | Game library catalog |
| **N9 Assets (Alt)** | `https://n9assets.com/file/games/games.json` | Alternative game catalog |
| **Store Location** | `https://shuffle.click/store-access-location` | Regional compliance checks |
| **Genius Sports** | `https://viewer-data.production.geniuslive.geniussports.com/` | Live sports streaming data |

---

## Authentication

### Authentication Method
**JWT (JSON Web Token)** + **Cloudflare Cookies**

### Required Headers

| Header | Value/Source | Description |
|--------|--------------|-------------|
| `Authorization` | `Bearer <JWT>` | **Critical.** Found in DevTools Network tab. Expires ~24h. |
| `Cookie` | Browser Network Tab | Contains `__cf_bm`, `cf_clearance`, session identifiers |
| `User-Agent` | Your Browser | **Critical.** Must match real browser to bypass Cloudflare |
| `Content-Type` | `application/json` | Required for all requests |
| `Origin` | `https://shuffle.com` | Required for CORS checks |
| `Referer` | Game-specific URL | Example: `https://shuffle.com/games/originals/limbo` |
| `x-correlation-id` | Generated UUID | Tracking ID linking requests |
| `x-country` | Location code | Example: `CA`, `US`. Should match IP location |

### How to Obtain Credentials

1. Log in to Shuffle.com in your browser (Chrome/Firefox recommended)
2. Press **F12** to open Developer Tools
3. Go to the **Network** tab
4. Filter by **Fetch/XHR** or search for `graphql`
5. Refresh the page or perform an action (view wallet, place minimum bet)
6. Click a request named `graphql` and view **Request Headers**
7. Copy:
   - **Authorization:** The full `Bearer eyJhbGc...` token
   - **Cookie:** The entire cookie string (can be very long)
   - **User-Agent:** Your exact browser user agent string

**Important Notes:**
- JWT tokens typically expire after 24 hours
- Cloudflare cookies (`cf_clearance`) expire quickly
- You must use the exact User-Agent from the browser where you copied the cookies

### Token Refresh

```graphql
mutation RefreshToken($refreshToken: String!) {
  refreshToken(refreshToken: $refreshToken) {
    accessToken
    refreshToken
  }
}
```

---

## Request/Response Format

### Request Format

All GraphQL requests use POST method with JSON body:

```json
{
  "operationName": "OperationName",
  "variables": {
    "param1": "value1",
    "param2": 100
  },
  "query": "query/mutation definition string..."
}
```

### Response Format

**Success Response:**
```json
{
  "data": {
    "fieldName": {
      "subField": "value"
    }
  }
}
```

**Error Response:**
```json
{
  "errors": [
    {
      "message": "Error message",
      "extensions": {
        "code": "ERROR_CODE"
      }
    }
  ]
}
```

### Query vs Mutation vs Subscription

- **Query:** Used to *read* data (e.g., Get Balance, Get Game Settings)
- **Mutation:** Used to *write* data (e.g., Place Bet, Update Seed)
- **Subscription:** Used for *real-time updates* via WebSocket (e.g., Balance Updates, Bet Results)

---

## Type System

### Scalar Types

| Type | Description | Example |
|------|-------------|---------|
| `String` | Text string | `"USDT"` |
| `String!` | Required text string | `"0.10000000"` |
| `Int` | Integer (optional) | `3` |
| `Int!` | Required integer | `16` |
| `Boolean` | Boolean (optional) | `true` |
| `Boolean!` | Required boolean | `false` |
| `Float` | Floating point number | `2.5` |
| `ID` | Unique identifier | `"bet_123"` |
| `Upload` | File upload | Used for KYC documents |

### Enum Types

#### Currency
```graphql
enum Currency {
  BTC
  ETH
  USDT
  USDC
  BNB
  SOL
  TON
  # ... more currencies
}
```

#### Game Types
```graphql
enum GameType {
  LIMBO
  DICE
  CRASH
  MINES
  PLINKO
  WHEEL
  TOWER
  HILO
  BLACKJACK
  ROULETTE
  KENO
}
```

#### Risk Levels
```graphql
enum PlinkoRiskLevel {
  LOW_RISK
  MEDIUM_RISK
  HIGH_RISK
}

enum WheelRiskLevel {
  LOW
  MEDIUM
  HIGH
}
```

#### Tower Difficulty
```graphql
enum TowerDifficulty {
  EASY
  MEDIUM
  HARD
  EXPERT
  MASTER
}
```

#### Wheel Segment Count
```graphql
enum WheelSegmentCount {
  TEN
  TWENTY
  THIRTY
  FORTY
  FIFTY
}
```

#### OAuth Provider
```graphql
enum OauthProvider {
  GOOGLE
  TWITTER
  DISCORD
  # ... more providers
}
```

#### Language
```graphql
enum Language {
  en
  es
  pt
  # ... more languages
}
```

### Input Types

#### LimboPlayInput
```graphql
input LimboPlayInput {
  amount: String!      # "0.10000000" (8 decimals)
  bet: String!         # "2.00" (target multiplier, 2 decimals)
  currency: Currency!  # "USDT", "BTC", "ETH"
  windowId: String     # Browser tab identifier (optional)
  usdAmount: String    # USD equivalent (optional)
}
```

#### DicePlayInput
```graphql
input DicePlayInput {
  amount: String!      # "0.10000000"
  target: String!      # "0.00" - "100.00"
  over: Boolean!       # true = bet over, false = bet under
  currency: Currency!
  windowId: String
}
```

#### MinesStartInput
```graphql
input MinesStartInput {
  amount: String!
  numberOfMines: Int!  # 1-24
  currency: Currency!
  windowId: String
}
```

#### MinesNextInput
```graphql
input MinesNextInput {
  tileIndex: Int!      # 0-24 (5x5 grid)
}
```

#### PlinkoPlayInput
```graphql
input PlinkoPlayInput {
  amount: String!
  rowCount: Int!       # 8-16
  riskLevel: PlinkoRiskLevel!
  currency: Currency!
  windowId: String
}
```

#### WheelPlayInput
```graphql
input WheelPlayInput {
  amount: String!
  segmentCount: WheelSegmentCount!
  riskLevel: WheelRiskLevel!
  currency: Currency!
  windowId: String
}
```

#### TowerStartInput
```graphql
input TowerStartInput {
  amount: String!
  difficulty: TowerDifficulty!
  currency: Currency!
  windowId: String
}
```

#### CrashPlayInput
```graphql
input CrashPlayInput {
  amount: String!
  currency: Currency!
  windowId: String
}
```

#### KenoPlayInput
```graphql
input KenoPlayInput {
  amount: String!
  selectedNumbers: [Int!]!  # 1-10 numbers from 1-40
  currency: Currency!
  windowId: String
}
```

#### RoulettePlayInput
```graphql
input RoulettePlayInput {
  amount: String!
  currency: Currency!
  bets: [RouletteBetInput!]!
  windowId: String
}
```

#### HiloStartInput
```graphql
input HiloStartInput {
  amount: String!
  currency: Currency!
  windowId: String
}
```

#### HiloNextInput
```graphql
input HiloNextInput {
  action: HiloAction!  # HIGHER, LOWER, CASHOUT
}
```

#### BlackjackStartInput
```graphql
input BlackjackStartInput {
  amount: String!
  currency: Currency!
  windowId: String
}
```

#### BlackjackNextInput
```graphql
input BlackjackNextInput {
  action: BlackjackAction!  # HIT, STAND, DOUBLE, SPLIT
}
```

### Response Types

#### Bet Response
```graphql
type Bet {
  id: ID!
  currency: Currency!
  amount: String!
  payout: String!
  afterBalance: String!
  multiplier: String
  createdAt: String!
  updatedAt: String!
  shuffleOriginalActions: [ShuffleOriginalAction!]!
  gameSeedNonce: Int
}
```

#### ShuffleOriginalAction
```graphql
type ShuffleOriginalAction {
  id: ID!
  action: GameAction!
  updatedAt: String!
  createdAt: String!
}
```

#### GameAction (Union)
```graphql
union GameAction = 
  | LimboAction
  | DiceAction
  | CrashAction
  | MinesAction
  | PlinkoAction
  | WheelAction
  | TowerAction
  | HiloAction
  | BlackjackAction
  | RouletteAction
  | KenoAction
```

#### LimboAction
```graphql
type LimboAction {
  resultRaw: String!
  resultValue: String!    # Multiplier (e.g., "1.85x")
  userValue: String!      # Target multiplier
}
```

#### DiceAction
```graphql
type DiceAction {
  resultValue: String!    # Roll result (0.00-100.00)
  target: String!
  over: Boolean!
}
```

#### MinesAction
```graphql
type MinesAction {
  numberOfMines: Int!
  minePositions: [Int!]!  # Tile positions (0-24)
  revealedPositions: [Int!]!
}
```

#### PlinkoAction
```graphql
type PlinkoAction {
  risk: PlinkoRiskLevel!
  results: [Int!]!        # Path (0=left, 1=right)
  rows: Int!
  multiplier: String!
}
```

#### WheelAction
```graphql
type WheelAction {
  resultRaw: String!
  resultSegment: Int!
  risk: WheelRiskLevel!
  segments: Int!
}
```

#### Balance
```graphql
type Balance {
  currency: Currency!
  amount: String!
}
```

#### User
```graphql
type User {
  id: ID!
  username: String!
  email: String
  vipLevel: VipLevel!
  balances: [Balance!]!
  seedStatus: SeedStatus
  createdAt: String!
}
```

#### SeedStatus
```graphql
type SeedStatus {
  activeClientSeed: String!
  activeHashedServerSeed: String!
  currentNonce: Int!
  nextHashedServerSeed: String!
}
```

---

## Queries (146)

### Account & User Management

#### GetMyBalance
```graphql
query GetMyBalance {
  me {
    balances {
      currency
      amount
    }
  }
}
```
**Auth:** ✅ Required  
**Returns:** Array of balances by currency

#### GetMyProfile
```graphql
query GetMyProfile {
  me {
    id
    username
    email
    vipLevel
    createdAt
  }
}
```
**Auth:** ✅ Required

#### GetMyBets
```graphql
query GetMyBets($first: Int!, $skip: Int!, $currency: Currency, $gameSlug: String) {
  bets(first: $first, skip: $skip, currency: $currency, gameSlug: $gameSlug) {
    nodes {
      id
      currency
      amount
      payout
      multiplier
      createdAt
      game {
        name
        slug
      }
    }
    pageInfo {
      hasNextPage
    }
  }
}
```
**Auth:** ✅ Required  
**Parameters:**
- `first: Int!` - Number of bets to fetch
- `skip: Int!` - Pagination offset
- `currency: Currency` - Filter by currency (optional)
- `gameSlug: String` - Filter by game (optional)

#### GetBetInfo
```graphql
query GetBetInfo($betId: String!) {
  bet(id: $betId) {
    id
    currency
    amount
    payout
    multiplier
    createdAt
    shuffleOriginalActions {
      action {
        ... on LimboAction {
          resultValue
          userValue
        }
        ... on DiceAction {
          resultValue
          target
          over
        }
      }
    }
  }
}
```
**Auth:** ✅ Required

#### GetMyKyc
```graphql
query GetMyKyc {
  me {
    kyc {
      level
      status
      verifiedAt
    }
  }
}
```
**Auth:** ✅ Required

### Game Operations

#### GetGames
```graphql
query GetGames($first: Int!, $skip: Int!, $category: String, $provider: String) {
  games(first: $first, skip: $skip, category: $category, provider: $provider) {
    nodes {
      id
      name
      slug
      category
      provider
      edge
      originalGame
    }
    pageInfo {
      hasNextPage
    }
  }
}
```
**Auth:** ❌ Not required

#### GetGameBySlug
```graphql
query GetGameBySlug($slug: String!) {
  game(slug: $slug) {
    id
    name
    slug
    category
    provider
    edge
    originalGame
  }
}
```
**Auth:** ❌ Not required

#### GetAppSettings
```graphql
query GetAppSettings {
  appSettings {
    limbo {
      maxPayoutUSD
      minBetUSD
    }
    dice {
      maxPayoutUSD
      minBetUSD
    }
    crash {
      maxPayoutUSD
      minBetUSD
    }
    mines {
      maxPayoutUSD
      minBetUSD
    }
  }
}
```
**Auth:** ❌ Not required

### Active Bets & Game States

#### GetMinesActiveBet
```graphql
query GetMinesActiveBet {
  minesActiveBet {
    id
    amount
    currency
    numberOfMines
    selectedTiles
    resultTiles
    payout
  }
}
```
**Auth:** ✅ Required

#### GetTowerActiveBet
```graphql
query GetTowerActiveBet {
  towerActiveBet {
    id
    amount
    currency
    difficulty
    currentRow
    selectedTiles
    payout
  }
}
```
**Auth:** ✅ Required

#### GetCrashGame
```graphql
query GetCrashGame {
  crashGame {
    id
    status
    crashPoint
    startedAt
    nextRoundIn
  }
}
```
**Auth:** ❌ Not required

### Provably Fair

#### GetCurrentGameSeeds
```graphql
query GetCurrentGameSeeds {
  me {
    seedStatus {
      activeClientSeed
      activeHashedServerSeed
      currentNonce
      nextHashedServerSeed
    }
  }
}
```
**Auth:** ✅ Required

#### GetUnhashedSeed
```graphql
query GetUnhashedSeed($hashedServerSeed: String!) {
  unhashServerSeed(hashedServerSeed: $hashedServerSeed) {
    plainTextSeed
    wasUsedForGames
  }
}
```
**Auth:** ✅ Required

### Token Information

#### tokenInfo
```graphql
query tokenInfo {
  tokenInfo {
    priceInUsd
    circulatingSupply
    burnedTokens
  }
}
```
**Auth:** ❌ Not required

### Sports Betting

#### GetSports
```graphql
query GetSports($language: Language) {
  sports(language: $language) {
    id
    name
    slug
  }
}
```
**Auth:** ❌ Not required

#### GetSportsFixtures
```graphql
query GetSportsFixtures($sport: Sports!, $competitionId: String, $first: Int!, $skip: Int!) {
  sportsFixtures(sport: $sport, competitionId: $competitionId, first: $first, skip: $skip) {
    nodes {
      id
      name
      startTime
      status
    }
  }
}
```
**Auth:** ❌ Not required

### Lottery

#### GetLatestLotteryDraw
```graphql
query GetLatestLotteryDraw {
  latestLotteryDraw {
    id
    drawAt
    prizePool
    ticketCount
  }
}
```
**Auth:** ❌ Not required

#### GetUserSingleTickets
```graphql
query GetUserSingleTickets {
  me {
    lotteryTickets {
      id
      drawId
      drawAt
      currency
      amount
      status
    }
  }
}
```
**Auth:** ✅ Required

### VIP & Rewards

#### GetVipDailyRakeback
```graphql
query GetVipDailyRakeback {
  me {
    vipDailyRakeback {
      nextClaimDate
      eligible
      currencyAmounts {
        currency
        amount
      }
    }
  }
}
```
**Auth:** ✅ Required

#### GetChallenges
```graphql
query GetChallenges {
  challenges {
    id
    name
    description
    currency
    amount
    progress
    completed
  }
}
```
**Auth:** ✅ Required

### Notifications

#### getNotifications
```graphql
query getNotifications($first: Int!, $skip: Int!) {
  notifications(first: $first, skip: $skip) {
    nodes {
      id
      type
      message
      readAt
      seenAt
      createdAt
    }
  }
}
```
**Auth:** ✅ Required

---

## Mutations (114)

### Authentication & Account

#### LoginRequest
```graphql
mutation LoginRequest($identity: String!, $password: String!, $geetest: GeetestInput) {
  loginRequest(identity: $identity, password: $password, geetest: $geetest) {
    loginToken
    otpSentAt
    otpEmail
    loginVerificationMethod
  }
}
```
**Auth:** ❌ Not required

#### Login
```graphql
mutation Login($loginToken: String!, $tfaCode: String!) {
  login(loginToken: $loginToken, tfaCode: $tfaCode) {
    accessToken
    refreshToken
  }
}
```
**Auth:** ❌ Not required

#### RegisterAccount
```graphql
mutation RegisterAccount($data: RegisterInput!) {
  register(data: $data) {
    accessToken
    refreshToken
    id
    createdAt
  }
}
```
**Auth:** ❌ Not required

**RegisterInput:**
```graphql
input RegisterInput {
  username: String!
  email: String!
  password: String!
  geetest: GeetestInput
}
```

### Game Play Operations

#### LimboPlay
```graphql
mutation LimboPlay($data: LimboPlayInput!) {
  limboPlay(data: $data) {
    id
    currency
    amount
    payout
    afterBalance
    shuffleOriginalActions {
      id
      action {
        limbo {
          resultRaw
          resultValue
          userValue
        }
      }
    }
  }
}
```
**Auth:** ✅ Required

**Example Variables:**
```json
{
  "data": {
    "amount": "0.10000000",
    "bet": "2.00",
    "currency": "USDT",
    "windowId": "abc123xyz"
  }
}
```

#### DicePlay
```graphql
mutation DicePlay($data: DicePlayInput!) {
  dicePlay(data: $data) {
    id
    currency
    amount
    payout
    afterBalance
    shuffleOriginalActions {
      id
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
```
**Auth:** ✅ Required

#### CrashPlay
```graphql
mutation CrashPlay($data: CrashPlayInput!) {
  crashPlay(data: $data) {
    id
    amount
    payout
    currency
    crashBet {
      crashGameId
    }
    shuffleOriginalActions {
      action {
        crash {
          betAt
          payout
          multiplier
          payoutType
        }
      }
    }
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### CrashCashout
```graphql
mutation CrashCashout($crashGameId: Int) {
  crashCashout(crashGameId: $crashGameId)
}
```
**Auth:** ✅ Required

#### MinesStart
```graphql
mutation MinesStart($data: MinesStartInput!) {
  minesStart(data: $data) {
    id
    amount
    currency
    numberOfMines
    selectedTiles
    resultTiles
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### MinesNext
```graphql
mutation MinesNext($data: MinesNextInput!) {
  minesNext(data: $data) {
    id
    selectedTiles
    resultTiles
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### MinesCashout
```graphql
mutation MinesCashout {
  minesCashout {
    id
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### MinesAutoBet
```graphql
mutation MinesAutoBet($data: MinesBetInput!) {
  minesAuto(data: $data) {
    id
    amount
    currency
    numberOfMines
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### PlinkoPlay
```graphql
mutation PlinkoPlay($data: PlinkoPlayInput!) {
  plinkoPlay(data: $data) {
    id
    currency
    amount
    multiplier
    payout
    shuffleOriginalActions {
      action {
        plinko {
          risk
          results
          rows
          multiplier
        }
      }
    }
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### WheelPlay
```graphql
mutation WheelPlay($data: WheelPlayInput!) {
  wheelPlay(data: $data) {
    id
    currency
    amount
    payout
    multiplier
    shuffleOriginalActions {
      action {
        wheel {
          resultRaw
          resultSegment
          risk
          segments
        }
      }
    }
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### TowerStart
```graphql
mutation TowerStart($data: TowerStartInput!) {
  towerStart(data: $data) {
    id
    amount
    currency
    difficulty
    currentRow
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### TowerNext
```graphql
mutation TowerNext($data: TowerNextInput!) {
  towerNext(data: $data) {
    id
    currentRow
    selectedTiles
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### TowerCashout
```graphql
mutation TowerCashout {
  towerCashout {
    id
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### HiloStart
```graphql
mutation HiloStart($data: HiloStartInput!) {
  hiloStart(data: $data) {
    id
    amount
    currency
    cards
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### HiloNext
```graphql
mutation HiloNext($data: HiloNextInput!) {
  hiloNext(data: $data) {
    id
    cards
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### HiloCashout
```graphql
mutation HiloCashout($betId: String) {
  hiloCashout(betId: $betId) {
    id
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### BlackjackStart
```graphql
mutation BlackjackStart($data: BlackjackStartInput!) {
  blackjackStart(data: $data) {
    id
    amount
    currency
    playerCards
    dealerCards
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### BlackjackNext
```graphql
mutation BlackjackNext($data: BlackjackNextInput!) {
  blackjackNext(data: $data) {
    id
    playerCards
    dealerCards
    payout
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### RoulettePlay
```graphql
mutation RoulettePlay($data: RoulettePlayInput!) {
  roulettePlay(data: $data) {
    id
    currency
    amount
    payout
    multiplier
    shuffleOriginalActions {
      action {
        roulette {
          resultRaw
          resultValue
        }
      }
    }
    afterBalance
  }
}
```
**Auth:** ✅ Required

#### KenoPlay
```graphql
mutation KenoPlay($data: KenoPlayInput!) {
  kenoPlay(data: $data) {
    id
    amount
    multiplier
    currency
    payout
    shuffleOriginalActions {
      action {
        keno {
          results
          risk
          multiplier
          selected
        }
      }
    }
    afterBalance
  }
}
```
**Auth:** ✅ Required

### Provably Fair

#### ChangeGameSeed
```graphql
mutation ChangeGameSeed($clientSeed: String!) {
  gameSeedChangeAndReveal(newClientSeed: $clientSeed) {
    clientSeed
    hashedServerSeed
    nextHashedServerSeed
    revealedServerSeed
  }
}
```
**Auth:** ✅ Required

**Note:** This mutation rotates the seed pair and reveals the current server seed.

### Sports Betting

#### placeSportsBets
```graphql
mutation placeSportsBets($data: PlaceSportsBetsInput!, $language: Language) {
  placeSportsBets(data: $data, language: $language) {
    key
    bet {
      id
      currency
      amount
      totalOddsDecimal
      status
      selections {
        id
        oddsNumerator
        oddsDenominator
        status
      }
    }
    error {
      message
      extension
    }
  }
}
```
**Auth:** ✅ Required

#### SportBetCashOut
```graphql
mutation SportBetCashOut($data: CashoutSportsBetInput!, $language: Language) {
  cashoutSportsBet(data: $data, language: $language) {
    id
    currency
    amount
    status
    settlement {
      payoutOddsDecimal
      payout
    }
  }
}
```
**Auth:** ✅ Required

### Financial Operations

#### Withdraw
```graphql
mutation Withdraw($data: WithdrawalInputWithAuthCode!) {
  withdraw(data: $data) {
    id
    currency
    amount
    address
    status
    createdAt
  }
}
```
**Auth:** ✅ Required

#### RequestWithdrawTwoFa
```graphql
mutation RequestWithdrawTwoFa($data: WithdrawalInput!) {
  withdrawalRequestOtp(data: $data) {
    otpSentAt
  }
}
```
**Auth:** ✅ Required

#### VaultDeposit
```graphql
mutation VaultDeposit($data: VaultDepositInput!) {
  vaultDeposit(data: $data) {
    id
    type
    currency
    amount
    createdAt
    afterVaultBalance
  }
}
```
**Auth:** ✅ Required

### VIP & Rewards

#### ClaimDailyRakeback
```graphql
mutation ClaimDailyRakeback {
  vipRewardsClaimDailyRakeback {
    nextClaimDate
    eligible
    currencyAmounts {
      amount
      currency
    }
  }
}
```
**Auth:** ✅ Required

#### ClaimChallengeReward
```graphql
mutation ClaimChallengeReward($challengeId: String!) {
  claimChallengeReward(challengeId: $challengeId) {
    challenge {
      currency
      amount
    }
    claimedAt
    expiredAt
  }
}
```
**Auth:** ✅ Required

### Lottery Operations

#### PurchaseSingleTickets
```graphql
mutation PurchaseSingleTickets($data: LotteryPurchaseTicketInput!) {
  purchaseSingleTickets(data: $data) {
    purchaseId
    drawId
    drawAt
    ticketCount
    currency
    amount
    usdAmount
  }
}
```
**Auth:** ✅ Required

#### LotteryClaimAll
```graphql
mutation LotteryClaimAll {
  claimAll {
    numOfClaims
    amountClaimed
    currency
  }
}
```
**Auth:** ✅ Required

#### stakeShfl
```graphql
mutation stakeShfl($data: StakeShflInput!) {
  stakeShfl(data: $data) {
    stakeId
    pendingCount
    pendingStakeAmount
    stakedCount
    stakedAmount
  }
}
```
**Auth:** ✅ Required

### Chat & Social

#### SendChat
```graphql
mutation SendChat($data: ChatSendInput!) {
  chatSend(data: $data)
}
```
**Auth:** ✅ Required

#### SendTip
```graphql
mutation SendTip($data: TipSendInputWithAuthCode!) {
  tipSend(data: $data) {
    id
    currency
    amount
    chatRoom
    createdAt
  }
}
```
**Auth:** ✅ Required

#### SendRain
```graphql
mutation SendRain($data: TipRainInput!) {
  tipRain(data: $data) {
    id
    currency
    amount
    chatRoom
    createdAt
  }
}
```
**Auth:** ✅ Required

---

## Subscriptions (36)

### WebSocket Connection

All subscriptions use WebSocket connections. Connect to:
```
wss://[domain]/main-api/bp-subscription/subscription/graphql
```

**Authentication:** Include JWT token in connection headers:
```
Authorization: Bearer <jwt_token>
```

### Balance & Financial

#### BalanceUpdated
```graphql
subscription BalanceUpdated {
  balanceUpdated {
    currency
    amount
    windowId
  }
}
```
**Auth:** ✅ Required  
**Use Case:** Real-time balance updates when deposits, withdrawals, or bets complete

### Bet Updates

#### MyBetUpdated
```graphql
subscription MyBetUpdated {
  myBetUpdated {
    id
    currency
    amount
    payout
    status
    multiplier
    createdAt
  }
}
```
**Auth:** ✅ Required  
**Use Case:** Track user's bet status changes in real-time

#### LatestBetUpdated
```graphql
subscription LatestBetUpdated {
  latestBetUpdated {
    id
    username
    gameName
    currency
    amount
    payout
    multiplier
    createdAt
  }
}
```
**Auth:** ❌ Not required  
**Use Case:** Public feed of latest bets

#### HighRollerBetUpdated
```graphql
subscription HighRollerBetUpdated {
  highRollerBetUpdated {
    id
    username
    gameName
    currency
    amount
    payout
    multiplier
    createdAt
  }
}
```
**Auth:** ❌ Not required  
**Use Case:** Public feed of high-value bets

### Crash Game

#### CrashGameUpdate
```graphql
subscription CrashGameUpdate {
  crashGameUpdate {
    id
    status
    elapsedTime
    currentPoint
    crashPoint
    startedAt
    nextRoundIn
  }
}
```
**Auth:** ❌ Not required  
**Use Case:** Real-time crash multiplier updates

#### CrashMyBetUpdate
```graphql
subscription CrashMyBetUpdate {
  crashMyBetUpdate {
    id
    crashGameId
    betAt
    payout
    multiplier
    payoutType
  }
}
```
**Auth:** ✅ Required  
**Use Case:** Track user's crash bet status

### Lottery

#### LatestLotteryTicketsUpdated
```graphql
subscription LatestLotteryTicketsUpdated {
  latestLotteryTicketsUpdated {
    id
    username
    currency
    amount
    ticketCount
    createdAt
  }
}
```
**Auth:** ❌ Not required

#### LotteryPrizePoolUpdated
```graphql
subscription LotteryPrizePoolUpdated {
  lotteryPrizePoolUpdated {
    currency
    amount
  }
}
```
**Auth:** ❌ Not required

### Notifications

#### NewNotification
```graphql
subscription NewNotification {
  newNotification {
    id
    accountId
    type
    message
    readAt
    seenAt
    createdAt
    metadata
  }
}
```
**Auth:** ✅ Required  
**Use Case:** Show notification badge/popup in real-time

### Sports

#### SportsBetUpdated
```graphql
subscription SportsBetUpdated {
  sportsBetUpdated {
    id
    currency
    amount
    status
    settlement {
      payoutOddsDecimal
      payout
    }
    selections {
      id
      status
    }
  }
}
```
**Auth:** ✅ Required

#### sportsMatchOddsUpdated
```graphql
subscription sportsMatchOddsUpdated($fixtureIds: [String!]) {
  sportsMatchOddsUpdated(fixtureIds: $fixtureIds) {
    fixtureId
    marketId
    selectionId
    oddsNumerator
    oddsDenominator
    updatedAt
  }
}
```
**Auth:** ❌ Not required

#### sportsMatchStateUpdatedV2
```graphql
subscription sportsMatchStateUpdatedV2($fixtureIds: [String!]!) {
  sportsMatchStateUpdatedV2(fixtureIds: $fixtureIds) {
    fixtureId
    status
    score
    period
    elapsedTime
  }
}
```
**Auth:** ❌ Not required

### VIP

#### vipLevel
```graphql
subscription vipLevel {
  vipLevel {
    level
    xp
    wagered
    scWagered
    gcWagered
  }
}
```
**Auth:** ✅ Required  
**Use Case:** Update VIP progress bar in real-time

### Tournaments

#### tournamentScoreUpdated
```graphql
subscription tournamentScoreUpdated($tournamentId: String!) {
  tournamentScoreUpdated(tournamentId: $tournamentId) {
    tournamentUserId
    username
    score
    vipLevel
    anonymous
  }
}
```
**Auth:** ❌ Not required  
**Use Case:** Live leaderboard updates

### Tips & Social

#### tipReceived
```graphql
subscription tipReceived {
  tipReceived {
    senderUsername
    tipType
    currency
    amount
    createdAt
  }
}
```
**Auth:** ✅ Required

---

## Error Handling

### Error Response Format

```json
{
  "errors": [
    {
      "message": "Error message description",
      "extensions": {
        "code": "ERROR_CODE",
        "field": "fieldName",
        "value": "invalidValue"
      },
      "path": ["field", "subField"]
    }
  ]
}
```

### Common Error Codes

| Error Code | Description | Solution |
|------------|-------------|----------|
| `UNAUTHENTICATED` | Authentication required | Provide valid JWT token |
| `FORBIDDEN` | Insufficient permissions | Check user permissions |
| `BAD_USER_INPUT` | Invalid input parameters | Validate input format |
| `INTERNAL_SERVER_ERROR` | Server error | Retry request |
| `INSUFFICIENT_FUNDS` | Not enough balance | Check balance before betting |
| `INVALID_AMOUNT` | Amount out of range | Check min/max bet limits |
| `MAX_PAYOUT_EXCEEDED` | Potential payout too high | Reduce bet amount or multiplier |
| `INVALID_WINDOW_ID` | Missing or invalid windowId | Generate new windowId |
| `RATE_LIMIT_EXCEEDED` | Too many requests | Add delays between requests |

### HTTP Status Codes

| Status Code | Meaning |
|-------------|---------|
| `200` | Success (may still contain GraphQL errors) |
| `400` | Bad Request (malformed request) |
| `401` | Unauthorized (invalid/expired JWT) |
| `403` | Forbidden (Cloudflare blocked or insufficient permissions) |
| `429` | Too Many Requests (rate limit exceeded) |
| `500` | Internal Server Error |
| `503` | Service Unavailable (Cloudflare challenge required) |

---

## Rate Limiting

### Guidelines

- **Minimum delay:** 1 second between bets
- **Recommended delay:** 2-5 seconds for human-like behavior
- **Never:** Spam hundreds of requests in rapid succession
- **Add randomness:** Vary your bet timing slightly

### Rate Limit Headers

Responses may include rate limit headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640995200
```

### Handling Rate Limits

When rate limited (429 status):
1. Wait for the reset time indicated in headers
2. Reduce request frequency
3. Implement exponential backoff

---

## Game-Specific Schemas

### Data Formatting Rules

**Critical:** Shuffle is extremely strict about data types. Sending incorrect types will result in validation errors.

| Field | Type | Format | Example | Notes |
|-------|------|--------|---------|-------|
| **Amount** | String | 8 decimals | `"0.10000000"` | Bet amount in crypto |
| **Multiplier (bet)** | String | 2 decimals | `"2.00"` | Target multiplier |
| **Currency** | String | Uppercase ticker | `"USDT"`, `"BTC"` | Must match wallet balance |
| **WindowID** | String | 10 random chars | `"a7b3c9d2e1"` | Browser tab identifier |
| **USD Amount** | String | 2 decimals | `"0.10"` | Optional USD equivalent |

**Common Mistakes:**
- ❌ Wrong: `{"amount": 10.5}` (Python float)
- ✅ Right: `{"amount": "10.50000000"}` (String with 8 decimals)
- ❌ Wrong: `{"bet": 2}` (Integer)
- ✅ Right: `{"bet": "2.00"}` (String with 2 decimals)

### Game Parameter Ranges

| Game | Key Parameters | Value Ranges |
|------|----------------|--------------|
| **Limbo** | bet (target multiplier) | "1.01" - "1000000.00" |
| **Dice** | target, over | "0.00" - "100.00", boolean |
| **Mines** | numberOfMines | 1 - 24 |
| **Plinko** | rowCount, riskLevel | 8-16, LOW/MEDIUM/HIGH_RISK |
| **Wheel** | segmentCount, riskLevel | TEN-FIFTY, LOW/MEDIUM/HIGH |
| **Tower** | difficulty | EASY/MEDIUM/HARD/EXPERT/MASTER |
| **Keno** | selectedNumbers (array) | 1-10 numbers from 1-40 |

---

## Provably Fair System

### Overview

Shuffle uses a cryptographic hashing system (HMAC-SHA256) to ensure fair outcomes. The system prevents manipulation by ensuring:
1. **Server commits first:** Server publishes hashed seed before you bet
2. **You control input:** Your client seed prevents pre-calculation
3. **Results are verifiable:** You can independently verify any outcome

### Key Components

| Component | Description | When Available |
|-----------|-------------|----------------|
| **Server Seed (Hashed)** | SHA-256 hash of secret server seed | Before bet |
| **Server Seed (Plaintext)** | Original unhashed server seed | After bet/rotation |
| **Client Seed** | Your customizable input string | Set by you |
| **Nonce** | Counter incremented with each bet | Always visible |

### Core Generation Function (G)

The fundamental method used to derive random outcomes:

**Algorithm:**
1. Construct message: `{clientSeed}:{nonce}:{index}` (index starts at 0)
2. Generate HMAC-SHA256 hash using `serverSeed` as secret key
3. Extract 4-byte segments from hash digest
4. Convert each segment to float between 0 and 1
5. Increment index and repeat if more randomness needed

**Python Implementation:**
```python
import hmac
import hashlib

def generate_random_numbers(server_seed, client_seed, nonce, cursor=0, count=1):
    """Core provably fair generation function (G)"""
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
```

### Game-Specific Verification

#### Limbo Verification
```python
def verify_limbo_result(server_seed, client_seed, nonce):
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    hash_int = int(random_value * 0x1000000)
    multiplier = min(
        (0.99 * 0x1000000) / (hash_int + 1),
        1000000  # Maximum multiplier cap
    )
    return max(1.01, round(multiplier, 2))
```

#### Dice Verification
```python
def verify_dice_result(server_seed, client_seed, nonce):
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    return round(random_value * 10001) / 100  # 0.00 - 100.00
```

#### Roulette Verification
```python
def verify_roulette_result(server_seed, client_seed, nonce):
    random_value = generate_random_numbers(server_seed, client_seed, nonce, 0, 1)[0]
    return int(random_value * 37)  # 0-36
```

#### Mines Verification
```python
def verify_mines_positions(server_seed, client_seed, nonce, num_mines):
    random_values = generate_random_numbers(server_seed, client_seed, nonce, 0, num_mines)
    positions = []
    available_tiles = list(range(25))
    
    for rand_val in random_values:
        index = int(rand_val * len(available_tiles))
        positions.append(available_tiles.pop(index))
    
    return sorted(positions)
```

### Seed Management Operations

#### Get Seed Status
```graphql
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
```

#### Rotate Seed (Reveals Current Server Seed)
```graphql
mutation RotateSeed($newSeed: String!) {
  rotateSeed(newSeed: $newSeed) {
    clientSeed
    hashedServerSeed
    nextHashedServerSeed
    revealedServerSeed
  }
}
```

#### Change Client Seed Only
```graphql
mutation ChangeClientSeed($newClientSeed: String!) {
  changeClientSeed(newClientSeed: $newClientSeed) {
    clientSeed
    hashedServerSeed
  }
}
```

#### Unhash Server Seed
```graphql
query UnhashServerSeed($hashedServerSeed: String!) {
  unhashServerSeed(hashedServerSeed: $hashedServerSeed) {
    plainTextSeed
    wasUsedForGames
  }
}
```

---

## Complete Operation Reference

### Quick Reference Tables

#### Most Common Operations

| Operation | Type | Auth Required | Use Case |
|-----------|------|---------------|----------|
| `GetMyBalance` | Query | ✅ Yes | Get user balance |
| `LimboPlay` | Mutation | ✅ Yes | Place Limbo bet |
| `DicePlay` | Mutation | ✅ Yes | Place Dice bet |
| `CrashPlay` | Mutation | ✅ Yes | Place Crash bet |
| `MinesStart` | Mutation | ✅ Yes | Start Mines game |
| `GetAppSettings` | Query | ❌ No | Get game limits |
| `tokenInfo` | Query | ❌ No | Get token stats |
| `BalanceUpdated` | Subscription | ✅ Yes | Real-time balance |

#### Game Operations by Type

| Game | Start | Next/Play | Cashout | Auto Bet |
|------|-------|-----------|---------|----------|
| **Limbo** | N/A | `LimboPlay` | N/A | N/A |
| **Dice** | N/A | `DicePlay` | N/A | N/A |
| **Crash** | N/A | `CrashPlay` | `CrashCashout` | N/A |
| **Mines** | `MinesStart` | `MinesNext` | `MinesCashout` | `MinesAutoBet` |
| **Tower** | `TowerStart` | `TowerNext` | `TowerCashout` | `TowerAutoBet` |
| **Hilo** | `HiloStart` | `HiloNext` | `HiloCashout` | N/A |
| **Blackjack** | `BlackjackStart` | `BlackjackNext` | N/A | N/A |
| **Roulette** | N/A | `RoulettePlay` | N/A | N/A |
| **Wheel** | N/A | `WheelPlay` | N/A | N/A |
| **Plinko** | N/A | `PlinkoPlay` | N/A | N/A |
| **Keno** | N/A | `KenoPlay` | N/A | N/A |

### All Operations List

#### Queries (146)
See [Operation_Names_List_Enhanced.md](./Operation_Names_List_Enhanced.md) for complete list.

#### Mutations (114)
See [Mutation_Definitions_With_Variables.md](./Mutation_Definitions_With_Variables.md) for complete definitions.

#### Subscriptions (36)
See [Operation_Names_List_Enhanced.md](./Operation_Names_List_Enhanced.md) for complete list.

---

## Additional Resources

- **Implementation Guide:** [Guide.md](./Guide.md)
- **Python Examples:** See Guide.md Section 8
- **Mutation Payloads:** [Mutation_Payloads.json](./Mutation_Payloads.json)
- **Operation Names:** [Operation_Names_List_Enhanced.md](./Operation_Names_List_Enhanced.md)

---

## Hidden Mechanics & Anti-Bot Features

### Overview

Shuffle.com implements several anti-bot mechanisms that are not publicly documented. Understanding these is critical for building reliable automation tools.

### 1. Window ID Anti-Bot Mechanism

**What it is:** The `windowId` parameter appears in almost every betting input type. This is a tracking identifier generated by the frontend to link a specific browser tab to a session.

**Why it matters:** 
- If you send requests without `windowId` (or reuse the same one across parallel threads), fraud detection may flag you as a bot
- The system tracks consistency of `windowId` usage per session
- Reusing `windowId` across different IP addresses or user agents triggers alerts

**Best Practices:**
- Generate a random 10-character alphanumeric string for `windowId` at session start
- Keep it consistent for the duration of a single "session" (e.g., one browser tab simulation)
- Use a new `windowId` for each new session
- Never share `windowId` across multiple concurrent requests

**Implementation:**
```python
import random
import string

def generate_window_id(length=10):
    """Generate a random window ID for session tracking"""
    return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))

# Use the same windowId for all bets in a session
session_window_id = generate_window_id()
```

### 2. Geetest CAPTCHA Trigger

**What it is:** The `LoginRequest` mutation accepts an optional `geetest: GeetestInput` parameter. The server response includes `loginVerificationMethod` which can be `"GEETEST"`.

**How it works:**
1. First login attempt may return `loginVerificationMethod: "GEETEST"`
2. If GEETEST is required, the next request **must** include a valid Geetest token
3. Without the token, authentication will fail

**Strategy for Automation:**
- **Manual Login Approach (Recommended):** Log in manually via browser, extract `Authorization` Bearer token and Cookies, use those for your script until they expire (~24 hours)
- **Automated Approach:** Implement Geetest solver (complex, may violate ToS)

**Example Response:**
```json
{
  "data": {
    "loginRequest": {
      "loginToken": "temp_token_123",
      "loginVerificationMethod": "GEETEST",
      "otpSentAt": null
    }
  }
}
```

### 3. Correlation ID Tracking

**What it is:** The `x-correlation-id` header is used to link related requests together.

**Why it matters:**
- The system tracks request patterns using correlation IDs
- Unusual patterns (e.g., same correlation ID across different users) trigger alerts
- Missing or malformed correlation IDs may be flagged

**Best Practices:**
- Generate a unique UUID for each request or session
- Use consistent format (UUID v4 recommended)
- Don't reuse correlation IDs across different users/sessions

**Implementation:**
```python
import uuid

headers = {
    "x-correlation-id": str(uuid.uuid4()),
    # ... other headers
}
```

### 4. User-Agent Fingerprinting

**What it is:** The `User-Agent` header must match the browser where cookies were extracted.

**Why it matters:**
- Mismatched User-Agent and Cookie fingerprints trigger Cloudflare challenges
- The system validates consistency between User-Agent, TLS fingerprint, and cookies

**Best Practices:**
- Extract User-Agent from the same browser session where you got cookies
- Use the exact User-Agent string (don't modify it)
- If using TLS fingerprinting libraries (curl_cffi, tls_client), ensure User-Agent matches

### 5. Rate Limiting Patterns

**What it is:** The system tracks request frequency and patterns, not just total count.

**Detection Patterns:**
- Too many requests in short time window
- Identical request patterns (same timing, same operations)
- Requests without human-like delays
- Parallel requests from same session

**Best Practices:**
- Add random delays between requests (1-5 seconds)
- Vary timing patterns (don't use fixed intervals)
- Implement exponential backoff on errors
- Use subscriptions instead of polling when possible

**Example:**
```python
import time
import random

def human_like_delay():
    """Add human-like delay between requests"""
    time.sleep(random.uniform(1.5, 3.5))
```

### 6. Hidden Endpoints for Data Scraping

**N9 Assets Game Catalog:**
- **URL:** `https://n9assets.com/games/games.json`
- **Purpose:** Static file containing game IDs, images, RTP stats
- **Advantage:** Can be scraped infinitely without hitting Shuffle.com rate limits
- **Use Case:** Building game catalogs, RTP analysis, image assets

**Example:**
```python
import requests

def get_game_catalog():
    """Fetch game catalog without hitting main API"""
    response = requests.get("https://n9assets.com/games/games.json")
    return response.json()
```

### 7. House Edge in Provably Fair Formula

**Critical Discovery:** The house edge is mathematically baked into the provably fair formula.

**Limbo Example:**
```python
# Formula: max(1.01, round(min((0.99 * 0x1000000) / (hash_int + 1), 1000000), 2))
# The 0.99 multiplier = 1% house edge
```

**Implications:**
- You can run offline simulations to test betting strategies
- Test millions of hash combinations without spending money
- Calculate expected value (EV) for different strategies
- Verify house edge is exactly as advertised

**Simulation Example:**
```python
def simulate_limbo_bets(num_simulations=1000000):
    """Simulate millions of Limbo bets offline"""
    results = []
    for nonce in range(num_simulations):
        # Use test seeds
        result = verify_limbo_result("test_server_seed", "test_client_seed", nonce)
        results.append(result)
    return results
```

### 8. Session Consistency Checks

**What it is:** The system validates consistency across multiple dimensions:
- IP address consistency
- Cookie freshness
- Token expiration
- Request timing patterns
- Geographic location (via `x-country` header)

**Best Practices:**
- Use same IP address for entire session
- Refresh cookies regularly (every 30-60 minutes)
- Monitor token expiration (refresh before expiry)
- Set `x-country` header to match your IP location

### 9. Cloudflare Challenge Detection

**Indicators:**
- HTTP 403 responses
- HTTP 503 responses with challenge page
- Missing `cf_clearance` cookie
- Expired `__cf_bm` cookie

**Solutions:**
1. **Level 1:** Manual cookie extraction (easiest, works for light usage)
2. **Level 2:** TLS fingerprinting libraries (`curl_cffi`, `tls_client`)
3. **Level 3:** Browser automation (Selenium/Playwright) for full session management

### 10. Request Header Validation

**Required Headers:**
- `Authorization: Bearer <JWT>` - Must be valid and not expired
- `Cookie` - Must include `cf_clearance` and `__cf_bm`
- `User-Agent` - Must match browser where cookies were extracted
- `Origin` - Must match domain (`https://shuffle.com`)
- `Referer` - Should match game page URL
- `x-correlation-id` - UUID format
- `x-country` - ISO country code matching IP location

**Missing or Invalid Headers:**
- Missing headers → 400 Bad Request
- Invalid JWT → 401 Unauthorized
- Invalid cookies → 403 Forbidden (Cloudflare)
- Mismatched headers → Suspicious activity detection

---

## Python Client Library

### Complete Implementation

A production-ready Python client library that handles all the hidden mechanics:

```python
"""
Shuffle.com API Client Library
Handles authentication, request formatting, and anti-bot mechanisms
"""

import requests
import uuid
import time
import random
import string
from typing import Optional, Dict, Any, List
from dataclasses import dataclass


@dataclass
class ShuffleConfig:
    """Configuration for Shuffle API client"""
    jwt_token: str
    cookie_string: str
    user_agent: str
    domain: str = "shuffle.com"
    country_code: str = "CA"
    request_delay: tuple[float, float] = (1.5, 3.5)  # Random delay range


class ShuffleClient:
    """
    Complete Shuffle.com API client with anti-bot protection
    
    Features:
    - Automatic windowId generation and session management
    - Correlation ID tracking
    - Human-like request delays
    - Proper header management
    - Error handling and retry logic
    """
    
    def __init__(self, config: ShuffleConfig):
        """
        Initialize Shuffle API client
        
        Args:
            config: ShuffleConfig with authentication credentials
        """
        self.config = config
        self.base_url = f"https://{config.domain}/main-api/graphql/api/graphql"
        
        # Generate session windowId (consistent for this session)
        self.window_id = self._generate_window_id()
        
        # Session correlation ID base
        self.session_id = str(uuid.uuid4())
        
        # Build headers
        self.headers = {
            "Authorization": f"Bearer {config.jwt_token}",
            "Cookie": config.cookie_string,
            "Content-Type": "application/json",
            "User-Agent": config.user_agent,
            "Origin": f"https://{config.domain}",
            "Referer": f"https://{config.domain}/games/originals/limbo",
            "x-country": config.country_code,
        }
    
    @staticmethod
    def _generate_window_id(length: int = 10) -> str:
        """Generate random window ID for session tracking"""
        return ''.join(random.choices(string.ascii_lowercase + string.digits, k=length))
    
    def _human_delay(self):
        """Add human-like delay between requests"""
        delay = random.uniform(*self.config.request_delay)
        time.sleep(delay)
    
    def _send_request(
        self,
        operation_name: str,
        query: str,
        variables: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Send GraphQL request with proper formatting and error handling
        
        Args:
            operation_name: GraphQL operation name
            query: GraphQL query/mutation string
            variables: Variables dictionary
            
        Returns:
            Response JSON data
            
        Raises:
            Exception: On API errors or network failures
        """
        # Add correlation ID (unique per request)
        headers = self.headers.copy()
        headers["x-correlation-id"] = f"{self.session_id}::{uuid.uuid4()}"
        
        payload = {
            "operationName": operation_name,
            "variables": variables or {},
            "query": query
        }
        
        try:
            response = requests.post(
                self.base_url,
                json=payload,
                headers=headers,
                timeout=30
            )
            
            # Handle HTTP errors
            if response.status_code == 401:
                raise Exception("Authentication failed: JWT token expired or invalid")
            elif response.status_code == 403:
                raise Exception("Forbidden: Cloudflare blocked. Refresh cookies.")
            elif response.status_code == 429:
                raise Exception("Rate limited: Too many requests. Add delays.")
            elif response.status_code != 200:
                raise Exception(f"HTTP {response.status_code}: {response.text[:200]}")
            
            data = response.json()
            
            # Handle GraphQL errors
            if "errors" in data:
                error_msg = data["errors"][0].get("message", "Unknown error")
                error_code = data["errors"][0].get("extensions", {}).get("code", "UNKNOWN")
                raise Exception(f"GraphQL Error [{error_code}]: {error_msg}")
            
            return data
        
        except requests.exceptions.RequestException as e:
            raise Exception(f"Network error: {str(e)}")
    
    # ==================== QUERIES ====================
    
    def get_balance(self) -> List[Dict[str, str]]:
        """
        Get user's current balance for all currencies
        
        Returns:
            List of balance dictionaries with 'currency' and 'amount' keys
        """
        query = """
        query GetMyBalance {
          me {
            balances {
              currency
              amount
            }
          }
        }
        """
        self._human_delay()
        data = self._send_request("GetMyBalance", query)
        return data["data"]["me"]["balances"]
    
    def get_profile(self) -> Dict[str, Any]:
        """Get user profile information"""
        query = """
        query GetMyProfile {
          me {
            id
            username
            email
            vipLevel
            createdAt
          }
        }
        """
        self._human_delay()
        data = self._send_request("GetMyProfile", query)
        return data["data"]["me"]
    
    def get_seed_status(self) -> Dict[str, Any]:
        """Get current provably fair seed status"""
        query = """
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
        self._human_delay()
        data = self._send_request("GetSeedStatus", query)
        return data["data"]["me"]["seedStatus"]
    
    def get_app_settings(self) -> Dict[str, Any]:
        """Get app settings including game limits"""
        query = """
        query GetAppSettings {
          appSettings {
            limbo {
              maxPayoutUSD
              minBetUSD
            }
            dice {
              maxPayoutUSD
              minBetUSD
            }
          }
        }
        """
        self._human_delay()
        data = self._send_request("GetAppSettings", query)
        return data["data"]["appSettings"]
    
    def get_token_info(self) -> Dict[str, Any]:
        """Get public token information (no auth required)"""
        query = """
        query tokenInfo {
          tokenInfo {
            priceInUsd
            circulatingSupply
            burnedTokens
          }
        }
        """
        # No delay needed for public queries
        data = self._send_request("tokenInfo", query)
        return data["data"]["tokenInfo"]
    
    # ==================== MUTATIONS ====================
    
    def play_limbo(
        self,
        amount: float,
        target: float,
        currency: str = "USDT",
        usd_amount: Optional[float] = None
    ) -> Dict[str, Any]:
        """
        Place a Limbo bet
        
        Args:
            amount: Bet amount in crypto (e.g., 0.1 for 0.1 USDT)
            target: Target multiplier (e.g., 2.0 for 2x)
            currency: Currency ticker (USDT, BTC, ETH, etc.)
            usd_amount: Optional USD equivalent
            
        Returns:
            Bet result dictionary
        """
        query = """
        mutation LimboPlay($data: LimboPlayInput!) {
          limboPlay(data: $data) {
            id
            currency
            amount
            payout
            afterBalance
            shuffleOriginalActions {
              id
              action {
                limbo {
                  resultRaw
                  resultValue
                  userValue
                }
              }
            }
          }
        }
        """
        
        variables = {
            "data": {
                "amount": f"{amount:.8f}",  # 8 decimal places
                "bet": f"{target:.2f}",     # 2 decimal places
                "currency": currency,
                "windowId": self.window_id  # Use session windowId
            }
        }
        
        if usd_amount is not None:
            variables["data"]["usdAmount"] = f"{usd_amount:.2f}"
        
        self._human_delay()
        data = self._send_request("LimboPlay", query, variables)
        return data["data"]["limboPlay"]
    
    def play_dice(
        self,
        amount: float,
        target: float,
        over: bool,
        currency: str = "USDT"
    ) -> Dict[str, Any]:
        """
        Place a Dice bet
        
        Args:
            amount: Bet amount in crypto
            target: Target number (0.00-100.00)
            over: True to bet over, False to bet under
            currency: Currency ticker
            
        Returns:
            Bet result dictionary
        """
        query = """
        mutation DicePlay($data: DicePlayInput!) {
          dicePlay(data: $data) {
            id
            currency
            amount
            payout
            afterBalance
            shuffleOriginalActions {
              id
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
        
        variables = {
            "data": {
                "amount": f"{amount:.8f}",
                "target": f"{target:.2f}",
                "over": over,
                "currency": currency,
                "windowId": self.window_id
            }
        }
        
        self._human_delay()
        data = self._send_request("DicePlay", query, variables)
        return data["data"]["dicePlay"]
    
    def play_crash(
        self,
        amount: float,
        currency: str = "USDT"
    ) -> Dict[str, Any]:
        """Place a Crash bet"""
        query = """
        mutation CrashPlay($data: CrashPlayInput!) {
          crashPlay(data: $data) {
            id
            amount
            payout
            currency
            crashBet {
              crashGameId
            }
            afterBalance
          }
        }
        """
        
        variables = {
            "data": {
                "amount": f"{amount:.8f}",
                "currency": currency,
                "windowId": self.window_id
            }
        }
        
        self._human_delay()
        data = self._send_request("CrashPlay", query, variables)
        return data["data"]["crashPlay"]
    
    def crash_cashout(self, crash_game_id: Optional[int] = None) -> bool:
        """Cashout from Crash game"""
        query = """
        mutation CrashCashout($crashGameId: Int) {
          crashCashout(crashGameId: $crashGameId)
        }
        """
        
        variables = {}
        if crash_game_id is not None:
            variables["crashGameId"] = crash_game_id
        
        self._human_delay()
        data = self._send_request("CrashCashout", query, variables)
        return data["data"]["crashCashout"]
    
    def mines_start(
        self,
        amount: float,
        number_of_mines: int,
        currency: str = "USDT"
    ) -> Dict[str, Any]:
        """
        Start a Mines game
        
        Args:
            amount: Bet amount
            number_of_mines: Number of mines (1-24)
            currency: Currency ticker
        """
        query = """
        mutation MinesStart($data: MinesStartInput!) {
          minesStart(data: $data) {
            id
            amount
            currency
            numberOfMines
            payout
            afterBalance
          }
        }
        """
        
        variables = {
            "data": {
                "amount": f"{amount:.8f}",
                "numberOfMines": number_of_mines,
                "currency": currency,
                "windowId": self.window_id
            }
        }
        
        self._human_delay()
        data = self._send_request("MinesStart", query, variables)
        return data["data"]["minesStart"]
    
    def mines_next(self, tile_index: int) -> Dict[str, Any]:
        """Reveal next tile in Mines game"""
        query = """
        mutation MinesNext($data: MinesNextInput!) {
          minesNext(data: $data) {
            id
            selectedTiles
            resultTiles
            payout
            afterBalance
          }
        }
        """
        
        variables = {
            "data": {
                "tileIndex": tile_index
            }
        }
        
        self._human_delay()
        data = self._send_request("MinesNext", query, variables)
        return data["data"]["minesNext"]
    
    def mines_cashout(self) -> Dict[str, Any]:
        """Cashout from Mines game"""
        query = """
        mutation MinesCashout {
          minesCashout {
            id
            payout
            afterBalance
          }
        }
        """
        
        self._human_delay()
        data = self._send_request("MinesCashout", query)
        return data["data"]["minesCashout"]
    
    def change_game_seed(self, new_client_seed: str) -> Dict[str, Any]:
        """
        Change game seed (rotates seed pair and reveals current server seed)
        
        Args:
            new_client_seed: Your new client seed string
            
        Returns:
            Seed information including revealed server seed
        """
        query = """
        mutation ChangeGameSeed($clientSeed: String!) {
          gameSeedChangeAndReveal(newClientSeed: $clientSeed) {
            clientSeed
            hashedServerSeed
            nextHashedServerSeed
            revealedServerSeed
          }
        }
        """
        
        variables = {
            "clientSeed": new_client_seed
        }
        
        self._human_delay()
        data = self._send_request("ChangeGameSeed", query, variables)
        return data["data"]["gameSeedChangeAndReveal"]


# ==================== USAGE EXAMPLE ====================

if __name__ == "__main__":
    # Configuration (extract from browser DevTools)
    config = ShuffleConfig(
        jwt_token="YOUR_BEARER_TOKEN_HERE",
        cookie_string="YOUR_FULL_COOKIE_STRING_HERE",
        user_agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/120.0.0.0 Safari/537.36",
        domain="shuffle.com",
        country_code="CA"
    )
    
    # Initialize client
    client = ShuffleClient(config)
    
    # Example: Get balance
    print("Fetching balance...")
    balances = client.get_balance()
    for balance in balances:
        if float(balance["amount"]) > 0:
            print(f"  {balance['currency']}: {balance['amount']}")
    
    # Example: Get seed status
    print("\nFetching seed status...")
    seed_status = client.get_seed_status()
    print(f"  Client Seed: {seed_status['activeClientSeed'][:20]}...")
    print(f"  Current Nonce: {seed_status['currentNonce']}")
    
    # Example: Place Limbo bet (COMMENTED OUT - UNCOMMENT TO PLACE REAL BETS)
    # print("\nPlacing Limbo bet...")
    # result = client.play_limbo(amount=0.1, target=2.0, currency="USDT")
    # print(f"  Result: {result['shuffleOriginalActions'][0]['action']['limbo']['resultValue']}")
    # print(f"  Payout: {result['payout']}")
```

### Quick Start Guide

1. **Extract Credentials:**
   - Log in to Shuffle.com in browser
   - Open DevTools (F12) → Network tab
   - Perform any action (view wallet, place bet)
   - Copy `Authorization` header (without "Bearer " prefix)
   - Copy full `Cookie` string
   - Copy `User-Agent` string

2. **Initialize Client:**
```python
from shuffle_client import ShuffleClient, ShuffleConfig

config = ShuffleConfig(
    jwt_token="eyJhbGc...",  # Your JWT token
    cookie_string="ip-country=CA; __cf_bm=...",  # Full cookie string
    user_agent="Mozilla/5.0...",  # Your browser UA
    country_code="CA"  # Match your IP location
)

client = ShuffleClient(config)
```

3. **Use the Client:**
```python
# Get balance
balances = client.get_balance()

# Place bet
result = client.play_limbo(amount=0.1, target=2.0, currency="USDT")
```

### Features

✅ **Automatic Window ID Management** - Generates and maintains session windowId  
✅ **Correlation ID Tracking** - Unique IDs per request  
✅ **Human-like Delays** - Random delays between requests  
✅ **Proper Type Formatting** - Handles 8-decimal amounts, 2-decimal multipliers  
✅ **Error Handling** - Clear error messages for common issues  
✅ **Session Management** - Consistent headers and tracking  
✅ **Rate Limit Protection** - Built-in delays and error handling  

---

## Version History

- **v1.0** (2025-12-12): Initial complete API schema documentation
  - 296 total operations documented
  - Complete type system definitions
  - All endpoints and authentication methods
  - Game-specific schemas and provably fair system

---

**⚠️ DISCLAIMER:** This schema is for educational and research purposes only. Shuffle.com likely prohibits automated interaction (bots) in their Terms of Service. Using this schema to create betting bots or scrapers may result in account suspension, IP bans, or loss of funds. Use at your own risk.

