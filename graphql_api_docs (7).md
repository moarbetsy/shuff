# GraphQL API Documentation v2.0

## Overview

This GraphQL API provides access to a comprehensive gaming and betting platform with **dual-mode currency systems**: Standard Crypto (Global) and Sweepstakes (US). The platform supports user authentication, wallet management, casino game logic (Crash, Dice, Mines, etc.), sports betting with DRM-protected live streaming, lottery, SHFL token staking and governance, VIP rewards, responsible gaming controls, and real-time state updates.

### Key Features
- **Dual Currency Systems**: Standard crypto gambling (global) and US-compliant Sweepstakes model (GC/SC)
- **SHFL Tokenomics**: Staking, vesting schedules, and airdrop management
- **Responsible Gaming**: Self-exclusion, deposit limits, and loss limits
- **Live Sports Streaming**: DRM-protected video streams with Widevine/PlayReady/FairPlay
- **Provably Fair Gaming**: Cryptographic verification for all casino games
- **Real-Time Updates**: WebSocket subscriptions for live odds, game states, and notifications

## Table of Contents

- [Schema Overview](#schema-overview)
- [Scalar Types](#scalar-types)
- [Enumerations](#enumerations)
- [Authentication](#authentication)
- [Sweepstakes Model (US Compliance)](#sweepstakes-model-us-compliance)
- [SHFL Tokenomics](#shfl-tokenomics)
- [Responsible Gaming](#responsible-gaming)
- [Queries](#queries)
  - [User & Profile](#user--profile)
  - [Wallet & Finance](#wallet--finance)
  - [Casino Games](#casino-games)
  - [Sports Betting](#sports-betting)
  - [Sports Streaming (DRM)](#sports-streaming-drm)
  - [Lottery System](#lottery-system)
  - [Promotions & Rewards](#promotions--rewards)
  - [System & Config](#system--config)
- [Mutations](#mutations)
  - [Auth & Account](#auth--account)
  - [Game Actions](#game-actions)
  - [Betting & Transactions](#betting--transactions)
  - [Finance Mutations](#finance-mutations)
  - [Rewards & Claims](#rewards--claims)
- [Subscriptions](#subscriptions)
- [File Uploads (KYC)](#file-uploads-kyc)
- [Usage Examples](#usage-examples)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Best Practices](#best-practices)

---

## Schema Overview

The API is organized into three main operation types. All requests should be sent to the GraphQL endpoint (typically `/graphql`).

```graphql
schema {
  query: Query
  mutation: Mutation
  subscription: Subscription
}
```

---

## Scalar Types

| Type | Description | Example |
|------|-------------|---------|
| `Int` | Signed 32-bit integer | `42` |
| `Float` | Signed double-precision floating-point value | `3.14159` |
| `String` | UTF-8 character sequence | `"Hello World"` |
| `Boolean` | True or false | `true` |
| `ID` | Unique identifier (often serialized as a string) | `"user_123"` |
| `Decimal` | High-precision decimal for financial calculations | `123.45678900` |
| `DateTime` | ISO 8601 date-time string | `2025-12-12T14:30:00.000Z` |
| `UUID` | Universally Unique Identifier | `550e8400-e29b-41d4-a716-446655440000` |
| `JSON` | Arbitrary JSON object | `{"key": "value"}` |
| `Upload` | File upload payload for KYC documents | Binary file data |

---

## Enumerations

The API uses extensive enumerations to define game types, currencies, and states.

### Core Enums

#### CurrencyCode / Currency
Supported cryptocurrencies and sweepstakes currencies for transactions:

**Major Cryptocurrencies:**
- `BTC` / `BITCOIN` - Bitcoin
- `ETH` / `ETHEREUM` - Ethereum
- `SOL` / `SOLANA` - Solana
- `LTC` - Litecoin

**Stablecoins:**
- `USDT` - Tether
- `USDC` - USD Coin
- `BUSD` - Binance USD
- `DAI` - Dai

**Sweepstakes Currencies (US Only):**
- `GC` - Gold Coins (social currency, no monetary value)
- `SC` - Sweepstakes Coins (redeemable currency)

**Altcoins:**
- `TRX` - Tron
- `MATIC` - Polygon
- `XRP` - Ripple
- `BNB` - Binance Coin
- `DOGE` - Dogecoin
- `AVAX` - Avalanche
- `TON` - Toncoin
- `SHIB` - Shiba Inu
- `BONK` - Bonk
- `WIF` - Dogwifhat
- `SHFL` - Shuffle token (platform utility token)
- `TRUMP` - Trump token
- `PUMP` - Pump token

#### FiatCurrency
Supported fiat currencies for display and conversion:
- `USD` - United States Dollar
- `EUR` - Euro
- `JPY` - Japanese Yen
- `BRL` - Brazilian Real
- `CAD` - Canadian Dollar
- `CNY` - Chinese Yuan
- `IDR` - Indonesian Rupiah
- `INR` - Indian Rupee
- `KRW` - South Korean Won
- `MXN` - Mexican Peso
- `PHP` - Philippine Peso
- `TRY` - Turkish Lira
- `VND` - Vietnamese Dong
- `RUB` - Russian Ruble
- `ARS` - Argentine Peso
- `PLN` - Polish Złoty
- `DKK` - Danish Krone
- `NZD` - New Zealand Dollar

#### VipLevel
Hierarchical VIP tier system with 57 distinct levels including special hidden tiers:

**Entry Tier:** `UNRANKED`, `WOOD` (starter tiers)

**Bronze Tier:** `BRONZE_1`, `BRONZE_2`, `BRONZE_3`, `BRONZE_4`, `BRONZE_5`

**Silver Tier:** `SILVER_1`, `SILVER_2`, `SILVER_3`, `SILVER_4`, `SILVER_5`

**Gold Tier:** `GOLD_1`, `GOLD_2`, `GOLD_3`, `GOLD_4`, `GOLD_5`

**Platinum Tier:** `PLATINUM_1`, `PLATINUM_2`, `PLATINUM_3`, `PLATINUM_4`, `PLATINUM_5`

**Sapphire Tier:** `SAPPHIRE_1`, `SAPPHIRE_2`, `SAPPHIRE_3`, `SAPPHIRE_4`, `SAPPHIRE_5`

**Ruby Tier:** `RUBY_1`, `RUBY_2`, `RUBY_3`, `RUBY_4`, `RUBY_5`

**Diamond Tier:** `DIAMOND_1`, `DIAMOND_2`, `DIAMOND_3`, `DIAMOND_4`, `DIAMOND_5`

**Jade Tier:** `JADE_1`, `JADE_2`, `JADE_3`, `JADE_4`, `JADE_5`

**Opal Tier:** `OPAL_1`, `OPAL_2`, `OPAL_3`, `OPAL_4`, `OPAL_5`

**Dragon Tier:** `DRAGON_1`, `DRAGON_2`, `DRAGON_3`, `DRAGON_4`, `DRAGON_5`

**Elite Tiers:** `MYTHIC` (highest standard tier), `DARK` (hidden tier), `LEGEND` (special tier), `MASTER` (special tier)

#### Language
Supported interface languages:
`EN` (English), `JA` (Japanese), `ES` (Spanish), `PT` (Portuguese), `ZH` (Chinese), `KO` (Korean), `FR` (French), `DE` (German), `RU` (Russian)

### Game Specifics

#### GameType
Available casino games:
- `CRASH` - Multiplier crash game
- `DICE` - Roll under/over dice game
- `MINES` - Minesweeper-style game
- `PLINKO` - Plinko board game
- `LIMBO` - Limbo multiplier game
- `KENO` - Number selection lottery
- `HILO` - Higher or lower card game
- `BLACKJACK` - Classic blackjack
- `ROULETTE` - Roulette wheel
- `WHEEL` - Wheel of fortune
- `TOWER` - Tower climbing game

#### GameAction
Actions that can be performed during games:

**Blackjack Actions:**
- `START` - Begin a new game
- `HIT` - Request another card
- `STAND` - Keep current hand and end turn
- `DOUBLE_DOWN` - Double bet and receive one final card
- `SPLIT` - Split a pair into two hands

**General Actions:**
- `CASHOUT` - Exit game and collect winnings
- `NEXT` - Proceed to next round

**Hilo Actions:**
- `HIGHER` / `SAME_OR_ABOVE` - Bet next card is higher
- `LOWER` / `SAME_OR_BELOW` - Bet next card is lower
- `SKIP` - Skip current card

#### SportsBetStatus
Status of sports betting wagers:
- `PENDING` - Bet placed, waiting for event result
- `WON` - Bet won
- `LOST` - Bet lost
- `CASHOUT` - Early cashout taken
- `VOID` - Bet cancelled/voided
- `REFUNDED` - Stake returned

### Verification & Compliance

#### KycLevel
Know Your Customer verification levels:
- `KYC_1` - Basic verification (email, phone)
- `KYC_2` - Intermediate verification (ID document)
- `KYC_3` - Advanced verification (proof of address, enhanced due diligence)

#### RiskLevel
User risk assessment classification:
- `LOW_RISK` - Standard user with normal activity
- `MEDIUM_RISK` - User requiring monitoring
- `HIGH_RISK` - User flagged for suspicious activity

#### UsState
All 50 US states for geographical compliance:
`ALABAMA`, `ALASKA`, `ARIZONA`, `ARKANSAS`, `CALIFORNIA`, `COLORADO`, `CONNECTICUT`, `DELAWARE`, `FLORIDA`, `GEORGIA`, `HAWAII`, `IDAHO`, `ILLINOIS`, `INDIANA`, `IOWA`, `KANSAS`, `KENTUCKY`, `LOUISIANA`, `MAINE`, `MARYLAND`, `MASSACHUSETTS`, `MICHIGAN`, `MINNESOTA`, `MISSISSIPPI`, `MISSOURI`, `MONTANA`, `NEBRASKA`, `NEVADA`, `NEW_HAMPSHIRE`, `NEW_JERSEY`, `NEW_MEXICO`, `NEW_YORK`, `NORTH_CAROLINA`, `NORTH_DAKOTA`, `OHIO`, `OKLAHOMA`, `OREGON`, `PENNSYLVANIA`, `RHODE_ISLAND`, `SOUTH_CAROLINA`, `SOUTH_DAKOTA`, `TENNESSEE`, `TEXAS`, `UTAH`, `VERMONT`, `VIRGINIA`, `WASHINGTON`, `WEST_VIRGINIA`, `WISCONSIN`, `WYOMING`

### Miscellaneous Enums

#### GenericEnum
Multi-purpose enumeration containing various categories:

**Game Difficulty:** `EASY`, `MEDIUM`, `HARD`, `EXPERT`

**Roulette Betting Options:** `BLACK`, `RED`, `ODD`, `EVEN`, `FIRST`, `SECOND`, `THIRD`, `TOP`, `MIDDLE`, `BOTTOM`

**Number Ranges:** `TEN`, `TWENTY`, `THIRTY`, `FORTY`, `FIFTY`

**Game Results:** `CORRECT_GUESS`, `WRONG_GUESS`

**Insurance Options:** `BUY_INSURANCE`, `REJECT_INSURANCE`, `FORCE_CASHOUT`

**Authentication Methods:** `EMAIL_OTP`, `TOTP`

**Document Types:** `PASSPORT`, `ID_CARD`, `DRIVERS`, `RESIDENCE_PERMIT`

**Status Values:** `ACTIVE`, `COMPLETED`, `INCOMPLETE`, `REQUESTED`, `IN_REVIEW`, `IN_MANUAL_REVIEW`, `ACTION_REQUIRED`, `ADMIN_REQUIRED`

**Blockchain Networks:** `TRON`, `BINANCE_SMART_CHAIN`, `ARBITRUM`, `BASE`, `MATIC_POLYGON`, `AVAXC`

**User Roles:** `MODERATOR`

**Game Modes:** `CLASSIC`, `DARK`

**Transaction Types:** `BUY`, `SELL`, `GET`, `SKIP`

**Generic Options:** `ANY`, `NONE`, `TEAM`

**Error Codes:** `E158`, `E225`, `E344`, `E394`

---

## Authentication

Authentication is handled via Bearer tokens. You must obtain an `accessToken` via the `login` or `register` mutations.

**Required Header:**
```
Authorization: Bearer <your_access_token>
```

**Authentication Flow:**
1. Call `loginRequest` with credentials → receive `loginToken`
2. Call `login` with `loginToken` and 2FA code (if enabled) → receive `accessToken`
3. Include `accessToken` in all subsequent requests

**Token Refresh:**
Use the `refreshToken` returned during login to obtain a new `accessToken` when it expires.

---

## Sweepstakes Model (US Compliance)

The API automatically detects user region. **US users operate under a Sweepstakes model** to comply with local gambling regulations.

### Currency System

**GC (Gold Coins):**
- Social/play currency with no monetary value
- Used for entertainment and gameplay
- Cannot be redeemed for real money
- Can be purchased in packages

**SC (Sweepstakes Coins):**
- Prize/sweepstakes currency
- Can be redeemed for cryptocurrency
- Awarded as bonuses with GC purchases
- Subject to redemption minimums and KYC requirements

### Sweepstakes Queries

#### GetSweepstakesBalance
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

#### GetPurchaseLimit
US users have daily/monthly purchase limits on GC packages for consumer protection.

```graphql
query GetPurchaseLimit {
  sweepStakesPurchaseLimit {
    purchaseLimit
    currentPurchases
    nextLimitResetTime
  }
}
```

### Sweepstakes Mutations

#### RequestSweepStakesRedemption
Convert SC winnings into cryptocurrency (redemption process).

```graphql
mutation RequestRedemption($data: RedeemInputWithAuthCode!) {
  requestSweepStakesRedemption(data: $data) {
    id
    status
    amount
    currency
    address
    estimatedProcessingTime
  }
}
```

**Requirements:**
- Minimum SC balance (typically 100 SC)
- KYC Level 2 or higher
- Valid cryptocurrency wallet address
- 2FA code if enabled

#### ClaimSweepStakesDailyBonus
Daily login bonus for Sweepstakes players.

```graphql
mutation ClaimSweepstakesDaily {
  claimSweepStakesDailyBonus {
    claimStatus {
      currentStreak
      nextUnlockDate
    }
    claimedAmounts {
      gc
      sc
    }
  }
}
```

**Features:**
- Daily GC and SC bonuses
- Streak multipliers for consecutive days
- Resets at midnight UTC

---

## SHFL Tokenomics

The platform includes a comprehensive **SHFL token utility layer** for staking, governance, and rewards. SHFL token holders can stake for lottery entries and participate in airdrops.

### Token Queries

#### GetTokenInfo
Global statistics for the SHFL token ecosystem.

```graphql
query GetTokenInfo {
  tokenInfo {
    tvl # Total Value Locked in staking
    marketCap
    shflHolders
    totalBonuses
    circulatingSupply
    price
    priceChange24h
  }
}
```

#### GetMyStakingInfo
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

#### GetAirdropInfo
Check vesting status for token airdrops.

```graphql
query GetAirdropInfo($type: AirdropType!) {
  airdropInfo(airdropType: $type) {
    airdropAllocation
    tokensVested
    tokensClaimed
    tokensClaimable
    vestingSchedule {
      date
      amount
      unlocked
    }
  }
}
```

**AirdropType values:**
- `EARLY_USER` - Early platform users
- `VIP_HOLDER` - Historical VIP members
- `LIQUIDITY_PROVIDER` - LP providers
- `GOVERNANCE` - Governance participants

### Token Mutations

#### StakeShfl
Stake SHFL tokens to receive lottery entries and rewards.

```graphql
mutation StakeShfl($data: StakeShflInput!) {
  stakeShfl(data: $data) {
    stakeId
    stakedAmount
    lotteryEntries
    lockupEndsAt
    estimatedApy
  }
}
```

**StakeShflInput:**
```graphql
input StakeShflInput {
  amount: Decimal!
  lockupPeriod: Int! # Days: 30, 90, 180, 365
}
```

**Benefits:**
- Automatic lottery entries (1 entry per 100 SHFL staked)
- APY based on lockup period
- Platform fee discounts
- Governance voting power

#### UnstakeShfl
Withdraw staked tokens after lockup period.

```graphql
mutation UnstakeShfl($data: UnstakeShflInput!) {
  unstakeShfl(data: $data) {
    unstakeId
    amount
    penaltyAmount # If unstaking early
    estimatedArrival
  }
}
```

**Notes:**
- Early unstaking incurs penalties (10-30% depending on remaining time)
- Rewards are automatically claimed during unstaking
- Lottery entries are removed proportionally

#### ClaimAirdrop
Claim vested airdrop tokens.

```graphql
mutation ClaimAirdrop($type: AirdropType!) {
  claimAirdrop(type: $type) {
    claimedAmount
    remainingVested
    transactionHash
  }
}
```

---

## Responsible Gaming

The platform includes comprehensive **Responsible Gaming (RG)** features for player protection and regulatory compliance.

### Self-Exclusion

#### CreateSelfExclusion
Temporarily or permanently disable account access.

```graphql
mutation CreateSelfExclusion($data: SelfExclusionInput!) {
  confirmSelfExclusion(data: $data) {
    id
    selfExclusionType # PERMANENT, TEMPORARY
    selfExclusionUntilAt
    status
    canReverse
  }
}
```

**SelfExclusionInput:**
```graphql
input SelfExclusionInput {
  type: SelfExclusionType! # TEMPORARY, PERMANENT
  duration: Int # Required for TEMPORARY (days: 7, 30, 90, 180, 365)
  reason: String
  password: String! # Confirmation required
}
```

**Important:**
- Permanent exclusions **cannot be reversed**
- Temporary exclusions have minimum durations
- All pending bets are voided
- Withdrawals are processed before exclusion

#### CheckSelfExclusionStatus
Verify current self-exclusion state.

```graphql
query CheckSelfExclusionStatus {
  mySelfExclusion {
    isActive
    type
    expiresAt
    canLift
  }
}
```

### Deposit & Loss Limits

#### EnableResponsibleLimit
Set financial limits for responsible gambling.

```graphql
mutation EnableResponsibleLimit($data: EnableResponsibleLimitInput!) {
  enableResponsibleLimit(data: $data) {
    id
    type # DEPOSIT_LIMIT, LOSS_LIMIT, WAGER_LIMIT
    period # DAILY, WEEKLY, MONTHLY
    amount
    currency
    currentAmount
    nextResetAt
  }
}
```

**Limit Types:**
- `DEPOSIT_LIMIT` - Maximum deposits in period
- `LOSS_LIMIT` - Maximum net losses in period
- `WAGER_LIMIT` - Maximum amount wagered in period

**EnableResponsibleLimitInput:**
```graphql
input EnableResponsibleLimitInput {
  type: LimitType!
  period: LimitPeriod!
  amount: Decimal!
  currency: CurrencyCode!
}
```

#### GetMyLimits
View all active responsible gaming limits.

```graphql
query GetMyLimits {
  myResponsibleLimits {
    limits {
      type
      period
      amount
      currentAmount
      nextResetAt
    }
    canIncrease
    cooldownEndsAt
  }
}
```

**Limit Rules:**
- Decreases take effect immediately
- Increases have a 24-48 hour cooldown
- Limits persist across all currencies (converted to USD equivalent)

#### RemoveResponsibleLimit
Request removal of a limit (subject to cooldown).

```graphql
mutation RemoveResponsibleLimit($limitId: String!) {
  removeResponsibleLimit(limitId: $limitId) {
    success
    effectiveAt # When removal takes effect
  }
}
```

### Reality Checks

#### GetSessionInfo
Track session duration and wagering.

```graphql
query GetSessionInfo {
  mySessionInfo {
    sessionDuration # Minutes
    totalWagered
    netProfit
    betCount
    sessionStartedAt
  }
}
```

**Note:** The platform displays mandatory reality check popups at configured intervals (e.g., every 60 minutes).

---

## Queries

### User & Profile

Operations related to the current user's identity, stats, and settings.

#### GetMyProfile
Fetches complete profile including balances, VIP level, and XP.

```graphql
query GetMyProfile {
  myProfile {
    id
    username
    email
    vipLevel
    totalWagered
    totalWon
    balances {
      currency
      available
      vault
    }
  }
}
```

#### GetUserProfile
Public profile data of another user.

```graphql
query GetUserProfile($username: String!) {
  userProfile(username: $username) {
    username
    vipLevel
    joinedAt
    stats {
      totalBets
      biggestWin
    }
  }
}
```

#### GetUserSessions
Login history and active sessions.

```graphql
query GetUserSessions($cursor: DateTime, $first: Int, $inactive: Boolean) {
  userSessions(cursor: $cursor, first: $first, inactive: $inactive) {
    edges {
      id
      ipAddress
      userAgent
      createdAt
      lastActiveAt
      isActive
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

#### GetMyKyc
Current KYC verification status.

```graphql
query GetMyKyc {
  myKyc {
    level
    status
    verifiedAt
    documents {
      type
      status
      rejectionReason
    }
  }
}
```

#### GetMyReferralStats
Referral program statistics.

```graphql
query GetMyReferralStats {
  myReferralStats {
    totalReferrals
    activeReferrals
    totalCommissionEarned
    availableCommission
  }
}
```

#### GetMyReferees
List of users referred by your account.

```graphql
query GetMyReferees($cursor: String, $first: Int) {
  myReferees(cursor: $cursor, first: $first) {
    edges {
      username
      joinedAt
      totalWagered
      commissionEarned
    }
  }
}
```

#### GetPaginatedIgnoredUsers
List of blocked/ignored users in chat.

```graphql
query GetPaginatedIgnoredUsers($cursor: String, $first: Int) {
  paginatedIgnoredUsers(cursor: $cursor, first: $first) {
    edges {
      userId
      username
      ignoredAt
    }
  }
}
```

#### GetBanStatus
Check if user is banned from chat.

```graphql
query GetBanStatus {
  banStatus {
    isBanned
    reason
    expiresAt
  }
}
```

---

### Wallet & Finance

#### GetMyBalance
Current available wallet balances for all currencies.

```graphql
query GetMyBalance {
  myBalance {
    currency
    available
    inPlay
    vault
  }
}
```

#### GetWallets
Deposit addresses for each supported cryptocurrency.

```graphql
query GetWallets {
  wallets {
    currency
    address
    network
    qrCode
  }
}
```

#### GetDeposits
History of deposit transactions.

```graphql
query GetDeposits($cursor: String, $first: Int, $currency: CurrencyCode) {
  deposits(cursor: $cursor, first: $first, currency: $currency) {
    edges {
      id
      currency
      amount
      status
      txHash
      confirmations
      createdAt
    }
    pageInfo {
      hasNextPage
    }
  }
}
```

#### GetWithdrawals
History of withdrawal transactions.

```graphql
query GetWithdrawals($cursor: String, $first: Int, $currency: CurrencyCode) {
  withdrawals(cursor: $cursor, first: $first, currency: $currency) {
    edges {
      id
      currency
      amount
      address
      status
      txHash
      fee
      createdAt
    }
  }
}
```

#### GetTransactions
Generic transaction history (bets, wins, transfers, tips).

```graphql
query GetTransactions($cursor: String, $first: Int, $type: TransactionType) {
  transactions(cursor: $cursor, first: $first, type: $type) {
    edges {
      id
      type
      currency
      amount
      balance
      createdAt
      metadata
    }
  }
}
```

#### GetTips
History of tips sent and received.

```graphql
query GetTips($cursor: String, $first: Int, $sent: Boolean) {
  tips(cursor: $cursor, first: $first, sent: $sent) {
    edges {
      id
      from
      to
      currency
      amount
      message
      createdAt
    }
  }
}
```

#### GetMyUsdWagered
Total amount wagered converted to USD.

```graphql
query GetMyUsdWagered {
  myUsdWagered {
    total
    last24Hours
    last7Days
    last30Days
  }
}
```

#### GetShuffleUsRedeemableBalance
Sweepstakes balance eligible for redemption (US users).

```graphql
query GetShuffleUsRedeemableBalance {
  shuffleUsRedeemableBalance {
    amount
    eligible
    minimumRequired
  }
}
```

---

### Casino Games

#### GetLobbyGames
Fetches categorized games (Slots, Live, Originals).

```graphql
query GetLobbyGames {
  lobbyGames {
    originals {
      slug
      name
      thumbnail
      isNew
    }
    slots {
      slug
      name
      provider
      thumbnail
      rtp
    }
    liveGames {
      slug
      name
      provider
      thumbnail
    }
  }
}
```

#### GetGames
Search and filter games by provider, category, or tags.

```graphql
query GetGames($search: String, $provider: String, $category: String, $first: Int) {
  games(search: $search, provider: $provider, category: $category, first: $first) {
    edges {
      slug
      name
      provider
      category
      thumbnail
      tags
      rtp
    }
  }
}
```

#### GetGameBySlug
Detailed information for a specific game.

```graphql
query GetGameBySlug($slug: String!) {
  gameBySlug(slug: $slug) {
    slug
    name
    provider
    description
    rtp
    volatility
    maxWin
    thumbnail
    isFavorite
  }
}
```

#### GetGameSeeds
Current client/server seeds for provably fair verification.

```graphql
query GetGameSeeds {
  gameSeeds {
    clientSeed
    serverSeed
    serverSeedHash
    nonce
  }
}
```

#### GetBets
Global bet feed showing recent platform bets.

```graphql
query GetBets($cursor: String, $first: Int, $game: GameType) {
  bets(cursor: $cursor, first: $first, game: $game) {
    edges {
      id
      user {
        username
        vipLevel
      }
      game
      currency
      amount
      multiplier
      payout
      createdAt
    }
  }
}
```

#### GetMyBets
User's personal bet history.

```graphql
query GetMyBets($cursor: String, $first: Int, $game: GameType, $currency: CurrencyCode) {
  myBets(cursor: $cursor, first: $first, game: $game, currency: $currency) {
    edges {
      id
      game
      currency
      amount
      multiplier
      payout
      profit
      createdAt
      gameData
    }
    stats {
      totalBets
      totalWagered
      totalWon
      biggestWin
    }
  }
}
```

#### GetHighRollerBets
Feed of large bets placed on the platform.

```graphql
query GetHighRollerBets($cursor: String, $first: Int, $minAmount: Decimal) {
  highRollerBets(cursor: $cursor, first: $first, minAmount: $minAmount) {
    edges {
      id
      user {
        username
        vipLevel
      }
      game
      amount
      payout
      multiplier
    }
  }
}
```

#### GetBetInfo
Detailed result and provably fair data for a specific bet.

```graphql
query GetBetInfo($betId: String!) {
  betInfo(betId: $betId) {
    id
    game
    currency
    amount
    multiplier
    payout
    result
    provablyFair {
      clientSeed
      serverSeed
      nonce
      hash
    }
    createdAt
  }
}
```

#### GetCrashGame
Current state of the active Crash game round.

```graphql
query GetCrashGame {
  crashGame {
    id
    status
    startedAt
    multiplier
    crashedAt
    hash
  }
}
```

#### BlackjackActiveBet
Active Blackjack hand for the current user.

```graphql
query BlackjackActiveBet {
  blackjackActiveBet {
    id
    hands {
      cards
      value
      canHit
      canStand
      canDouble
      canSplit
    }
    dealerCards
    dealerValue
    status
  }
}
```

#### HiloActiveBet
Active Hilo game state.

```graphql
query HiloActiveBet {
  hiloActiveBet {
    id
    currentCard
    nextCardOptions
    multiplier
    round
    status
  }
}
```

#### GetTowerActiveBet
Active Tower game state.

```graphql
query GetTowerActiveBet {
  towerActiveBet {
    id
    difficulty
    currentLevel
    multiplier
    availablePositions
    status
  }
}
```

#### GetMinesActiveBet
Active Mines game state.

```graphql
query GetMinesActiveBet {
  minesActiveBet {
    id
    mineCount
    revealedTiles
    availablePositions
    multiplier
    status
  }
}
```

---

### Sports Betting

#### GetSports
List of available sports categories.

```graphql
query GetSports($language: Language) {
  sports(language: $language) {
    id
    name
    slug
    icon
    eventCount
  }
}
```

#### GetSportsCompetitions
Leagues and tournaments for a specific sport.

```graphql
query GetSportsCompetitions($sportId: String!, $language: Language) {
  sportsCompetitions(sportId: $sportId, language: $language) {
    id
    name
    slug
    country
    sportId
    fixtureCount
  }
}
```

#### GetSportsFixtures
Matches and events for a specific competition.

```graphql
query GetSportsFixtures($competitionId: String, $startDate: DateTime, $endDate: DateTime, $first: Int) {
  sportsFixtures(competitionId: $competitionId, startDate: $startDate, endDate: $endDate, first: $first) {
    edges {
      id
      homeTeam
      awayTeam
      startTime
      status
      score {
        home
        away
      }
      markets {
        id
        name
        selections {
          id
          name
          odds
        }
      }
    }
  }
}
```

#### GetSportsMarketInfo
Detailed odds and markets for a specific fixture.

```graphql
query GetSportsMarketInfo($fixtureId: String!) {
  sportsMarketInfo(fixtureId: $fixtureId) {
    fixtureId
    markets {
      id
      name
      type
      selections {
        id
        name
        odds
        status
      }
    }
  }
}
```

#### GetSportsBet
Details of a placed sports bet.

```graphql
query GetSportsBet($id: String!) {
  sportsBet(id: $id) {
    id
    type
    stake
    currency
    potentialPayout
    status
    selections {
      fixtureId
      marketId
      selectionId
      odds
      result
    }
    settledAt
    payout
  }
}
```

#### GetMySportsBetsTotal
Count of user's sports bets.

```graphql
query GetMySportsBetsTotal {
  mySportsBetsTotal {
    total
    pending
    settled
    won
    lost
  }
}
```

#### GetSportsMatchStates
Real-time score and status of specific matches.

```graphql
query GetSportsMatchStates($fixtureIds: [String!]!) {
  sportsMatchStates(fixtureIds: $fixtureIds) {
    fixtureId
    status
    score {
      home
      away
    }
    time
    period
  }
}
```

---

### Sports Streaming (DRM)

The platform provides **DRM-protected live video streams** for sports events, supporting Widevine, PlayReady, and FairPlay protection schemes.

#### CreateSportsStream
Request a time-limited streaming URL with DRM tokens.

```graphql
mutation CreateSportsStream($data: CreateSportsStreamInput!) {
  createSportsStream(data: $data) {
    url # HLS or DASH stream URL
    expiresAt
    providerId
    format # HLS, DASH
    drm {
      fairplay {
        licenseUri
        certificateUri
      }
      widevine {
        licenseUri
      }
      playready {
        licenseUri
      }
    }
  }
}
```

**CreateSportsStreamInput:**
```graphql
input CreateSportsStreamInput {
  fixtureId: String!
  quality: StreamQuality # HD, SD, AUTO
}
```

**Implementation Notes:**
- Streams expire after 4 hours or when event ends
- DRM license exchange must be handled by the video player
- Use Shaka Player (web) or ExoPlayer (Android) for DRM support
- Quality adjusts automatically based on bandwidth

**Example Player Setup (Shaka Player):**
```javascript
const player = new shaka.Player(videoElement);

player.configure({
  drm: {
    servers: {
      'com.widevine.alpha': streamData.drm.widevine.licenseUri,
      'com.microsoft.playready': streamData.drm.playready.licenseUri,
      'com.apple.fps': streamData.drm.fairplay.licenseUri
    }
  }
});

await player.load(streamData.url);
```

#### GetAvailableStreams
Check which fixtures have live streams available.

```graphql
query GetAvailableStreams($fixtureIds: [String!]!) {
  availableStreams(fixtureIds: $fixtureIds) {
    fixtureId
    hasStream
    streamQuality
    requiresSubscription
  }
}
```

---

### Lottery System

The platform features a comprehensive lottery system with standard and boosted ticket options.

#### GetActiveLotteryDraw
Information about the current active lottery draw.

```graphql
query GetActiveLotteryDraw {
  activeLotteryDraw {
    id
    drawDate
    prizePool
    ticketPrice
    maxNumber
    numbersPerTicket
    ticketsSold
    jackpot
  }
}
```

#### GetMyLotteryTickets
View purchased tickets for upcoming draws.

```graphql
query GetMyLotteryTickets($drawId: Float) {
  myLotteryTickets(drawId: $drawId) {
    tickets {
      id
      numbers
      drawId
      isBoosted
      multiplier
      purchasedAt
    }
    totalTickets
    totalSpent
  }
}
```

#### GetLotteryHistory
Past lottery results and winners.

```graphql
query GetLotteryHistory($first: Int, $cursor: String) {
  lotteryHistory(first: $first, cursor: $cursor) {
    edges {
      id
      drawDate
      winningNumbers
      prizePool
      winners {
        username
        matchedNumbers
        prize
      }
    }
  }
}
```

### Lottery Mutations

#### PurchaseSingleTickets
Batch purchase standard lottery tickets.

```graphql
mutation PurchaseSingleTickets($data: LotteryPurchaseTicketInput!) {
  purchaseSingleTickets(data: $data) {
    purchaseId
    ticketCount
    drawId
    totalCost
    currency
    tickets {
      id
      numbers
    }
  }
}
```

**LotteryPurchaseTicketInput:**
```graphql
input LotteryPurchaseTicketInput {
  drawId: Float!
  ticketCount: Int! # 1-100 tickets per transaction
  numbers: [[Int!]!] # Optional: manually select numbers
  currency: CurrencyCode!
}
```

**Features:**
- Quick pick (random numbers) if not specified
- Bulk purchase discounts
- Automatic entry into draw

#### PurchaseBoostedTickets
Purchase tickets with multiplier potential for increased winnings.

```graphql
mutation PurchaseBoostedTickets($data: LotteryPurchaseBoostedTicketInput!) {
  purchaseBoostedSingleTickets(data: $data) {
    purchaseId
    ticketCount
    multiplier # 2x, 5x, 10x
    totalCost
    potentialPayout
  }
}
```

**LotteryPurchaseBoostedTicketInput:**
```graphql
input LotteryPurchaseBoostedTicketInput {
  drawId: Float!
  ticketCount: Int!
  multiplier: Int! # 2, 5, or 10
  numbers: [[Int!]!]
  currency: CurrencyCode!
}
```

**Boosted Ticket Mechanics:**
- Costs 2x/5x/10x standard ticket price
- Winnings are multiplied by chosen multiplier
- Limited availability per draw
- Higher risk, higher reward

---

### Promotions & Rewards

#### GetCampaigns
Active marketing campaigns and promotions.

```graphql
query GetCampaigns {
  campaigns {
    id
    name
    description
    startDate
    endDate
    rewardType
    imageUrl
  }
}
```

#### GetVipBonus
Available VIP bonuses for the current level.

```graphql
query GetVipBonus {
  vipBonus {
    availableBonuses {
      type
      amount
      currency
      claimableAt
    }
  }
}
```

#### GetVipDailyRakeback
Daily rakeback status and claimable amount.

```graphql
query GetVipDailyRakeback {
  vipDailyRakeback {
    amount
    currency
    claimable
    nextClaimAt
  }
}
```

#### GetVipWeeklyBonus
Weekly bonus status.

```graphql
query GetVipWeeklyBonus {
  vipWeeklyBonus {
    amount
    currency
    claimable
    progress
    requiredWager
  }
}
```

#### GetVipMonthlyBonus
Monthly bonus status.

```graphql
query GetVipMonthlyBonus {
  vipMonthlyBonus {
    amount
    currency
    claimable
    progress
    requiredWager
  }
}
```

#### GetRaceLeaderBoardV2
Current wager race standings.

```graphql
query GetRaceLeaderBoardV2($raceId: String!) {
  raceLeaderBoardV2(raceId: $raceId) {
    race {
      id
      name
      startTime
      endTime
      prizePool
    }
    leaders {
      rank
      username
      wagered
      prize
    }
    myPosition {
      rank
      wagered
    }
  }
}
```

#### GetChallengeRewards
Completed challenges ready to claim.

```graphql
query GetChallengeRewards {
  challengeRewards {
    id
    name
    reward
    currency
    claimable
    completedAt
  }
}
```

#### GetLotteryDraw
Results of a specific lottery draw.

```graphql
query GetLotteryDraw($id: Float!) {
  lotteryDraw(id: $id) {
    id
    drawDate
    winningNumbers
    prizePool
    winners {
      username
      prize
      matchedNumbers
    }
  }
}
```

---

### System & Config

#### GetAppSettings
Global platform settings, limits, and configurations.

```graphql
query GetAppSettings {
  appSettings {
    minBetAmount
    maxBetAmount
    minWithdrawal
    maxWithdrawal
    kycLimits {
      level1DailyLimit
      level2DailyLimit
      level3DailyLimit
    }
    supportedCurrencies
    maintenanceMode
  }
}
```

#### GetPrices
Cryptocurrency to fiat exchange rates.

```graphql
query GetPrices($currency: FiatCurrency!) {
  prices(currency: $currency) {
    BTC
    ETH
    SOL
    LTC
    USDT
    USDC
    # ... all supported cryptocurrencies
  }
}
```

#### GetChainsCurrencies
Supported blockchain networks and withdrawal fees.

```graphql
query GetChainsCurrencies {
  chainsCurrencies {
    currency
    networks {
      name
      chainId
      withdrawalFee
      minWithdrawal
      confirmations
    }
  }
}
```

---

## Mutations

### Auth & Account

#### LoginRequest
Step 1 of login flow - validates credentials and returns login token.

```graphql
mutation LoginRequest($identity: String!, $password: String!) {
  loginRequest(identity: $identity, password: $password) {
    loginToken
    loginVerificationMethod # TOTP or EMAIL_OTP
    requiresTfa
  }
}
```

#### Login
Step 2 of login - exchanges login token and 2FA code for access token.

```graphql
mutation Login($loginToken: String!, $tfaCode: String) {
  login(loginToken: $loginToken, tfaCode: $tfaCode) {
    accessToken
    refreshToken
    user {
      id
      username
      email
    }
  }
}
```

#### RegisterAccount
Create a new user account.

```graphql
mutation RegisterAccount($data: RegisterInput!) {
  registerAccount(data: $data) {
    accessToken
    refreshToken
    user {
      id
      username
      email
    }
  }
}
```

**RegisterInput:**
```graphql
input RegisterInput {
  username: String!
  email: String!
  password: String!
  referralCode: String
  acceptedTerms: Boolean!
  dateOfBirth: DateTime
  country: String
}
```

#### LoginRegisterRequestWithGoogleOneTap
Authenticate using Google One Tap OAuth.

```graphql
mutation GoogleLogin($credentials: String!) {
  loginRegisterRequestWithGoogleOneTap(credentials: $credentials) {
    login {
      accessToken
      refreshToken
      user {
        id
        username
        email
      }
    }
    register {
      oauthToken
      emailRequired
      email
    }
  }
}
```

**Flow:**
1. User clicks Google sign-in button
2. Google returns JWT credential
3. Send credential to this mutation
4. If account exists: returns `login` with tokens
5. If new user: returns `register` with OAuth token to complete registration

**Complete Google Registration:**
```graphql
mutation CompleteGoogleRegistration($data: CompleteGoogleRegistrationInput!) {
  completeGoogleRegistration(data: $data) {
    accessToken
    refreshToken
    user {
      id
      username
      email
    }
  }
}

input CompleteGoogleRegistrationInput {
  oauthToken: String!
  username: String!
  acceptedTerms: Boolean!
  referralCode: String
}
```

#### LogoutOtherSessions
Invalidate all other active sessions except the current one.

```graphql
mutation LogoutOtherSessions {
  logoutOtherSessions {
    sessionsTerminated
    remainingSessions
  }
}
```

**Use Case:** Security measure when suspicious activity is detected.

#### ChangePassword
Update user password.

```graphql
mutation ChangePassword($currentPassword: String!, $newPassword: String!) {
  changePassword(currentPassword: $currentPassword, newPassword: $newPassword) {
    success
  }
}
```

#### ToggleTotp
Enable or disable two-factor authentication.

```graphql
mutation ToggleTotp($enable: Boolean!, $code: String) {
  toggleTotp(enable: $enable, code: $code) {
    enabled
    secret # Only returned when enabling
    qrCode # Only returned when enabling
    backupCodes
  }
}
```

#### VerifyEmail
Confirm email address with verification code.

```graphql
mutation VerifyEmail($code: String!) {
  verifyEmail(code: $code) {
    success
    verified
  }
}
```

#### UpdatePreferences
Update language, marketing consent, and other preferences.

```graphql
mutation UpdatePreferences($data: PreferencesInput!) {
  updatePreferences(data: $data) {
    language
    marketingConsent
    soundEffects
    animations
  }
}
```

#### SendKycLevel1
Submit Level 1 KYC verification.

```graphql
mutation SendKycLevel1($data: KycLevel1Input!) {
  sendKycLevel1(data: $data) {
    status
    level
  }
}
```

#### SendKycLevel2
Submit Level 2 KYC with document upload.

```graphql
mutation SendKycLevel2($data: KycLevel2Input!) {
  sendKycLevel2(data: $data) {
    status
    level
    documentsRequired
  }
}
```

#### SendKycLevel3
Submit Level 3 KYC with proof of address.

```graphql
mutation SendKycLevel3($data: KycLevel3Input!) {
  sendKycLevel3(data: $data) {
    status
    level
  }
}
```

---

### Game Actions

These mutations control the flow of "Originals" games (Crash, Dice, Mines, Blackjack, etc.).

#### Crash

**CrashPlay** - Place a bet on the Crash game.

```graphql
mutation CrashPlay($data: CrashPlayInput!) {
  crashPlay(data: $data) {
    id
    amount
    currency
    autoCashoutAt
    status
  }
}
```

**CrashCashout** - Manually cashout during an active round.

```graphql
mutation CrashCashout {
  crashCashout {
    id
    cashedOutAt
    multiplier
    payout
  }
}
```

#### Dice

**DicePlay** - Place a Dice bet.

```graphql
mutation DicePlay($data: DicePlayInput!) {
  dicePlay(data: $data) {
    id
    amount
    currency
    target
    rollOver
    result
    multiplier
    payout
  }
}
```

**DicePlayInput:**
```graphql
input DicePlayInput {
  currency: CurrencyCode!
  amount: Decimal!
  target: Float! # Number between 0-100
  rollOver: Boolean! # True = roll over target, False = roll under
}
```

#### Mines

**MinesStart** - Start a new Mines game.

```graphql
mutation MinesStart($data: MinesStartInput!) {
  minesStart(data: $data) {
    id
    active
    mineCount
    availablePositions
    revealedTiles
  }
}
```

**MinesNext** - Reveal a tile.

```graphql
mutation MinesNext($data: MinesNextInput!) {
  minesNext(data: $data) {
    id
    field # Position clicked (0-24 for 5x5 grid)
    minesResult # GEM or BOMB
    multiplier
    availablePositions # Remaining safe positions
    status # ACTIVE, WON, LOST
    revealedTiles # Array of already revealed positions
  }
}
```

**MinesNextInput:**
```graphql
input MinesNextInput {
  gameId: String!
  field: Int! # Position to reveal (0-24)
}
```

**Game Mechanics:**
- Grid is 5x5 (25 positions, indexed 0-24)
- Mine count options: 1, 3, 5, 7, 10, 15, 20, 24
- Each safe tile increases multiplier
- Hit a mine = instant loss
- More mines = higher multiplier per safe tile

**MinesCashout** - End game and collect winnings.

```graphql
mutation MinesCashout {
  minesCashout {
    id
    payout
    multiplier
    minePositions
    afterBalance
  }
}
```

**MinesAutoBet** - Configure automated betting.

```graphql
mutation MinesAutoBet($data: MinesAutoBetInput!) {
  minesAutoBet(data: $data) {
    enabled
    numberOfBets
    stopOnWin
    stopOnLoss
  }
}
```

#### Blackjack

**BlackjackStart** - Begin a new Blackjack hand.

```graphql
mutation BlackjackStart($data: BlackjackStartInput!) {
  blackjackStart(data: $data) {
    id
    hands {
      cards
      value
    }
    dealerCards
    dealerValue
    canInsure
  }
}
```

**BlackjackNext** - Perform action (Hit, Stand, Double, Split).

```graphql
mutation BlackjackNext($data: BlackjackNextInput!) {
  blackjackNext(data: $data) {
    id
    action
    hands {
      cards
      value
      status
    }
    dealerCards
    dealerValue
    payout
    status
  }
}
```

**BlackjackInsurance** - Buy insurance when dealer shows Ace.

```graphql
mutation BlackjackInsurance($accept: Boolean!) {
  blackjackInsurance(accept: $accept) {
    id
    insured
    insuranceAmount
  }
}
```

#### Hilo

**HiloStart** - Start a new Hilo game.

```graphql
mutation HiloStart($data: HiloStartInput!) {
  hiloStart(data: $data) {
    id
    currentCard
    multiplier
    round
  }
}
```

**HiloNext** - Guess higher or lower.

```graphql
mutation HiloNext($data: HiloNextInput!) {
  hiloNext(data: $data) {
    id
    guess # HIGHER or LOWER
    nextCard
    correct
    multiplier
    round
    status
  }
}
```

**HiloCashout** - End game and collect winnings.

```graphql
mutation HiloCashout {
  hiloCashout {
    id
    payout
    totalRounds
    afterBalance
  }
}
```

#### Roulette

**RoulettePlay** - Place a Roulette bet.

```graphql
mutation RoulettePlay($data: RoulettePlayInput!) {
  roulettePlay(data: $data) {
    id
    amount
    currency
    payout
    multiplier
    shuffleOriginalActions {
      action {
        roulette {
          resultRaw # Internal result value
          resultValue # Winning number (0-36)
          resultColor # RED, BLACK, GREEN
        }
      }
    }
  }
}
```

**RoulettePlayInput:**
```graphql
input RoulettePlayInput {
  currency: CurrencyCode!
  bets: [RouletteBetInput!]! # Multiple bets in one spin
}

input RouletteBetInput {
  type: RouletteBetType!
  amount: Decimal!
  numbers: [Int!] # Specific numbers for straight/split bets
}
```

**Bet Types:**
- `STRAIGHT` - Single number (35:1 payout)
- `SPLIT` - Two adjacent numbers (17:1)
- `STREET` - Row of three numbers (11:1)
- `CORNER` - Four numbers (8:1)
- `LINE` - Two rows/six numbers (5:1)
- `DOZEN` - First/Second/Third dozen (2:1)
- `COLUMN` - Top/Middle/Bottom column (2:1)
- `RED` / `BLACK` - Color bet (1:1)
- `ODD` / `EVEN` - Parity bet (1:1)
- `HIGH` / `LOW` - 1-18 or 19-36 (1:1)

#### Plinko

**PlinkoPlay** - Drop a Plinko ball.

```graphql
mutation PlinkoPlay($data: PlinkoPlayInput!) {
  plinkoPlay(data: $data) {
    id
    amount
    currency
    risk # LOW, MEDIUM, HIGH
    rows
    path
    multiplier
    payout
  }
}
```

#### Limbo

**LimboPlay** - Place a Limbo bet.

```graphql
mutation LimboPlay($data: LimboPlayInput!) {
  limboPlay(data: $data) {
    id
    amount
    currency
    target
    result
    multiplier
    payout
  }
}
```

#### Keno

**KenoPlay** - Play Keno lottery.

```graphql
mutation KenoPlay($data: KenoPlayInput!) {
  kenoPlay(data: $data) {
    id
    amount
    currency
    selectedNumbers
    drawnNumbers
    matches
    multiplier
    payout
  }
}
```

#### Tower

**TowerStart** - Start climbing the Tower.

```graphql
mutation TowerStart($data: TowerStartInput!) {
  towerStart(data: $data) {
    id
    difficulty # EASY, MEDIUM, HARD, EXPERT
    currentLevel
    availablePositions
    multiplier
    status
  }
}
```

**TowerStartInput:**
```graphql
input TowerStartInput {
  amount: Decimal!
  currency: CurrencyCode!
  difficulty: GameDifficulty! # EASY, MEDIUM, HARD, EXPERT
}
```

**Difficulty Levels:**
- `EASY` - 3 safe tiles per level, lower multipliers
- `MEDIUM` - 2 safe tiles per level, balanced risk/reward
- `HARD` - 1 safe tile per level, high multipliers
- `EXPERT` - 1 safe tile per level, maximum multipliers

**TowerNext** - Choose next tile to climb higher.

```graphql
mutation TowerNext($data: TowerNextInput!) {
  towerNext(data: $data) {
    id
    position
    correct # Boolean: did user survive?
    currentLevel
    multiplier
    status # ACTIVE, WON, LOST
    availablePositions # Next level's safe positions (if survived)
  }
}
```

**TowerNextInput:**
```graphql
input TowerNextInput {
  position: Int! # Index of tile to reveal (0-based)
}
```

**TowerCashout** - Collect winnings and end climb.

```graphql
mutation TowerCashout {
  towerCashout {
    id
    levelsCleared
    multiplier
    payout
    afterBalance
  }
}
```

#### Wheel

**WheelPlay** - Spin the Wheel.

```graphql
mutation WheelPlay($data: WheelPlayInput!) {
  wheelPlay(data: $data) {
    id
    amount
    currency
    risk # LOW, MEDIUM, HIGH
    payout
    multiplier
    shuffleOriginalActions {
      action {
        wheel {
          resultSegment # Segment index that won
          risk # Risk level used
          segments # Array of all segment multipliers
        }
      }
    }
  }
}
```

**WheelPlayInput:**
```graphql
input WheelPlayInput {
  currency: CurrencyCode!
  amount: Decimal!
  risk: RiskLevel! # LOW, MEDIUM, HIGH
}
```

**Risk Levels:**

| Risk | Segments | Multipliers | Max Win |
|------|----------|-------------|---------|
| `LOW` | 10 | 1.2x - 2x | 2x |
| `MEDIUM` | 20 | 1.5x - 5x | 5x |
| `HIGH` | 40 | 1.0x - 50x | 50x |

**Game Mechanics:**
- Wheel is divided into segments based on risk level
- Higher risk = more segments with wider multiplier range
- Each segment has equal probability
- No losing segments (minimum 1x return on HIGH)

#### Seed Management

**ChangeGameSeed** - Rotate client seed for provably fair games.

```graphql
mutation ChangeGameSeed($newClientSeed: String!) {
  changeGameSeed(newClientSeed: $newClientSeed) {
    clientSeed
    serverSeedHash
    previousServerSeed # Revealed after rotation
  }
}
```

---

### Betting & Transactions

#### PlaceSportsBets
Submit a sports betting slip (single, accumulator, or system bets).

```graphql
mutation PlaceSportsBets($data: PlaceSportsBetsInput!) {
  placeSportsBets(data: $data) {
    bet {
      id
      type # SINGLE, ACCUMULATOR, SYSTEM
      stake
      currency
      totalOddsDecimal
      potentialPayout
      status
      selections {
        id
        fixtureId
        marketId
        selectionId
        oddsNumerator
        oddsDenominator
        oddsDecimal
        status
      }
    }
    error {
      message # e.g., "Odds changed", "Market suspended"
      code
      affectedSelections
    }
  }
}
```

**PlaceSportsBetsInput:**
```graphql
input PlaceSportsBetsInput {
  type: BetType! # SINGLE, ACCUMULATOR, SYSTEM
  stake: Decimal!
  currency: CurrencyCode!
  selections: [BetSelectionInput!]!
  acceptOddsChanges: Boolean # Auto-accept minor odds changes
}

input BetSelectionInput {
  fixtureId: String!
  marketId: String!
  selectionId: String!
  odds: Decimal! # Odds at time of placing bet
}
```

**Bet Types:**
- `SINGLE` - One selection, simple bet
- `ACCUMULATOR` - Multiple selections, all must win (higher payout)
- `SYSTEM` - Partial win possible (e.g., 3/4 selections correct)

**Error Handling:**
If bet placement fails due to odds changes, the response includes:
- `error.message` - Human-readable error
- `error.affectedSelections` - Which selections have issues
- Original request can be resubmitted with new odds

#### SportBetCashOut
Early cashout of a sports bet before settlement.

```graphql
mutation SportBetCashOut($data: CashoutSportsBetInput!) {
  cashoutSportsBet(data: $data) {
    id
    amount # Cashout amount returned
    status # CASHED_OUT
    originalStake
    originalPotentialPayout
    settlement {
      payoutOddsDecimal # Effective odds for cashout
      settledAt
    }
  }
}
```

**CashoutSportsBetInput:**
```graphql
input CashoutSportsBetInput {
  betId: String!
  acceptValue: Boolean! # Must acknowledge cashout value
}
```

**Cashout Mechanics:**
- Not all bets are eligible for cashout
- Cashout value updates in real-time based on current match state
- Guaranteed payout (removes risk but caps profit)
- Subscribe to `sportsBetCanCashout` for live cashout availability

#### PurchaseSingleTickets
Buy lottery tickets.

```graphql
mutation PurchaseSingleTickets($data: PurchaseTicketsInput!) {
  purchaseSingleTickets(data: $data) {
    tickets {
      id
      numbers
      drawId
    }
    totalCost
    currency
  }
}
```

#### SendTip
Transfer funds to another user directly.

```graphql
mutation SendTip($data: TipSendInputWithAuthCode!) {
  tipSend(data: $data) {
    id
    amount
    currency
    recipient
    message
    sent
    timestamp
  }
}
```

**TipSendInputWithAuthCode:**
```graphql
input TipSendInputWithAuthCode {
  username: String! # Recipient username
  currency: CurrencyCode!
  amount: Decimal!
  message: String # Optional message (max 200 chars)
  tfaCode: String # Required if 2FA enabled
}
```

**Tipping Rules:**
- Minimum tip amount: 1 USD equivalent
- Maximum tip amount: Based on VIP level
- Recipient must have verified account
- Tips are instant and irreversible
- Fee: 0% (platform absorbed)

#### SendRain
Distribute funds to multiple chat users at once.

```graphql
mutation SendRain($data: TipRainInput!) {
  tipRain(data: $data) {
    id
    amount # Total amount distributed
    currency
    chatRoom # ENGLISH, SPANISH, etc.
    recipientCount # Number of users who received
    amountPerUser # Individual payout
    timestamp
  }
}
```

**TipRainInput:**
```graphql
input TipRainInput {
  currency: CurrencyCode!
  amount: Decimal! # Total amount to distribute
  chatRoom: ChatRoom!
  minLevel: VipLevel # Optional: minimum VIP level to receive
  tfaCode: String
}
```

**Rain Mechanics:**
- Funds distributed equally to active chat users
- Only users who chatted in last 5 minutes are eligible
- Minimum recipients: 10 users (or rain fails)
- Rain amount must be at least 10 USD equivalent
- Creates engagement in chat rooms

---

### Finance Mutations

#### Withdraw
Request a cryptocurrency withdrawal.

```graphql
mutation Withdraw($data: WithdrawalInputWithAuthCode!) {
  withdraw(data: $data) {
    id
    currency
    amount
    address
    network
    fee
    status
    estimatedArrival
  }
}
```

**WithdrawalInputWithAuthCode:**
```graphql
input WithdrawalInputWithAuthCode {
  currency: CurrencyCode!
  amount: Decimal!
  address: String!
  network: String!
  tfaCode: String # Required if 2FA enabled
}
```

#### VaultDeposit
Move funds from main wallet to vault (secure storage, cannot be used for betting).

```graphql
mutation VaultDeposit($data: VaultDepositInput!) {
  vaultDeposit(data: $data) {
    id
    currency
    amount
    afterVaultBalance
    afterAvailableBalance
    timestamp
  }
}
```

**VaultDepositInput:**
```graphql
input VaultDepositInput {
  currency: CurrencyCode!
  amount: Decimal!
}
```

**Vault Benefits:**
- Protected from accidental betting losses
- Requires deliberate action to move back to main wallet
- Still counts toward VIP progress
- Separate balance tracking

#### VaultWithdraw
Move funds from vault to main wallet.

```graphql
mutation VaultWithdraw($data: VaultWithdrawInput!) {
  vaultWithdraw(data: $data) {
    id
    currency
    amount
    afterVaultBalance
    afterAvailableBalance
    timestamp
  }
}
```

**VaultWithdrawInput:**
```graphql
input VaultWithdrawInput {
  currency: CurrencyCode!
  amount: Decimal!
}
```

#### ConvertConfirmOrder
Swap between currencies.

```graphql
mutation ConvertConfirmOrder($data: ConvertOrderInput!) {
  convertConfirmOrder(data: $data) {
    id
    fromCurrency
    toCurrency
    fromAmount
    toAmount
    rate
    fee
  }
}
```

---

### Rewards & Claims

#### ClaimDailyRakeback
Collect daily VIP rakeback rewards.

```graphql
mutation ClaimDailyRakeback {
  claimDailyRakeback {
    amount
    currency
    nextClaimAt
  }
}
```

#### ClaimRakebacks
Collect instant rakeback.

```graphql
mutation ClaimRakebacks {
  claimRakebacks {
    totalAmount
    currency
    rakebackRate
  }
}
```

#### ClaimUserReward
Claim prizes from wager races.

```graphql
mutation ClaimUserReward($raceId: String!) {
  claimUserReward(raceId: $raceId) {
    raceId
    rank
    prize
    currency
    claimed
  }
}
```

#### RedeemPromoCode
Apply a promotional bonus code.

```graphql
mutation RedeemPromoCode($data: PromotionCodeInput!) {
  redeemPromoCode(data: $data) {
    code
    rewardType
    amount
    currency
    redeemed
  }
}
```

#### ClaimFreeSpinBonus
Activate free spins for slot games.

```graphql
mutation ClaimFreeSpinBonus($bonusId: String!) {
  claimFreeSpinBonus(bonusId: $bonusId) {
    bonusId
    gameSlug
    freeSpins
    expiresAt
  }
}
```

---

## Subscriptions

Subscriptions provide real-time data streams over WebSocket connections.

### BalanceUpdated
Triggers when the user's wallet balance changes (bet placed, win, deposit, withdrawal).

```graphql
subscription BalanceUpdated {
  balanceUpdated {
    currency
    amount # New balance
    available # Spendable balance
    inPlay # Currently locked in active bets
    vault # Balance in vault
    windowId # Session identifier
    timestamp
  }
}
```

**Trigger Events:**
- Bet placed (available decreases, inPlay increases)
- Bet settled (inPlay decreases, available increases/decreases)
- Deposit confirmed (available increases)
- Withdrawal processed (available decreases)
- Vault transfer (available/vault exchange)
- Tip received (available increases)

**Usage:**
```javascript
subscription.subscribe({
  next: (data) => {
    const { currency, available, inPlay, vault } = data.balanceUpdated;
    updateWalletDisplay(currency, available, inPlay, vault);
  }
});
```

### LatestBetUpdated
Global feed of all new bets placed on the platform.

```graphql
subscription LatestBetUpdated {
  latestBetUpdated {
    id
    user {
      username
      vipLevel
    }
    game
    currency
    amount
    multiplier
    payout
    timestamp
  }
}
```

### MyBetUpdated
Stream of the current user's bets.

```graphql
subscription MyBetUpdated {
  myBetUpdated {
    id
    game
    currency
    amount
    multiplier
    payout
    profit
    timestamp
  }
}
```

### GameResult
Real-time result updates for the current game session.

```graphql
subscription GameResult {
  gameResult {
    gameId
    result
    multiplier
    payout
    timestamp
  }
}
```

### CrashGameUpdate
Real-time multiplier updates during Crash game rounds.

```graphql
subscription CrashGameUpdate {
  crashEventUpdate {
    status # STARTING, BETTING, PLAYING, CRASHED
    currentPoint # Current multiplier (e.g., 2.54)
    crashedAt # Final multiplier when crashed
    nextRoundIn # Seconds until next round starts
    hash # Provably fair game hash
    bettingTimeLeft # Seconds remaining to place bets
  }
}
```

**Game Flow:**
1. `STARTING` - Pre-game countdown
2. `BETTING` - Accepting bets (15-30 seconds)
3. `PLAYING` - Multiplier increasing, cashouts enabled
4. `CRASHED` - Round ended, showing final multiplier

**Usage Pattern:**
```javascript
subscription.subscribe({
  next: (data) => {
    const { status, currentPoint } = data.crashEventUpdate;
    if (status === 'PLAYING') {
      updateMultiplierDisplay(currentPoint);
    } else if (status === 'CRASHED') {
      showFinalResult(data.crashEventUpdate.crashedAt);
    }
  }
});
```

### NewNotification
System messages, tips received, and important alerts.

```graphql
subscription NewNotification {
  newNotification {
    id
    type # TIP_RECEIVED, SYSTEM_MESSAGE, REWARD_AVAILABLE
    title
    message
    data
    timestamp
  }
}
```

### SportsMatchOddsUpdated
Real-time odds changes for sports fixtures.

```graphql
subscription SportsMatchOddsUpdated($fixtureIds: [String!]) {
  sportsMatchOddsUpdated(fixtureIds: $fixtureIds) {
    fixtureId
    marketId
    selectionId
    oddsDecimal # New odds value
    previousOdds # Previous odds value
    change # INCREASED, DECREASED
    timestamp
  }
}
```

**Parameters:**
- `fixtureIds` - Optional array to filter specific matches
- Omit parameter to receive all odds updates (high volume)

**Usage:**
```javascript
// Subscribe to specific matches only
const subscription = client.subscribe({
  query: SPORTS_MATCH_ODDS_UPDATED,
  variables: {
    fixtureIds: ['fixture_123', 'fixture_456']
  }
});

subscription.subscribe({
  next: (data) => {
    const { fixtureId, marketId, selectionId, oddsDecimal, change } = 
      data.sportsMatchOddsUpdated;
    
    // Update odds display with visual indicator
    updateOddsDisplay(fixtureId, marketId, selectionId, oddsDecimal, change);
  }
});
```

**Best Practice:**
- Subscribe only to fixtures shown on current page
- Unsubscribe when navigating away
- Rate limit odds updates on UI (max 1 update per second per selection)

### SportsMatchStateUpdatedV2
Live score and time updates for sports matches.

```graphql
subscription SportsMatchStateUpdatedV2($fixtureId: String!) {
  sportsMatchStateUpdatedV2(fixtureId: $fixtureId) {
    fixtureId
    status
    score {
      home
      away
    }
    time
    period
    events {
      type
      player
      minute
    }
  }
}
```

### TipReceived
Notification when another user sends you a tip.

```graphql
subscription TipReceived {
  tipReceived {
    id
    from
    amount
    currency
    message
    timestamp
  }
}
```

### VipLevel
Real-time updates to XP and VIP progress.

```graphql
subscription VipLevel {
  vipLevel {
    currentLevel
    currentXp
    nextLevelXp
    progress # Percentage to next level
  }
}
```

### VipLevelUpdated
Enhanced VIP progress tracking including sweepstakes wagering.

```graphql
subscription VipLevelUpdated {
  vipLevelUpdated {
    currentLevel
    xp
    xpToNextLevel
    scWagered # Sweepstakes Coins wagered
    gcWagered # Gold Coins wagered
    cryptoWagered # Crypto wagered in USD
    progressPercent
    rewardsUnlocked {
      type
      amount
    }
  }
}
```

### SportsBetCanCashout
Real-time cashout availability and value updates for active sports bets.

```graphql
subscription SportsBetCanCashout($sportsBetId: String!) {
  sportsBetCanCashout(sportsBetId: $sportsBetId) {
    canCashout # Boolean: is cashout currently available
    reason # Why cashout is/isn't available
    cashoutOddsDecimal # Current cashout odds
    cashoutAmount # Current cashout value
    originalStake
    potentialPayout # Original potential payout
  }
}
```

**Cashout Availability:**
- `canCashout: true` - Cashout button should be enabled
- `canCashout: false` - Button disabled, check `reason` field

**Reason Codes:**
- `AVAILABLE` - Cashout is available
- `MATCH_NOT_STARTED` - Too early to cashout
- `MATCH_SUSPENDED` - Temporary suspension
- `ODDS_UNAVAILABLE` - System calculating new odds
- `TOO_CLOSE_TO_SETTLEMENT` - Match ending soon
- `ALREADY_SETTLED` - Bet already won/lost

**Usage Pattern:**
```javascript
// Subscribe when user views their bet slip
const subscription = client.subscribe({
  query: SPORTS_BET_CAN_CASHOUT,
  variables: { sportsBetId: bet.id }
});

subscription.subscribe({
  next: (data) => {
    const { canCashout, cashoutAmount } = data.sportsBetCanCashout;
    updateCashoutButton(canCashout, cashoutAmount);
  }
});

// Unsubscribe when leaving bet details page
```

### TournamentScoreUpdated
Live leaderboard updates for active tournaments and wager races.

```graphql
subscription TournamentScoreUpdated($tournamentId: String!) {
  tournamentScoreUpdated(tournamentId: $tournamentId) {
    tournamentId
    leaderboard {
      rank
      username
      score
      prize
    }
    myPosition {
      rank
      score
      percentile
    }
    endsAt
  }
}
```

---

## File Uploads (KYC)

The API uses the `Upload` scalar type for document submissions following the **GraphQL Multipart Request Specification**.

### Upload Format

KYC documents must be uploaded using multipart/form-data format with three parts:

1. **operations**: GraphQL query/mutation as JSON
2. **map**: Mapping of file placeholders to variables
3. **files**: Actual file binary data

### Example: KYC Level 2 Document Upload

**cURL Example:**
```bash
curl https://api.shuffle.com/graphql \
  -H "Authorization: Bearer $TOKEN" \
  -F operations='{ 
    "query": "mutation ($file: Upload!) { 
      kycLevel2Update(
        identityFile: $file, 
        country: \"US\", 
        idDocType: PASSPORT
      ) { 
        level 
        status 
      } 
    }", 
    "variables": { "file": null } 
  }' \
  -F map='{ "0": ["variables.file"] }' \
  -F 0=@/path/to/passport.jpg
```

**JavaScript Example (using fetch):**
```javascript
const formData = new FormData();

const operations = {
  query: `
    mutation ($file: Upload!) {
      kycLevel2Update(
        identityFile: $file
        country: "US"
        idDocType: PASSPORT
      ) {
        level
        status
      }
    }
  `,
  variables: {
    file: null
  }
};

formData.append('operations', JSON.stringify(operations));
formData.append('map', JSON.stringify({ '0': ['variables.file'] }));
formData.append('0', fileInput.files[0]);

const response = await fetch('https://api.shuffle.com/graphql', {
  method: 'POST',
  headers: {
    'Authorization': `Bearer ${accessToken}`
  },
  body: formData
});
```

### Supported File Types

| Document Type | Accepted Formats | Max Size |
|--------------|------------------|----------|
| Identity Documents | JPG, PNG, PDF | 10MB |
| Proof of Address | JPG, PNG, PDF | 10MB |
| Selfie Verification | JPG, PNG | 5MB |

### Upload Requirements

- **Image Quality**: Minimum 1200x900 pixels
- **Document Visibility**: All corners visible, no glare
- **File Names**: Use descriptive names (e.g., `passport-front.jpg`)
- **Compression**: Avoid excessive compression that reduces legibility

---

## Usage Examples

### Example 1: Complete Login Flow

```graphql
# Step 1: Initiate login
mutation Step1 {
  loginRequest(
    identity: "user@example.com"
    password: "securepassword123"
  ) {
    loginToken
    loginVerificationMethod
    requiresTfa
  }
}

# Response:
# {
#   "loginToken": "lt_abc123...",
#   "loginVerificationMethod": "TOTP",
#   "requiresTfa": true
# }

# Step 2: Complete login with 2FA
mutation Step2 {
  login(
    loginToken: "lt_abc123..."
    tfaCode: "123456"
  ) {
    accessToken
    refreshToken
    user {
      id
      username
      email
      vipLevel
    }
  }
}
```

### Example 2: Playing Mines

```graphql
# Start a new game with 3 mines
mutation StartMines {
  minesStart(data: {
    currency: BTC
    amount: 0.001
    mines: 3
  }) {
    id
    active
    mineCount
    availablePositions
  }
}

# Response:
# {
#   "id": "game_xyz789",
#   "active": true,
#   "mineCount": 3,
#   "availablePositions": [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24]
# }

# Reveal tile at position 5
mutation RevealTile {
  minesNext(data: {
    gameId: "game_xyz789"
    field: 5
  }) {
    id
    field
    minesResult # GEM or BOMB
    multiplier
    availablePositions
    status
  }
}

# Response (if safe):
# {
#   "id": "game_xyz789",
#   "field": 5,
#   "minesResult": "GEM",
#   "multiplier": 1.32,
#   "availablePositions": [0,1,2,3,4,6,7,8,9,...],
#   "status": "ACTIVE"
# }

# Cashout
mutation CashoutMines {
  minesCashout {
    id
    payout
    multiplier
    minePositions
    afterBalance
  }
}
```

### Example 3: Placing a Sports Bet

```graphql
# Get fixture and markets
query GetFixture {
  sportsMarketInfo(fixtureId: "fixture_123") {
    fixtureId
    markets {
      id
      name
      selections {
        id
        name
        odds
      }
    }
  }
}

# Place accumulator bet
mutation PlaceBet {
  placeSportsBets(data: {
    type: ACCUMULATOR
    stake: 10.00
    currency: USD
    selections: [
      {
        fixtureId: "fixture_123"
        marketId: "market_456"
        selectionId: "selection_789"
        odds: 2.50
      },
      {
        fixtureId: "fixture_124"
        marketId: "market_457"
        selectionId: "selection_790"
        odds: 1.80
      }
    ]
  }) {
    id
    type
    stake
    potentialPayout
    selections {
      fixtureId
      odds
    }
  }
}
```

### Example 4: Managing Wallet

```graphql
# Check balance
query CheckBalance {
  myBalance {
    currency
    available
    vault
  }
}

# Move funds to vault
mutation SecureFunds {
  vaultDeposit(
    currency: BTC
    amount: 0.5
  ) {
    success
    newVaultBalance
    newAvailableBalance
  }
}

# Request withdrawal
mutation WithdrawFunds {
  withdraw(data: {
    currency: BTC
    amount: 0.1
    address: "bc1q..."
    network: "Bitcoin"
    tfaCode: "123456"
  }) {
    id
    currency
    amount
    address
    status
    fee
    estimatedArrival
  }
}
```

### Example 5: Real-Time Subscriptions

```graphql
# Subscribe to balance changes
subscription WatchBalance {
  balanceUpdated {
    currency
    available
    inPlay
    vault
    timestamp
  }
}

# Subscribe to Crash game updates
subscription WatchCrash {
  crashGameUpdate {
    gameId
    status
    multiplier
    crashedAt
    timestamp
  }
}

# Subscribe to incoming tips
subscription WatchTips {
  tipReceived {
    id
    from
    amount
    currency
    message
    timestamp
  }
}
```

### Example 5: Real-Time Subscriptions

```graphql
# Subscribe to balance changes
subscription WatchBalance {
  balanceUpdated {
    currency
    available
    inPlay
    vault
    timestamp
  }
}

# Subscribe to Crash game updates
subscription WatchCrash {
  crashGameUpdate {
    gameId
    status
    multiplier
    crashedAt
    timestamp
  }
}

# Subscribe to incoming tips
subscription WatchTips {
  tipReceived {
    id
    from
    amount
    currency
    message
    timestamp
  }
}
```

### Example 6: US Sweepstakes Flow

```graphql
# Check balance (US user)
query CheckSweepstakesBalance {
  me {
    account {
      balances {
        currency # GC or SC
        amount
      }
    }
    redeemableBalance {
      redeemableAmount
      nextRedemptionDate
    }
  }
}

# Play game with Gold Coins
mutation PlayWithGC {
  dicePlay(data: {
    currency: GC
    amount: 1000
    target: 50.5
    rollOver: true
  }) {
    id
    result
    payout
  }
}

# Claim daily bonus
mutation ClaimDaily {
  claimSweepStakesDailyBonus {
    claimedAmounts {
      gc
      sc
    }
    claimStatus {
      currentStreak
    }
  }
}

# Redeem SC for cryptocurrency
mutation RedeemSC {
  requestSweepStakesRedemption(data: {
    amount: 500.00
    currency: BTC
    address: "bc1q..."
    tfaCode: "123456"
  }) {
    id
    status
    estimatedProcessingTime
  }
}
```

### Example 7: SHFL Token Staking

```graphql
# Check token info
query CheckTokenStats {
  tokenInfo {
    tvl
    price
    circulatingSupply
  }
}

# Stake SHFL for lottery entries
mutation StakeTokens {
  stakeShfl(data: {
    amount: 10000
    lockupPeriod: 90
  }) {
    stakeId
    lotteryEntries
    estimatedApy
    lockupEndsAt
  }
}

# Check airdrop allocation
query CheckAirdrop {
  airdropInfo(airdropType: EARLY_USER) {
    airdropAllocation
    tokensClaimable
    vestingSchedule {
      date
      amount
      unlocked
    }
  }
}

# Claim vested airdrop
mutation ClaimTokens {
  claimAirdrop(type: EARLY_USER) {
    claimedAmount
    transactionHash
  }
}
```

### Example 8: Responsible Gaming Setup

```graphql
# Set a weekly deposit limit
mutation SetDepositLimit {
  enableResponsibleLimit(data: {
    type: DEPOSIT_LIMIT
    period: WEEKLY
    amount: 1000.00
    currency: USD
  }) {
    id
    type
    period
    amount
    nextResetAt
  }
}

# Set a daily loss limit
mutation SetLossLimit {
  enableResponsibleLimit(data: {
    type: LOSS_LIMIT
    period: DAILY
    amount: 500.00
    currency: USD
  }) {
    id
    currentAmount
    nextResetAt
  }
}

# Check session info
query CheckSession {
  mySessionInfo {
    sessionDuration
    totalWagered
    netProfit
    betCount
  }
}

# Create temporary self-exclusion
mutation TakeBreak {
  confirmSelfExclusion(data: {
    type: TEMPORARY
    duration: 30
    reason: "Taking a break for mental health"
    password: "mypassword"
  }) {
    id
    selfExclusionUntilAt
    status
  }
}
```

### Example 9: Sports Betting with Live Streaming

```graphql
# Check if stream is available
query CheckStream {
  availableStreams(fixtureIds: ["fixture_123"]) {
    fixtureId
    hasStream
    streamQuality
  }
}

# Create stream session
mutation GetStream {
  createSportsStream(data: {
    fixtureId: "fixture_123"
    quality: HD
  }) {
    url
    expiresAt
    drm {
      widevine {
        licenseUri
      }
      fairplay {
        licenseUri
        certificateUri
      }
    }
  }
}

# Subscribe to live odds
subscription WatchOdds {
  sportsMatchOddsUpdated(fixtureId: "fixture_123") {
    marketId
    selectionId
    oldOdds
    newOdds
  }
}

# Subscribe to cashout availability
subscription WatchCashout {
  sportsBetCanCashout(betId: "bet_456") {
    betId
    canCashout
    cashoutValue
    status
  }
}
```

### Example 10: Lottery with Boosted Tickets

```graphql
# Get active draw
query GetDraw {
  activeLotteryDraw {
    id
    drawDate
    prizePool
    jackpot
    ticketPrice
  }
}

# Buy standard tickets
mutation BuyTickets {
  purchaseSingleTickets(data: {
    drawId: 1234
    ticketCount: 10
    currency: USD
  }) {
    purchaseId
    tickets {
      id
      numbers
    }
    totalCost
  }
}

# Buy boosted tickets (10x multiplier)
mutation BuyBoostedTickets {
  purchaseBoostedSingleTickets(data: {
    drawId: 1234
    ticketCount: 5
    multiplier: 10
    currency: USD
  }) {
    purchaseId
    potentialPayout
    multiplier
  }
}

# Check my tickets
query MyTickets {
  myLotteryTickets(drawId: 1234) {
    tickets {
      id
      numbers
      isBoosted
      multiplier
    }
    totalTickets
  }
}
```

---

## Error Handling

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

### Common Error Codes

| Code | Description | Resolution |
|------|-------------|------------|
| `UNAUTHENTICATED` | Missing or invalid authentication token | Re-authenticate and obtain new token |
| `FORBIDDEN` | Insufficient permissions for action | Check account verification level |
| `NOT_FOUND` | Requested resource does not exist | Verify resource ID |
| `VALIDATION_ERROR` | Invalid input data | Check input format and constraints |
| `INSUFFICIENT_BALANCE` | Not enough funds for operation | Deposit funds or reduce amount |
| `RATE_LIMIT_EXCEEDED` | Too many requests | Wait before retrying |
| `GAME_IN_PROGRESS` | Cannot start new game while one is active | Complete or cashout current game |
| `BET_TOO_LOW` | Bet amount below minimum | Increase bet amount |
| `BET_TOO_HIGH` | Bet amount exceeds maximum | Reduce bet amount |
| `KYC_REQUIRED` | Action requires higher verification | Complete KYC verification |
| `WITHDRAWAL_LIMIT_EXCEEDED` | Daily/monthly limit reached | Wait for limit reset or upgrade KYC |
| `INVALID_2FA_CODE` | Two-factor authentication code incorrect | Re-enter valid code |
| `GAME_ERROR` | Error processing game action | Retry or contact support |
| `MARKET_SUSPENDED` | Sports market temporarily unavailable | Wait for market to reopen |
| `ODDS_CHANGED` | Odds have changed since bet placement | Accept new odds or cancel |

### Platform-Specific Error Codes

The API uses specific error enum codes for precise error handling and localization:

| Enum Code | Meaning | User Action |
|-----------|---------|-------------|
| `E158` | **Insufficient Balance** | Deposit more funds or reduce bet amount |
| `E225` | **User Self-Excluded** | Account is under self-exclusion period. Contact support if this is incorrect. |
| `E344` | **KYC Level Too Low** | Complete higher KYC verification to access this feature (withdrawals, higher limits) |
| `E394` | **Restricted Region (Geo-block)** | Service not available in your region due to regulations |

**Additional Error Codes:**
- `E158` may also indicate locked/vaulted funds
- `E225` applies to both temporary and permanent exclusions
- `E344` threshold varies by action (Level 1 for basic, Level 2 for withdrawals, Level 3 for high limits)
- `E394` is permanent and cannot be bypassed with VPN usage (account may be suspended)

### Error Response Examples

**Standard Error:**
```json
{
  "errors": [
    {
      "message": "Insufficient balance to place bet",
      "extensions": {
        "code": "E158",
        "details": {
          "required": 100.00,
          "available": 75.50,
          "currency": "USD"
        }
      }
    }
  ]
}
```

**Geo-Restriction Error:**
```json
{
  "errors": [
    {
      "message": "Service not available in your region",
      "extensions": {
        "code": "E394",
        "details": {
          "detectedCountry": "US",
          "detectedState": "WA",
          "reason": "RESTRICTED_JURISDICTION"
        }
      }
    }
  ]
}
```

**Self-Exclusion Error:**
```json
{
  "errors": [
    {
      "message": "Account is self-excluded",
      "extensions": {
        "code": "E225",
        "details": {
          "type": "TEMPORARY",
          "expiresAt": "2025-06-15T00:00:00Z",
          "daysRemaining": 45
        }
      }
    }
  ]
}
```

### Handling Errors in Client Code

```javascript
// Example using Apollo Client
try {
  const { data } = await client.mutate({
    mutation: CRASH_PLAY,
    variables: { data: { currency: "BTC", amount: 0.001 } }
  });
} catch (error) {
  if (error.graphQLErrors) {
    error.graphQLErrors.forEach(({ message, extensions }) => {
      console.error(`Error: ${message}`);
      console.error(`Code: ${extensions.code}`);
      
      // Handle specific errors
      if (extensions.code === 'INSUFFICIENT_BALANCE') {
        showDepositModal();
      } else if (extensions.code === 'KYC_REQUIRED') {
        redirectToKycPage();
      }
    });
  }
  
  if (error.networkError) {
    console.error('Network error:', error.networkError);
  }
}
```

---

## Rate Limiting

API requests are rate-limited to ensure fair usage and platform stability.

### Rate Limits by Operation Type

| Operation Type | Limit | Window |
|----------------|-------|--------|
| **Queries** | 100 requests | Per minute |
| **Mutations** | 30 requests | Per minute |
| **Subscriptions** | 10 concurrent | Per user |
| **Game Actions** | 60 requests | Per minute |
| **Authentication** | 5 attempts | Per 5 minutes |

### Rate Limit Headers

Rate limit information is included in HTTP response headers:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1702392000
```

### Handling Rate Limits

When rate limit is exceeded, the API returns:

```json
{
  "errors": [
    {
      "message": "Rate limit exceeded",
      "extensions": {
        "code": "RATE_LIMIT_EXCEEDED",
        "retryAfter": 45
      }
    }
  ]
}
```

Implement exponential backoff in your client:

```javascript
async function requestWithRetry(operation, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return await operation();
    } catch (error) {
      if (error.extensions?.code === 'RATE_LIMIT_EXCEEDED') {
        const delay = error.extensions.retryAfter * 1000;
        await new Promise(resolve => setTimeout(resolve, delay));
        continue;
      }
      throw error;
    }
  }
  throw new Error('Max retries exceeded');
}
```

---

## Best Practices

### 1. Authentication & Security

- **Store tokens securely**: Never expose access tokens in client-side code or URLs
- **Use HTTPS only**: All API requests must use secure connections
- **Enable 2FA**: Strongly recommended for all users, especially for withdrawals
- **Rotate seeds regularly**: Change client seed periodically for provably fair games

### 2. Efficient Querying

- **Request only needed fields**: GraphQL allows precise field selection
- **Use pagination**: Always paginate large result sets
- **Batch related queries**: Combine multiple queries when possible
- **Subscribe wisely**: Only subscribe to events you actively need

Example of efficient querying:

```graphql
# Bad - requesting everything
query GetUser {
  myProfile {
    id
    username
    email
    firstName
    lastName
    # ... 50+ fields
  }
}

# Good - requesting only what's needed
query GetUser {
  myProfile {
    id
    username
    vipLevel
    balances {
      currency
      available
    }
  }
}
```

### 3. Error Handling

- **Always handle errors gracefully**: Never assume operations will succeed
- **Show user-friendly messages**: Translate error codes to readable messages
- **Implement retry logic**: Retry transient failures with backoff
- **Log errors for debugging**: Track errors for troubleshooting

### 4. Real-Time Features

- **Manage WebSocket connections**: Properly open, maintain, and close connections
- **Handle reconnections**: Implement automatic reconnection with backoff
- **Unsubscribe when done**: Clean up subscriptions to free resources

```javascript
// Example subscription management
const subscription = client.subscribe({
  query: CRASH_GAME_UPDATE
}).subscribe({
  next: (data) => updateGameState(data),
  error: (error) => handleError(error),
  complete: () => cleanupSubscription()
});

// Unsubscribe when component unmounts
onUnmount(() => subscription.unsubscribe());
```

### 5. Game Operations

- **Check balance before betting**: Verify sufficient funds before placing bets
- **Validate inputs**: Ensure bet amounts are within min/max limits
- **Handle game state**: Always check for active games before starting new ones
- **Implement timeouts**: Set reasonable timeouts for game operations

### 6. Sports Betting

- **Monitor odds changes**: Subscribe to odds updates for live betting
- **Validate selections**: Ensure markets are still open before placing bets
- **Handle odds acceptance**: Implement UI for accepting odds changes
- **Track bet status**: Subscribe to match state updates for live tracking

---

## Provably Fair Gaming

The platform uses cryptographic hashing to ensure game fairness and transparency.

### How It Works

1. **Server Seed**: Generated by server, hash is shown to player before game
2. **Client Seed**: Set by player, can be changed anytime
3. **Nonce**: Increments with each bet
4. **Result**: Calculated using `Hash(ServerSeed + ClientSeed + Nonce)`

### Verifying Game Results

```graphql
query VerifyBet($betId: String!) {
  betInfo(betId: $betId) {
    id
    game
    result
    provablyFair {
      clientSeed
      serverSeed # Revealed after seed change
      nonce
      hash
    }
  }
}
```

Players can independently verify results using the provided seeds and nonce.

### Changing Seeds

```graphql
mutation ChangeSeed {
  changeGameSeed(newClientSeed: "my-new-random-seed-123") {
    clientSeed
    serverSeedHash
    previousServerSeed # Previous seed is revealed
  }
}
```

**Best Practice**: Change your client seed regularly and after big wins.

---

## WebSocket Connection

Subscriptions use WebSocket protocol for real-time communication.

### Connection URL

```
wss://api.example.com/graphql
```

### Connection Payload

```json
{
  "type": "connection_init",
  "payload": {
    "Authorization": "Bearer <access_token>"
  }
}
```

### Example with Apollo Client

```javascript
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';

const wsLink = new GraphQLWsLink(
  createClient({
    url: 'wss://api.example.com/graphql',
    connectionParams: {
      Authorization: `Bearer ${accessToken}`
    }
  })
);
```

---

## API Versioning

The current API version is **v1**. Breaking changes will be introduced in new versions with advance notice.

### Version Header

```
API-Version: v1
```

### Deprecation Policy

- Deprecated fields will be marked with `@deprecated` directive
- Deprecated fields will be supported for minimum 6 months
- Migration guides will be provided for breaking changes

---

## Support & Resources

### Documentation
- **API Reference**: https://docs.example.com/api
- **GraphQL Playground**: https://api.example.com/graphql (development only)
- **Changelog**: https://docs.example.com/changelog

### Support Channels
- **Email**: api-support@example.com
- **Discord**: https://discord.gg/example
- **Status Page**: https://status.example.com

### SDKs & Libraries
- **JavaScript/TypeScript**: `npm install @example/sdk`
- **Python**: `pip install example-sdk`
- **Go**: `go get github.com/example/sdk-go`

---

## Advanced Integration Topics

### Multi-Currency Architecture

The platform supports three distinct currency modes:

**1. Standard Crypto Mode (Global)**
- Direct cryptocurrency deposits and withdrawals
- Instant settlement on blockchain
- All major cryptocurrencies supported
- Used by players outside restricted jurisdictions

**2. Sweepstakes Mode (US)**
- GC (Gold Coins) for entertainment
- SC (Sweepstakes Coins) redeemable for crypto
- Complies with US sweepstakes regulations
- Separate balance tracking and game logic

**3. SHFL Token Integration**
- Utility token for platform governance
- Staking rewards and lottery entries
- Vesting schedules for airdrops
- Fee discounts for holders

### Session Management

**WebSocket Connection Lifecycle:**
```javascript
// 1. Establish connection with auth
const wsClient = createClient({
  url: 'wss://api.shuffle.com/graphql',
  connectionParams: {
    Authorization: `Bearer ${accessToken}`
  }
});

// 2. Subscribe to essential updates
const balanceSub = wsClient.subscribe({ query: BALANCE_UPDATED });
const notificationSub = wsClient.subscribe({ query: NEW_NOTIFICATION });

// 3. Handle reconnection
wsClient.on('closed', () => {
  setTimeout(() => wsClient.reconnect(), 5000);
});

// 4. Clean up on logout
function logout() {
  balanceSub.unsubscribe();
  notificationSub.unsubscribe();
  wsClient.dispose();
}
```

### Provably Fair Verification

All "Originals" games use a cryptographic verification system:

**Verification Process:**
1. Server generates random seed, shares hash with player
2. Player sets client seed (can be changed anytime)
3. Each bet increments nonce counter
4. Result = `Hash(ServerSeed + ClientSeed + Nonce)`
5. After seed rotation, previous server seed is revealed

**Implementation:**
```javascript
// Verify a game result
async function verifyResult(betId) {
  const bet = await client.query({
    query: GET_BET_INFO,
    variables: { betId }
  });
  
  const { serverSeed, clientSeed, nonce } = bet.provablyFair;
  const calculatedHash = sha256(`${serverSeed}${clientSeed}${nonce}`);
  
  // Compare calculated result with actual result
  return calculatedHash === bet.result;
}

// Change seed for new games
await client.mutate({
  mutation: CHANGE_GAME_SEED,
  variables: { newClientSeed: generateRandomString() }
});
```

### State Synchronization

**Handling Concurrent Updates:**
```javascript
// Use optimistic UI updates
const [balance, setBalance] = useState(initialBalance);

// 1. Optimistic update
function placeBet(amount) {
  setBalance(balance - amount); // Immediate UI update
  
  client.mutate({ mutation: PLACE_BET })
    .catch(() => {
      setBalance(balance); // Rollback on error
    });
}

// 2. Subscribe to authoritative updates
useSubscription(BALANCE_UPDATED, {
  onData: ({ data }) => {
    setBalance(data.balanceUpdated.available); // Server is source of truth
  }
});
```

### Error Recovery Patterns

**Idempotent Operations:**
```javascript
// Use idempotency keys for critical operations
async function safeWithdraw(params) {
  const idempotencyKey = `withdraw-${Date.now()}-${randomUUID()}`;
  
  try {
    return await client.mutate({
      mutation: WITHDRAW,
      variables: { ...params },
      context: {
        headers: { 'Idempotency-Key': idempotencyKey }
      }
    });
  } catch (error) {
    if (error.code === 'DUPLICATE_REQUEST') {
      // Safely retry - idempotency prevents double withdrawal
      return await checkWithdrawalStatus(idempotencyKey);
    }
    throw error;
  }
}
```

**Circuit Breaker Pattern:**
```javascript
class ApiCircuitBreaker {
  constructor() {
    this.failures = 0;
    this.threshold = 5;
    this.timeout = 60000;
    this.state = 'CLOSED';
  }
  
  async call(fn) {
    if (this.state === 'OPEN') {
      throw new Error('Circuit breaker is OPEN');
    }
    
    try {
      const result = await fn();
      this.onSuccess();
      return result;
    } catch (error) {
      this.onFailure();
      throw error;
    }
  }
  
  onSuccess() {
    this.failures = 0;
    this.state = 'CLOSED';
  }
  
  onFailure() {
    this.failures++;
    if (this.failures >= this.threshold) {
      this.state = 'OPEN';
      setTimeout(() => this.state = 'HALF_OPEN', this.timeout);
    }
  }
}
```

### Performance Optimization

**Query Batching:**
```javascript
// Bad: Multiple sequential queries
const user = await client.query({ query: GET_USER });
const balance = await client.query({ query: GET_BALANCE });
const bets = await client.query({ query: GET_MY_BETS });

// Good: Single batched query
const data = await client.query({
  query: gql`
    query Dashboard {
      user { id username vipLevel }
      balance { currency amount }
      myBets(first: 10) { edges { id game amount } }
    }
  `
});
```

**Pagination Strategy:**
```javascript
// Use cursor-based pagination for large datasets
async function loadAllBets() {
  let cursor = null;
  const allBets = [];
  
  while (true) {
    const { data } = await client.query({
      query: GET_MY_BETS,
      variables: { first: 50, cursor }
    });
    
    allBets.push(...data.myBets.edges);
    
    if (!data.myBets.pageInfo.hasNextPage) break;
    cursor = data.myBets.pageInfo.endCursor;
  }
  
  return allBets;
}
```

### Security Best Practices

**Token Management:**
```javascript
// Secure token storage
class TokenManager {
  static setTokens(accessToken, refreshToken) {
    // Never store in localStorage - use httpOnly cookies or secure storage
    sessionStorage.setItem('access_token', accessToken);
    sessionStorage.setItem('refresh_token', refreshToken);
  }
  
  static async getAccessToken() {
    const token = sessionStorage.getItem('access_token');
    
    if (this.isTokenExpired(token)) {
      return await this.refreshAccessToken();
    }
    
    return token;
  }
  
  static async refreshAccessToken() {
    const refreshToken = sessionStorage.getItem('refresh_token');
    const { data } = await fetch('/auth/refresh', {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${refreshToken}` }
    });
    
    this.setTokens(data.accessToken, data.refreshToken);
    return data.accessToken;
  }
}
```

**Input Sanitization:**
```javascript
// Always validate user input before sending to API
function sanitizeBetAmount(amount) {
  const parsed = parseFloat(amount);
  
  if (isNaN(parsed) || parsed <= 0) {
    throw new Error('Invalid bet amount');
  }
  
  if (parsed > MAX_BET_AMOUNT) {
    throw new Error('Bet exceeds maximum');
  }
  
  // Round to appropriate decimal places
  return Math.round(parsed * 100000000) / 100000000;
}
```

### Compliance & Responsible Gaming

**Implementing Time Limits:**
```javascript
// Track session duration
class SessionTracker {
  constructor() {
    this.startTime = Date.now();
    this.reminderInterval = 60 * 60 * 1000; // 60 minutes
    this.setupReminders();
  }
  
  setupReminders() {
    setInterval(() => {
      const duration = Date.now() - this.startTime;
      const hours = Math.floor(duration / (60 * 60 * 1000));
      
      // Show reality check
      showRealityCheck({
        sessionDuration: duration,
        totalWagered: this.getTotalWagered(),
        netProfit: this.getNetProfit()
      });
    }, this.reminderInterval);
  }
}
```

**Geo-Restriction Check:**
```javascript
// Check user location before showing features
async function checkFeatureAvailability(feature) {
  try {
    const { data } = await client.query({ query: GET_MY_PROFILE });
    
    if (data.me.country === 'US' && feature === 'crypto_deposit') {
      // Redirect to sweepstakes purchase
      return { available: false, alternative: 'sweepstakes' };
    }
    
    return { available: true };
  } catch (error) {
    if (error.extensions?.code === 'E394') {
      // Geo-blocked
      showGeoRestrictionMessage();
      return { available: false };
    }
  }
}
```

---

**API Version**: 2.0  
**Documentation Version**: 2.0.0  
**Last Updated**: December 12, 2025  
**Maintained by**: Platform Engineering Team

### Changelog

**Version 2.0** (December 2025)
- Added Sweepstakes model support for US compliance (GC/SC currencies)
- Integrated SHFL token staking, vesting, and airdrop system
- Implemented comprehensive Responsible Gaming controls (self-exclusion, limits)
- Added DRM-protected live sports streaming with Widevine/FairPlay/PlayReady
- Enhanced lottery system with boosted ticket multipliers
- Added Google One Tap OAuth authentication
- Introduced platform-specific error codes (E158, E225, E344, E394)
- Added session management and multi-device logout
- Enhanced VIP subscription with sweepstakes wagering tracking
- Added tournament leaderboard live subscriptions
- Documented GraphQL multipart upload specification for KYC

**Version 1.0** (November 2025)
- Initial API documentation release
- Core casino games, sports betting, and wallet operations
- Basic authentication and KYC flows