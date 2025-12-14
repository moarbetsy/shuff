***

# GraphQL API Documentation v2.0

## Overview

This GraphQL API provides access to a comprehensive gaming and betting platform with **dual-mode currency systems**: Standard Crypto (Global) and Sweepstakes (US). The platform supports user authentication, wallet management, casino game logic (Crash, Dice, Mines, etc.), sports betting with DRM-protected live streaming, lottery, SHFL token staking and governance, VIP rewards, responsible gaming controls, and real-time state updates.

### Key Features

*   **Dual Currency Systems:** Standard crypto gambling (global) and US-compliant Sweepstakes model (GC/SC).
*   **SHFL Tokenomics:** Staking, vesting schedules, and airdrop management.
*   **Responsible Gaming:** Self-exclusion, deposit limits, and loss limits.
*   **Live Sports Streaming:** DRM-protected video streams with Widevine/PlayReady/FairPlay.
*   **Provably Fair Gaming:** Cryptographic verification for all casino games with comprehensive verification tools.
*   **Real-Time Updates:** WebSocket subscriptions for live odds, game states, and notifications.

## Table of Contents

1.  [Schema Overview](#schema-overview)
2.  [Scalar Types](#scalar-types)
3.  [Enumerations](#enumerations)
    *   [Core Enums](#core-enums)
    *   [Game Specifics](#game-specifics)
    *   [Verification & Compliance](#verification--compliance)
    *   [Miscellaneous Enums](#miscellaneous-enums)
4.  [Authentication](#authentication)
5.  [Sweepstakes Model (US Compliance)](#sweepstakes-model-us-compliance)
6.  [SHFL Tokenomics](#shfl-tokenomics)
7.  [Responsible Gaming](#responsible-gaming)
    *   [Self-Exclusion](#self-exclusion)
    *   [Deposit & Loss Limits](#deposit--loss-limits)
    *   [Reality Checks](#reality-checks)
8.  [Queries](#queries)
    *   [User & Profile](#user--profile)
    *   [Wallet & Finance](#wallet--finance)
    *   [Casino Games](#casino-games)
    *   [Sports Betting](#sports-betting)
    *   [Sports Streaming (DRM)](#sports-streaming-drm)
    *   [Lottery System](#lottery-system)
    *   [Promotions & Rewards](#promotions--rewards)
    *   [System & Config](#system--config)
9.  [Mutations](#mutations)
    *   [Auth & Account](#auth--account)
    *   [Game Actions](#game-actions)
    *   [Betting & Transactions](#betting--transactions)
    *   [Finance Mutations](#finance-mutations)
    *   [Rewards & Claims](#rewards--claims)
10. [Subscriptions](#subscriptions)
11. [File Uploads (KYC)](#file-uploads-kyc)
12. [Usage Examples](#usage-examples)
13. [Error Handling](#error-handling)
14. [Rate Limiting](#rate-limiting)
15. [Best Practices](#best-practices)
16. [Provably Fair Gaming](#provably-fair-gaming)
17. [WebSocket Connection](#websocket-connection)
18. [API Versioning](#api-versioning)
19. [Support & Resources](#support--resources)
20. [Appendix A: Technical Specifications & Implementation Details](#appendix-a-technical-specifications--implementation-details)
    *   [1. Input Objects Reference](#1-input-objects-reference)
    *   [2. Standard Fragments](#2-standard-fragments)
    *   [3. Workflow Diagrams](#3-workflow-diagrams)
    *   [4. Public vs. Protected Access](#4-public-vs-protected-access)
    *   [5. Technical Implementation Nits](#5-technical-implementation-nits)
21. [Appendix B: Provably Fair Verification Guide](#appendix-b-provably-fair-verification-guide)
    *   [1. Bet Data Export Format](#1-bet-data-export-format)
    *   [2. Provably Fair Interface Components](#2-provably-fair-interface-components)
    *   [3. URL-Based Bet Verification](#3-url-based-bet-verification)
    *   [4. Bet ID System](#4-bet-id-system)
    *   [5. Complete Verification Example](#5-complete-verification-example)
    *   [6. Game-Specific Verification Algorithms](#6-game-specific-verification-algorithms)
    *   [7. Verification Tools & Resources](#7-verification-tools--resources)
    *   [8. Best Practices for Players](#8-best-practices-for-players)

***

## 1. Schema Overview

The API is organized into three main operation types. All requests should be sent to the GraphQL endpoint (typically `/graphql`).

```graphql
schema {
  query: Query
  mutation: Mutation
  subscription: Subscription
}
```

---

## 2. Scalar Types

| Type | Description | Example |
|------|-------------|---------|
| `Int` | Signed 32-bit integer | `42` |
| `Float` | Signed double-precision floating-point value | `3.14159` |
| `String` | UTF-8 character sequence | `"Hello World"` |
| `Boolean` | True or false | `true` |
| `ID` | Unique identifier (often serialized as a string) | `"user_123"` |
| `Decimal` | High-precision decimal for financial calculations (returned as a string) | `123.45678900` |
| `DateTime` | ISO 8601 date-time string | `2025-12-12T14:30:00.000Z` |
| `UUID` | Universally Unique Identifier | `550e8400-e29b-41d4-a716-446655440000` |
| `JSON` | Arbitrary JSON object | `{"key": "value"}` |
| `Upload` | File upload payload for KYC documents | Binary file data |

---

## 3. Enumerations

The API uses extensive enumerations to define game types, currencies, and states.

### Core Enums

#### `CurrencyCode` / `Currency`
Supported cryptocurrencies and sweepstakes currencies for transactions:

**Major Cryptocurrencies:**
- `BTC`, `ETH`, `SOL`, `LTC`
**Stablecoins:**
- `USDT`, `USDC`, `BUSD`, `DAI`
**Sweepstakes Currencies (US Only):**
- `GC` (Gold Coins), `SC` (Sweepstakes Coins)
**Altcoins:**
- `TRX`, `MATIC`, `XRP`, `BNB`, `DOGE`, `AVAX`, `TON`, `SHIB`, `BONK`, `WIF`, `SHFL`, `TRUMP`, `PUMP`

#### `FiatCurrency`
Supported fiat currencies for display and conversion:
- `USD`, `EUR`, `JPY`, `BRL`, `CAD`, `CNY`, `IDR`, `INR`, `KRW`, `MXN`, `PHP`, `TRY`, `VND`, `RUB`, `ARS`, `PLN`, `DKK`, `NZD`

#### `VipLevel`
Hierarchical VIP tier system with 57 distinct levels:
- `UNRANKED`, `WOOD`
- `BRONZE_1` to `BRONZE_5`
- `SILVER_1` to `SILVER_5`
- `GOLD_1` to `GOLD_5`
- `PLATINUM_1` to `PLATINUM_5`
- `SAPPHIRE_1` to `SAPPHIRE_5`
- `RUBY_1` to `RUBY_5`
- `DIAMOND_1` to `DIAMOND_5`
- `JADE_1` to `JADE_5`
- `OPAL_1` to `OPAL_5`
- `DRAGON_1` to `DRAGON_5`
- `MYTHIC`, `DARK`, `LEGEND`, `MASTER` (Elite Tiers)

#### `Language`
Supported interface languages:
`EN`, `JA`, `ES`, `PT`, `ZH`, `KO`, `FR`, `DE`, `RU`

### Game Specifics

#### `GameType`
Available casino games:
- `CRASH`, `DICE`, `MINES`, `PLINKO`, `LIMBO`, `KENO`, `HILO`, `BLACKJACK`, `ROULETTE`, `WHEEL`, `TOWER`

#### `GameAction`
Actions that can be performed during games:
- **Blackjack**: `START`, `HIT`, `STAND`, `DOUBLE_DOWN`, `SPLIT`
- **General**: `CASHOUT`, `NEXT`
- **Hilo**: `HIGHER`, `LOWER`, `SKIP`

#### `SportsBetStatus`
Status of sports betting wagers:
- `PENDING`, `WON`, `LOST`, `CASHOUT`, `VOID`, `REFUNDED`

### Verification & Compliance

#### `KycLevel`
Know Your Customer verification levels:
- `KYC_1`, `KYC_2`, `KYC_3`

#### `RiskLevel`
User risk assessment classification:
- `LOW_RISK`, `MEDIUM_RISK`, `HIGH_RISK`

#### `UsState`
All 50 US states for geographical compliance.

### Miscellaneous Enums

#### `GenericEnum`
Multi-purpose enumeration containing various categories:
- **Game Difficulty:** `EASY`, `MEDIUM`, `HARD`, `EXPERT`
- **Roulette Betting Options:** `BLACK`, `RED`, `ODD`, `EVEN`, etc.
- **Authentication Methods:** `EMAIL_OTP`, `TOTP`
- **Status Values:** `ACTIVE`, `COMPLETED`, `INCOMPLETE`, `REQUESTED`, etc.
- **Blockchain Networks:** `TRON`, `BINANCE_SMART_CHAIN`, `ARBITRUM`, `BASE`, `MATIC_POLYGON`, `AVAXC`

---

## 4. Authentication

Authentication is handled via **Bearer tokens**. You must obtain an `accessToken` via the `login` or `register` mutations.

**Required Header:**
```
Authorization: Bearer <your_access_token>
```

**Authentication Flow:**
1. Call `loginRequest` with credentials → receive `loginToken`
2. Call `login` with `loginToken` and 2FA code (if enabled) → receive `accessToken`
3. Include `accessToken` in all subsequent requests

**Token Refresh:** Use the `refreshToken` returned during login to obtain a new `accessToken` when it expires.

---

## 5. Sweepstakes Model (US Compliance)

The API automatically detects user region. US users operate under a Sweepstakes model to comply with local gambling regulations.

### Currency System
*   **GC (Gold Coins):** Social/play currency with no monetary value. Cannot be redeemed.
*   **SC (Sweepstakes Coins):** Prize/sweepstakes currency. Can be redeemed for cryptocurrency after meeting KYC/minimum requirements.

### Sweepstakes Queries

#### `GetSweepstakesBalance`
Fetches the dual-wallet state for US players.

```graphql
query GetSweepstakesBalance {
  me {
    account {
      balances {
        currency # Returns GC or SC
        amount
      }
    }
    redeemableBalance {
      redeemableAmount
      nextRedemptionDate
      allowedChainCurrency {
        chain
        currency
      }
    }
  }
}
```

### Sweepstakes Mutations

#### `RequestSweepStakesRedemption`
Convert SC winnings into cryptocurrency (redemption process).

```graphql
mutation RequestRedemption($data: RedeemInputWithAuthCode!) {
  requestSweepStakesRedemption(data: $data) {
    id
    status
    amount
    currency
    address
  }
}
```

---

## 6. SHFL Tokenomics

The platform includes a comprehensive SHFL token utility layer for staking, governance, and rewards.

### Token Queries

#### `GetTokenInfo`
Global statistics for the SHFL token ecosystem.

```graphql
query GetTokenInfo {
  tokenInfo {
    tvl # Total Value Locked in staking
    marketCap
    shflHolders
    price
    priceChange24h
  }
}
```

#### `GetMyStakingInfo`
Personal staking details and rewards.

```graphql
query GetMyStakingInfo {
  myStakingInfo {
    totalStaked
    pendingRewards
    lotteryEntries
    stakingApy
    lockupPeriod
  }
}
```

### Token Mutations

#### `StakeShfl`
Stake SHFL tokens to receive lottery entries and rewards.

```graphql
mutation StakeShfl($data: StakeShflInput!) {
  stakeShfl(data: $data) {
    stakeId
    stakedAmount
    lotteryEntries
    lockupEndsAt
  }
}
```

---

## 7. Responsible Gaming

The platform includes comprehensive Responsible Gaming (RG) features for player protection.

### Self-Exclusion

#### `CreateSelfExclusion`
Temporarily or permanently disable account access.

```graphql
mutation CreateSelfExclusion($data: SelfExclusionInput!) {
  confirmSelfExclusion(data: $data) {
    id
    selfExclusionType # PERMANENT, TEMPORARY
    selfExclusionUntilAt
    status
  }
}
```

### Deposit & Loss Limits

#### `EnableResponsibleLimit`
Set financial limits for responsible gambling.

```graphql
mutation EnableResponsibleLimit($data: EnableResponsibleLimitInput!) {
  enableResponsibleLimit(data: $data) {
    id
    type # DEPOSIT_LIMIT, LOSS_LIMIT, WAGER_LIMIT
    period # DAILY, WEEKLY, MONTHLY
    amount
    currency
  }
}
```

### Reality Checks

#### `GetSessionInfo`
Track session duration and wagering.

```graphql
query GetSessionInfo {
  mySessionInfo {
    sessionDuration # Minutes
    totalWagered
    netProfit
    betCount
  }
}
```

---

## 8. Queries

### User & Profile

| Query | Description |
|---|---|
| `GetMyProfile` | Fetches complete profile, balances, VIP level, and XP. |
| `GetUserProfile($username: String!)` | Public profile data of another user. |
| `GetUserSessions` | Login history and active sessions. |
| `GetMyKyc` | Current KYC verification status. |
| `GetMyReferralStats` | Referral program statistics. |

### Wallet & Finance

| Query | Description |
|---|---|
| `GetMyBalance` | Current available wallet balances for all currencies. |
| `GetWallets` | Deposit addresses for each supported cryptocurrency. |
| `GetDeposits` | History of deposit transactions. |
| `GetWithdrawals` | History of withdrawal transactions. |
| `GetTransactions` | Generic transaction history (bets, wins, transfers, tips). |
| `GetPrices($currency: FiatCurrency!)` | Cryptocurrency to fiat exchange rates. |

### Casino Games

| Query | Description |
|---|---|
| `GetLobbyGames` | Fetches categorized games (Slots, Live, Originals). |
| `GetGameSeeds` | Current client/server seeds for provably fair verification. |
| `GetBets` | Global bet feed showing recent platform bets. |
| `GetMyBets` | User's personal bet history. |
| `GetBetInfo($betId: String!)` | Detailed result and provably fair data for a specific bet. |
| `GetCrashGame` | Current state of the active Crash game round. |

### Sports Betting

| Query | Description |
|---|---|
| `GetSports` | List of available sports categories. |
| `GetSportsFixtures` | Matches and events for a specific competition. |
| `GetSportsMarketInfo($fixtureId: String!)` | Detailed odds and markets for a specific fixture. |
| `GetSportsBet($id: String!)` | Details of a placed sports bet. |
| `GetSportsMatchStates` | Real-time score and status of specific matches. |

### Sports Streaming (DRM)

#### `GetAvailableStreams`
Check which fixtures have live streams available.

```graphql
query GetAvailableStreams($fixtureIds: [String!]!) {
  availableStreams(fixtureIds: $fixtureIds) {
    fixtureId
    hasStream
    streamQuality
  }
}
```

### Lottery System

| Query | Description |
|---|---|
| `GetActiveLotteryDraw` | Information about the current active lottery draw. |
| `GetMyLotteryTickets` | View purchased tickets for upcoming draws. |
| `GetLotteryHistory` | Past lottery results and winners. |

### Promotions & Rewards

| Query | Description |
|---|---|
| `GetVipDailyRakeback` | Daily rakeback status and claimable amount. |
| `GetRaceLeaderBoardV2($raceId: String!)` | Current wager race standings. |
| `GetChallengeRewards` | Completed challenges ready to claim. |

### System & Config

| Query | Description |
|---|---|
| `GetAppSettings` | Global platform settings, limits, and configurations. |
| `GetChainsCurrencies` | Supported blockchain networks and withdrawal fees. |

---

## 9. Mutations

### Auth & Account

| Mutation | Description |
|---|---|
| `LoginRequest` | Step 1 of login flow (returns `loginToken`). |
| `Login` | Step 2 of login (returns `accessToken`, `refreshToken`). |
| `RegisterAccount` | Create a new user account. |
| `LogoutOtherSessions` | Invalidate all other active sessions. |
| `ChangePassword` | Update user password. |
| `ToggleTotp` | Enable or disable two-factor authentication. |
| `SendKycLevel1/2/3` | Submit KYC verification documents. |

### Game Actions

| Game | Start Mutation | Next Action Mutation | Cashout Mutation |
|---|---|---|---|
| **Crash** | `CrashPlay` | `CrashCashout` (manual) | `CrashCashout` |
| **Dice** | `DicePlay` | N/A | N/A |
| **Mines** | `MinesStart` | `MinesNext` (reveal tile) | `MinesCashout` |
| **Blackjack**| `BlackjackStart` | `BlackjackNext` (Hit/Stand/Split/Double) | N/A |
| **Limbo** | `LimboPlay` | N/A | N/A |
| **Tower** | `TowerStart` | `TowerNext` (climb) | `TowerCashout` |
| **Seeds** | N/A | `ChangeGameSeed` | N/A |

### Betting & Transactions

| Mutation | Description |
|---|---|
| `PlaceSportsBets` | Submit a sports betting slip (single, acca, system). |
| `SportBetCashOut` | Early cashout of a sports bet. |
| `PurchaseSingleTickets` | Buy lottery tickets. |
| `SendTip` | Transfer funds to another user. |
| `SendRain` | Distribute funds to multiple chat users at once. |

### Finance Mutations

| Mutation | Description |
|---|---|
| `Withdraw` | Request a cryptocurrency withdrawal. |
| `VaultDeposit` | Move funds to vault (secure storage). |
| `VaultWithdraw` | Move funds from vault to main wallet. |
| `ConvertConfirmOrder` | Swap between currencies. |

### Rewards & Claims

| Mutation | Description |
|---|---|
| `ClaimDailyRakeback` | Collect daily VIP rakeback rewards. |
| `RedeemPromoCode` | Apply a promotional bonus code. |
| `ClaimFreeSpinBonus` | Activate free spins for slot games. |

---

## 10. Subscriptions

Subscriptions provide real-time data streams over WebSocket connections.

| Subscription | Description |
|---|---|
| `BalanceUpdated` | User's wallet balance changes (bet, win, tip, etc.). |
| `LatestBetUpdated` | Global feed of all new bets placed. |
| `CrashGameUpdate` | Real-time multiplier updates during Crash game rounds. |
| `SportsMatchOddsUpdated` | Real-time odds changes for sports fixtures. |
| `SportsBetCanCashout` | Live cashout availability and value updates for active sports bets. |
| `NewNotification` | System messages, tips received, and important alerts. |
| `VipLevelUpdated` | Real-time updates to XP and VIP progress. |
| `TournamentScoreUpdated` | Live leaderboard updates for active tournaments. |

---

## 11. File Uploads (KYC)

The API uses the `Upload` scalar type for document submissions following the **GraphQL Multipart Request Specification**.

*   **Format:** `multipart/form-data` with `operations`, `map`, and file parts.
*   **Purpose:** KYC Level 2/3 document submissions.
*   **Supported Types:** JPG, PNG, PDF (Max 10MB).

---

## 12. Usage Examples

**(Detailed code examples for Login Flow, Mines, Sports Betting, Wallet Management, Subscriptions, US Sweepstakes, SHFL Staking, Responsible Gaming, and Live Streaming are provided in the full API reference.)**

---

## 13. Error Handling

The API returns standardized error responses following GraphQL conventions.

### Error Response Structure
```json
{
  "errors": [
    {
      "message": "Insufficient balance",
      "extensions": {
        "code": "INSUFFICIENT_BALANCE",
        "timestamp": "2025-12-12T14:30:00.000Z",
        "field": "amount"
      }
    }
  ],
  "data": null
}
```

### Platform-Specific Error Codes

| Enum Code | Meaning | User Action |
|-----------|---------|-------------|
| `E158` | **Insufficient Balance** | Deposit more funds or reduce bet amount |
| `E225` | **User Self-Excluded** | Account is under self-exclusion period. |
| `E344` | **KYC Level Too Low** | Complete higher KYC verification. |
| `E394` | **Restricted Region (Geo-block)** | Service not available in your region. |

---

## 14. Rate Limiting

API requests are rate-limited to ensure fair usage and platform stability.

| Operation Type | Limit | Window |
|----------------|-------|--------|
| **Queries** | 100 requests | Per minute |
| **Mutations** | 30 requests | Per minute |
| **Game Actions** | 60 requests | Per minute |
| **Authentication** | 5 attempts | Per 5 minutes |

Rate limit information is included in HTTP response headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`.

---

## 15. Best Practices

1.  **Request only needed fields:** Minimize payload size using precise GraphQL queries.
2.  **Handle errors gracefully:** Always check `error.graphQLErrors` for specific codes.
3.  **Use `Decimal` as a string:** Prevent floating-point issues by treating the `Decimal` scalar as a string on the client.
4.  **Subscribe wisely:** Only subscribe to necessary events and remember to unsubscribe.
5.  **Enable 2FA:** Recommend Two-Factor Authentication for all users.
6.  **Rotate seeds regularly:** Encourage users to change their client seed (see [Appendix B](#appendix-b-provably-fair-verification-guide)).

---

## 16. Provably Fair Gaming

The platform uses a cryptographic three-seed system for fairness: **Server Seed**, **Client Seed**, and **Nonce**. The game result is calculated using `HMAC-SHA256(ServerSeed, ClientSeed-Nonce)`. The Server Seed's hash is committed *before* bets, and the actual seed is only revealed when the player rotates to a new seed pair.

### Live Bet Verification Example

**(The detailed example showing the Bet Details Modal, Provably Fair Tab (Unverifiable), and Post-Rotation Verification is included in the full API reference, leading directly to Appendix B.)**

---

## 17. WebSocket Connection

Subscriptions use the WebSocket protocol (`graphql-ws`).

**Connection URL:** `wss://api.shuffle.com/graphql`

**Authentication:** Must be passed during the `connection_init` message.

```json
{
  "type": "connection_init",
  "payload": {
    "Authorization": "Bearer <access_token>"
  }
}
```

---

## 18. API Versioning

*   **Current Version:** **v2.0**
*   **Version Header:** `API-Version: v2.0`
*   **Deprecation Policy:** Deprecated fields are marked with `@deprecated` and supported for a minimum of 6 months.

---

## 19. Support & Resources

*   **API Reference:** `https://docs.shuffle.com/api`
*   **Email Support:** `api-support@shuffle.com`
*   **Status Page:** `https://status.shuffle.com`
*   **SDKs:** `@shuffle/sdk` (JS/TS), `shuffle-sdk` (Python)

---

## 20. Appendix A: Technical Specifications & Implementation Details

### 1. Input Objects Reference

| Input Object | Fields | Notes |
|---|---|---|
| `VaultDepositInput` | `currency`, `amount` | Must be positive. |
| `PlaceSportsBetsInput` | `type`, `stake`, `currency`, `selections`, `acceptOddsChanges` | `type` is `SINGLE`, `ACCUMULATOR`, or `SYSTEM`. |
| `BetSelectionInput` | `fixtureId`, `marketId`, `selectionId`, `odds` | `odds` is required for validation. |
| `CrashPlayInput` | `currency`, `amount`, `autoCashout` | `autoCashout` is an optional multiplier string. |

### 2. Standard Fragments

Standard fragments like `BetFields`, `GameBody`, and `KycFields` are recommended for consistent data fetching and UI rendering across the platform.

### 3. Workflow Diagrams

**(Authentication & Silent Refresh Flow and US Sweepstakes Logic diagrams are included in the full API reference.)**

### 4. Public vs. Protected Access

| Access Level | Header Requirement | Example Operations |
|---|---|---|
| **Public** | None | `GetLobbyGames`, `LoginRequest` |
| **Protected** | `Authorization: Bearer <token>` | `GetMyProfile`, `PlaceBet` |
| **Admin** | `Authorization: Bearer <admin_token>` | Admin functions (Internal) |

### 5. Technical Implementation Nits

*   **Decimal Handling:** Use client-side libraries (`decimal.js`, `BigNumber.js`) to parse `Decimal` strings to avoid floating-point errors.
*   **WebSocket Authentication:** Must be done during `connection_init`.
*   **GraphQL Multipart Request:** Use compliant clients (e.g., `apollo-upload-client`) for file uploads.

---

## 21. Appendix B: Provably Fair Verification Guide

### Overview

The provably fair system uses `HMAC-SHA256(ServerSeed, ClientSeed-Nonce)` to generate results, ensuring transparency and non-manipulation. The Server Seed is hashed and committed prior to play.

### 1. Bet Data Export Format

All bets can be exported in CSV format for independent verification, containing:
`ID,Game,Amount,Currency,Multiplier,Payout,ClientSeed,ServerSeed,Nonce,Timestamp`

**GraphQL Query for Export:**
```graphql
query ExportMyBets($first: Int!, $cursor: String) {
  myBets(first: $first, cursor: $cursor) {
    edges {
      # ... relevant fields including provablyFair { clientSeed, serverSeed, nonce }
    }
  }
}
```
*Note: `serverSeed` is null for bets made with the current active seed pair.*

### 2. Provably Fair Interface Components

*   **Active Client Seed:** Player-controlled string (1-64 chars), editable.
*   **Active Server Seed (hashed):** SHA-256 hash of the server seed, used for commitment proof.
*   **Total Bets Made with Pair:** Nonce counter for the current seed pair.
*   **Seed Rotation:** The `ChangeGameSeed` mutation is required to reveal the **Previous Server Seed** for verification.

### 3. URL-Based Bet Verification

Each bet has a shareable verification URL:

**Structure:** `https://shuffle.com/games/originals/{game}?md-id={betId}&modal=bet`

**Example:** `https://shuffle.com/games/originals/limbo?md-id=CW42KfxZe8aETNut7SGpQ&modal=bet`

### 4. Bet ID System

Shuffle uses a **20-character case-sensitive alphanumeric** identifier.

**Game Type Prefixes:**
| Prefix | Game | Prefix | Game | Prefix | Game |
|---|---|---|---|---|---|
| `CW` | Limbo | `DM` | Dice | `MN` | Mines |
| `CR` | Crash | `BJ` | Blackjack | `HL` | Hilo |

### 5. Complete Verification Example

The manual verification process for a Limbo bet with an actual result of `1.43x` involves:

1.  **Verify Server Seed Hash:** `SHA-256(ServerSeed) === ServerSeedHash`
2.  **Generate Result Hash:** `HMAC-SHA256(ServerSeed, "ClientSeed-Nonce")`
3.  **Convert Hash to Game Result (Limbo):**
    *   Take the first 8 hex characters of the result hash.
    *   Convert to decimal (`intValue`).
    *   Apply the formula: `Result = Math.max(1, (2^32 / (intValue + 1)) * (1 - 0.01))`

### 6. Game-Specific Verification Algorithms

Algorithms differ based on the game:

*   **Dice:** Uses the decimal conversion of the first 8 hex chars to get a result between `0.00` and `99.99`.
*   **Mines:** Uses `HMAC-SHA256` with an incrementing counter (`nonce-0`, `nonce-1`, etc.) to determine mine positions.
*   **Crash:** Uses `HMAC-SHA256(ServerSeed, PublicSeed)` and a calculation involving `2^32` to find the crash point.
*   **Blackjack:** Uses a series of HMAC calls to generate a cryptographically shuffled deck.

### 7. Verification Tools & Resources

*   **Official Tool:** `https://shuffle.com/provably-fair/verify`
*   **API Endpoint for Verification:** `query VerifyBet($betId: String!)` - returns verification data and a `matches` boolean.

### 8. Best Practices for Players

*   **Rotate Seeds:** Recommended every 100-200 bets, after a large win, or at least weekly.
*   **Client Seed Selection:** Use a long, random, unique string (e.g., `rng-f7a3b9c2e1d4`).
*   **Security:** Never share your client seed; verify a few random bets monthly.

***
