# Product Requirements Document: `semrush-rs` — A Rust CLI for the Semrush API

**Version:** 1.0.0
**Date:** 2026-03-05
**Author:** [Your Name]
**Status:** Draft

***

## Executive Summary

`semrush-rs` is a high-performance, agent-friendly command-line interface built in Rust that provides comprehensive access to the Semrush API ecosystem. It is designed as a first-class tool for AI agents operating from the command line, while remaining fully usable by humans. The CLI wraps the complete Semrush API surface — including Analytics v3, Trends v3, Backlinks v1, and Projects v4 — with structured JSON output, built-in rate limiting, local caching, and an optional MCP (Model Context Protocol) server mode for direct integration with AI agent frameworks.[^1][^2]

Existing tools in this space are incomplete TypeScript MCP servers or abandoned Ruby gems that cover only a fraction of the API. Users report frustration with Semrush's expensive API access, CSV-only responses in v3, data accuracy issues, and the lack of any official CLI. `semrush-rs` solves all of these by providing a single, fast, well-documented binary that AI agents and humans can use interchangeably.[^3][^2][^4][^5][^6][^7]

***

## Problem Statement

### Current Landscape

The Semrush API is a powerful but underserved programmatic interface to one of the largest SEO datasets in the world — 26.4B keywords, 808M domain profiles, 43T backlinks, and 142 geo databases. However, there is no official CLI, and existing community tools have significant gaps:[^8]

| Tool | Language | Coverage | Status | Limitations |
|------|----------|----------|--------|-------------|
| semrush-mcp (mrkooblu) | TypeScript | ~7 tools | Active | Domain overview, keywords, backlinks only[^1] |
| semrush-mcp (metehan777) | TypeScript | ~7 tools | Active | Same limited scope, no Trends or v4 APIs[^2] |
| arambert/semrush | Ruby | Partial | Unmaintained | Last meaningful update years ago[^3] |
| Composio MCP | TypeScript | ~37 tools | Active | Vendor-locked, requires Composio platform[^9] |

### User Pain Points (Research Findings)

Extensive analysis of user feedback on Reddit, LinkedIn, and review sites reveals recurring frustrations:[^6][^10][^7]

1. **No CLI or scriptable access**: Everything requires the web UI or custom HTTP scripting. Agents and automation pipelines have no turnkey solution.
2. **CSV-only v3 responses**: The Analytics v3 API returns only CSV, making parsing cumbersome for both agents and developers.[^11][^12]
3. **Expensive and confusing pricing**: API access requires a Business subscription plus purchased API units. Users report paying $400+/month and still hitting export limits.[^4][^5]
4. **Rate limiting friction**: A hard limit of 10 requests/second with no built-in backoff guidance causes frequent issues in automation.[^13][^14]
5. **Data discrepancies**: API results frequently differ from the Semrush UI, causing confusion and trust issues.[^10][^11]
6. **Steep learning curve**: Semrush has 55+ tools and the API has dozens of report types with cryptic column codes (e.g., `Nq`, `Cp`, `Kd`).[^15]
7. **Missing modern integrations**: No native MCP server, no structured JSON output, no agent-friendly interface.[^2][^1]

### Opportunity

Build a Rust CLI that is:
- **Complete**: Covers every public Semrush API endpoint (v3 Analytics, v3 Trends, v1 Backlinks, v4 Projects/Local)
- **Agent-first**: Designed for AI agents with structured JSON output, MCP server mode, and machine-parseable responses
- **Human-friendly**: Intuitive subcommand structure, shell completions, rich table output, and comprehensive help
- **Performant**: Async I/O with Tokio, built-in rate limiting, response caching, and concurrent batch requests

***

## Target Users

### Primary: AI Agents and LLM Tool Use
- AI coding agents (Claude Code, Cursor, Windsurf) that need to query SEO data programmatically
- MCP-compatible agent frameworks that consume tools via JSON-RPC
- Automation pipelines (CI/CD, cron jobs, GitHub Actions) that incorporate SEO checks

### Secondary: SEO Professionals and Developers
- Technical SEOs who prefer command-line workflows
- Developers building SEO dashboards or internal tools
- Agency developers automating client reporting

***

## Technical Architecture

### Language and Core Dependencies

| Crate | Purpose | Version |
|-------|---------|---------|
| `clap` (derive) | Command-line argument parsing with subcommands[^16][^17] | 4.x |
| `tokio` | Async runtime for concurrent HTTP requests[^18] | 1.x |
| `reqwest` | Async HTTP client with connection pooling | 0.12.x |
| `serde` / `serde_json` | JSON serialization/deserialization | 1.x |
| `csv` | Parse Semrush v3 CSV responses into structured data | 1.x |
| `tabled` | Pretty table output for human-readable mode | 0.x |
| `directories` | XDG-compliant config and cache paths | 5.x |
| `tracing` | Structured logging and diagnostics | 0.1.x |
| `governor` | Token-bucket rate limiter | 0.6.x |
| `keyring` | Secure credential storage (OS keychain) | 2.x |
| `indicatif` | Progress bars for long-running operations | 0.17.x |
| `anyhow` / `thiserror` | Error handling | 1.x |

### Project Structure

```
semrush-rs/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE                        # MIT or Apache-2.0
├── src/
│   ├── main.rs                    # Entry point, clap parsing
│   ├── cli/
│   │   ├── mod.rs                 # CLI module root
│   │   ├── domain.rs              # Domain subcommands
│   │   ├── keyword.rs             # Keyword subcommands
│   │   ├── backlink.rs            # Backlink subcommands
│   │   ├── trends.rs              # Trends / Traffic Analytics subcommands
│   │   ├── overview.rs            # Overview report subcommands
│   │   ├── project.rs             # Project management (v4) subcommands
│   │   ├── local.rs               # Listing Management & Map Rank Tracker
│   │   ├── account.rs             # API balance, auth
│   │   └── batch.rs               # Batch operations
│   ├── api/
│   │   ├── mod.rs                 # API module root
│   │   ├── client.rs              # HTTP client wrapper with rate limiting
│   │   ├── auth.rs                # API key (v3) and OAuth2 (v4) auth
│   │   ├── v3_analytics.rs        # Analytics v3 request builders
│   │   ├── v3_trends.rs           # Trends v3 request builders
│   │   ├── v1_backlinks.rs        # Backlinks v1 request builders
│   │   ├── v4_projects.rs         # Projects v4 request builders
│   │   ├── v4_local.rs            # Listing Management + Map Rank Tracker
│   │   ├── csv_parser.rs          # CSV-to-struct parser for v3 responses
│   │   ├── columns.rs             # Column code mapping (Ph -> Keyword, etc.)
│   │   └── rate_limiter.rs        # Token-bucket rate limiter (10 req/sec)
│   ├── models/
│   │   ├── mod.rs
│   │   ├── domain.rs              # Domain report structs
│   │   ├── keyword.rs             # Keyword report structs
│   │   ├── backlink.rs            # Backlink report structs
│   │   ├── trends.rs              # Traffic analytics structs
│   │   ├── overview.rs            # Overview report structs
│   │   ├── project.rs             # Project structs
│   │   ├── local.rs               # Listing Management structs
│   │   └── common.rs              # Shared types (Database, DeviceType, etc.)
│   ├── output/
│   │   ├── mod.rs
│   │   ├── json.rs                # JSON output formatter (default for agents)
│   │   ├── table.rs               # Pretty table formatter (default for humans)
│   │   ├── csv.rs                 # CSV passthrough or re-export
│   │   └── jsonl.rs               # JSON Lines for streaming
│   ├── cache/
│   │   ├── mod.rs
│   │   └── disk.rs                # File-based response cache with TTL
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs            # Config file parsing (TOML)
│   ├── mcp/
│   │   ├── mod.rs
│   │   └── server.rs              # MCP server mode (JSON-RPC over stdio)
│   └── error.rs                   # Unified error types
├── tests/
│   ├── integration/               # Integration tests with mock server
│   └── fixtures/                  # Sample API responses
├── completions/                   # Generated shell completions
│   ├── semrush.bash
│   ├── semrush.zsh
│   ├── semrush.fish
│   └── _semrush.ps1
└── docs/
    ├── USAGE.md
    ├── MCP.md                     # MCP server documentation
    └── COLUMN_REFERENCE.md        # Human-readable column code reference
```

***

## Command-Line Interface Design

### Global Options

```
semrush [OPTIONS] <COMMAND>

Options:
  --api-key <KEY>            Semrush API key (overrides config/env)
  --output <FORMAT>          Output format: json (default), table, csv, jsonl
  --database <DB>            Regional database (e.g., us, uk, de, mobile-us) [default: us]
  --limit <N>                Max results to return [default: 100]
  --offset <N>               Skip first N results [default: 0]
  --date <YYYYMM15>          Historical date for the report
  --no-cache                 Bypass local cache
  --cache-ttl <SECONDS>      Cache TTL override [default: 3600]
  --verbose                  Enable debug logging
  --quiet                    Suppress non-data output
  --config <PATH>            Config file path
  -h, --help                 Print help
  -V, --version              Print version
```

### Command Tree

The CLI is organized into subcommand groups that map to the Semrush API's logical groupings. Every public API endpoint is covered.[^19][^20][^8]

#### `semrush domain` — Domain Analytics

```bash
semrush domain overview <DOMAIN>                  # domain_ranks / domain_rank
semrush domain overview --all-databases <DOMAIN>  # domain_ranks (all databases)
semrush domain overview --history <DOMAIN>        # domain_rank_history
semrush domain organic <DOMAIN>                   # domain_organic
semrush domain organic --positions new <DOMAIN>   # domain_organic (display_positions=new)
semrush domain organic --positions lost <DOMAIN>  # domain_organic (display_positions=lost)
semrush domain paid <DOMAIN>                      # domain_adwords
semrush domain ads-copies <DOMAIN>                # domain_adwords_unique
semrush domain ad-history <DOMAIN>                # domain_adwords_historical
semrush domain competitors organic <DOMAIN>       # domain_organic_organic
semrush domain competitors paid <DOMAIN>          # domain_adwords_adwords
semrush domain pla-keywords <DOMAIN>              # domain_shopping
semrush domain pla-copies <DOMAIN>                # domain_shopping_unique
semrush domain pla-competitors <DOMAIN>           # domain_shopping_shopping
semrush domain pages <DOMAIN>                     # domain_organic_unique
semrush domain subdomains <DOMAIN>                # domain_organic_subdomains
semrush domain compare <D1> <D2> [D3] [D4] [D5]  # domain_domains (keyword gap)
  --mode shared|all|unique|untapped|missing|exclusive
  --type organic|paid
```

#### `semrush keyword` — Keyword Research

```bash
semrush keyword overview <PHRASE>                 # phrase_this (one database)
semrush keyword overview --all-databases <PHRASE> # phrase_all
semrush keyword batch <PHRASE1> <PHRASE2> ...      # phrase_these (up to 100)
semrush keyword organic <PHRASE>                  # phrase_organic
semrush keyword paid <PHRASE>                     # phrase_adwords
semrush keyword related <PHRASE>                  # phrase_related
semrush keyword broad-match <PHRASE>              # phrase_fullsearch
semrush keyword questions <PHRASE>                # phrase_questions
semrush keyword difficulty <PHRASE>               # phrase_kdi
semrush keyword ad-history <PHRASE>               # phrase_adwords_historical
```

#### `semrush backlink` — Backlink Analytics

All backlink endpoints use `https://api.semrush.com/analytics/v1/`:[^8]

```bash
semrush backlink overview <TARGET>                # backlinks_overview
  --target-type root_domain|domain|url
semrush backlink list <TARGET>                    # backlinks
  --target-type root_domain|domain|url
  --filter type=<text|image|form|frame>
  --filter newlink=true|false
  --filter lostlink=true|false
semrush backlink referring-domains <TARGET>       # backlinks_refdomains
semrush backlink referring-ips <TARGET>           # backlinks_refips
semrush backlink tld-distribution <TARGET>        # backlinks_tld
semrush backlink geo <TARGET>                     # backlinks_geo
semrush backlink anchors <TARGET>                 # backlinks_anchors
semrush backlink indexed-pages <TARGET>           # backlinks_pages
semrush backlink competitors <TARGET>             # backlinks_competitors
semrush backlink compare <T1> <T2> [T3...]        # backlinks_matrix
semrush backlink batch <T1> <T2> [T3...]          # backlinks_comparison (up to 200)
semrush backlink authority-score <TARGET>         # backlinks_ascore_profile
semrush backlink categories <TARGET>              # backlinks_categories
semrush backlink category-profile <TARGET>        # backlinks_categories_profile
semrush backlink history <TARGET>                 # backlinks_historical
```

#### `semrush trends` — Traffic Analytics (.Trends API)

All trends endpoints use `https://api.semrush.com/analytics/ta/api/v3/`:[^20]

```bash
semrush trends summary <DOMAIN1> [DOMAIN2...]     # /summary (up to 200 targets)
  --device desktop|mobile
  --country <ISO_CODE>
semrush trends daily <DOMAIN>                     # /summary_by_day
  --forecast                                      # include_forecasted_items=true
semrush trends weekly <DOMAIN>                    # /summary_by_week
  --forecast
semrush trends sources <DOMAIN>                   # /sources
  --channel direct|referral|search|social|mail|display_ad|ai_assistants|ai_search
  --traffic-type organic|paid
semrush trends destinations <DOMAIN>              # /destinations
semrush trends geo <DOMAIN>                       # /geo
  --geo-type country|subcontinent|continent
semrush trends subdomains <DOMAIN>                # /subdomains
semrush trends top-pages <DOMAIN>                 # /toppages
semrush trends rank                               # /rank (top domains by traffic)
  --country <ISO_CODE>
semrush trends categories <CATEGORY>              # /categories (industry breakdown)
semrush trends conversion <DOMAIN>                # /purchase_conversion
```

#### `semrush overview` — Overview Reports

```bash
semrush overview rank                             # rank (Semrush Rank)
semrush overview winners-losers                   # rank_difference
```

#### `semrush project` — Projects API (v4, OAuth2)

```bash
semrush project list                              # GET /projects
semrush project get <PROJECT_ID>                  # GET /projects/:id
semrush project create --name <NAME> --domain <DOMAIN>
semrush project update <PROJECT_ID> [OPTIONS]
semrush project delete <PROJECT_ID>
```

#### `semrush local` — Local SEO APIs (v4)

```bash
semrush local listing get <LOCATION_ID>           # Listing Management
semrush local listing list
semrush local listing create [OPTIONS]
semrush local listing update <LOCATION_ID> [OPTIONS]
semrush local listing delete <LOCATION_ID>
semrush local map-rank campaigns                  # Map Rank Tracker
semrush local map-rank keywords <CAMPAIGN_ID>
semrush local map-rank heatmap <CAMPAIGN_ID>
semrush local map-rank competitors <CAMPAIGN_ID>
```

#### `semrush account` — Account Management

```bash
semrush account balance                           # Check API unit balance
semrush account auth setup                        # Interactive API key setup
semrush account auth setup-oauth                  # OAuth2 device flow (v4)
semrush account auth status                       # Show current auth status
semrush account databases                         # List available databases
```

#### `semrush batch` — Batch Operations (Agent Power Feature)

```bash
semrush batch run <RECIPE_FILE>                   # Execute a TOML/JSON batch recipe
semrush batch estimate <RECIPE_FILE>              # Estimate API unit cost before running
```

#### `semrush mcp` — MCP Server Mode

```bash
semrush mcp serve                                 # Start MCP server (stdio JSON-RPC)
semrush mcp serve --transport sse --port 8080     # SSE transport
semrush mcp tools                                 # List available MCP tools
```

***

## Feature Specifications

### F1: Structured Output for Agents

**Problem**: Semrush API v3 returns CSV only. Agents need structured JSON.[^12][^11]

**Solution**: `semrush-rs` parses all CSV responses into typed Rust structs and serializes them to JSON by default. All column codes are mapped to human-readable field names.

```bash
# Agent-optimized JSON output (default)
$ semrush keyword overview "rust programming" --database us
{
  "keyword": "rust programming",
  "search_volume": 12100,
  "cpc": 2.45,
  "competition": 0.68,
  "keyword_difficulty": 72,
  "intent": "informational",
  "results_count": 458000000,
  "trends": [0.82, 0.91, 1.0, 0.95, 0.88, ...]
}

# Human-friendly table output
$ semrush keyword overview "rust programming" --output table
┌─────────────────────┬────────┬───────┬─────────────┬────────────┐
│ Keyword             │ Volume │ CPC   │ Difficulty  │ Intent     │
├─────────────────────┼────────┼───────┼─────────────┼────────────┤
│ rust programming    │ 12,100 │ $2.45 │ 72 (Hard)   │ Informational │
└─────────────────────┴────────┴───────┴─────────────┴────────────┘
```

**Column Code Mapping**: All Semrush cryptic codes are translated:[^8]

| Code | Human-Readable Name | Description |
|------|---------------------|-------------|
| Ph | keyword | Keyword text |
| Nq | search_volume | Average monthly searches |
| Cp | cpc | Cost per click (USD) |
| Co | competition | Competitive density (0-1) |
| Kd | keyword_difficulty | Difficulty index (0-100) |
| Or | organic_keywords | Total organic keywords |
| Ot | organic_traffic | Estimated organic traffic |
| Oc | organic_cost | Traffic cost estimate |
| In | intent | Search intent (0-3) |
| Po | position | SERP position |

The full mapping covers 80+ columns and is documented in `docs/COLUMN_REFERENCE.md`.

### F2: Intelligent Rate Limiting

**Problem**: Semrush enforces 10 requests/second per IP and 10 simultaneous requests. Exceeding this loses API access temporarily.[^14][^13][^12]

**Solution**: Built-in token-bucket rate limiter using `governor` that:
- Enforces 10 req/sec by default (configurable)
- Queues excess requests automatically with exponential backoff on 429 responses
- Shows a progress indicator for queued requests
- Reports rate limit status in `--verbose` mode
- Supports concurrent request pools (up to 10 simultaneous) for batch operations

### F3: Response Caching

**Problem**: API units are expensive (10-500 units per request), and repeated queries waste budget.[^13]

**Solution**: Transparent file-based caching:
- Cache key = hash of (endpoint + all parameters except API key)
- Default TTL: 1 hour (configurable via `--cache-ttl` or config file)
- Cache location: `~/.cache/semrush-rs/` (XDG-compliant)
- `--no-cache` flag to bypass
- `semrush cache clear` to flush
- `semrush cache stats` to show size and hit rate
- Cache respects Semrush's 1-month data freshness policy[^8]

### F4: API Unit Cost Estimation

**Problem**: Users are blindsided by API costs with no way to preview spend.[^14][^13]

**Solution**: Every command supports `--dry-run` which:
- Calculates the estimated API unit cost based on Semrush's published pricing
- Shows the cost before execution
- Supports `semrush batch estimate <recipe>` for batch cost previews

```bash
$ semrush domain organic example.com --limit 1000 --dry-run
Estimated cost: 10,000 API units (10 units/line × 1,000 lines)
Current balance: 250,000 units
Run without --dry-run to execute.
```

### F5: MCP Server Mode (Agent Integration)

**Problem**: AI agents need standardized tool interfaces. Existing MCP servers for Semrush only cover 7 basic tools.[^1][^2]

**Solution**: Built-in MCP server that exposes ALL CLI commands as MCP tools:

```bash
# Start MCP server over stdio (for Claude Desktop, Claude Code, etc.)
$ semrush mcp serve

# Start MCP server over SSE (for remote agents)
$ semrush mcp serve --transport sse --port 8080
```

**MCP Tool Schema (example)**:
```json
{
  "name": "semrush_keyword_overview",
  "description": "Get keyword metrics: search volume, CPC, difficulty, competition, intent",
  "inputSchema": {
    "type": "object",
    "properties": {
      "phrase": { "type": "string", "description": "Keyword to analyze" },
      "database": { "type": "string", "default": "us", "description": "Regional database code" }
    },
    "required": ["phrase"]
  }
}
```

The MCP server exposes 50+ tools covering every CLI command, with proper JSON schemas for input validation. This makes it the most comprehensive Semrush MCP server available, compared to existing ones with ~7 tools.[^9][^2][^1]

### F6: Batch Recipe System

**Problem**: Agents often need to run multiple related queries as a workflow (e.g., "analyze this competitor: get their keywords, backlinks, traffic, and compare to my domain").

**Solution**: TOML-based batch recipe files that define multi-step SEO workflows:

```toml
# competitor_analysis.toml
[meta]
name = "Competitor Deep Dive"
description = "Full competitor analysis workflow"

[[steps]]
command = "domain overview"
args = { domain = "{{target}}", database = "{{database}}" }
output_key = "overview"

[[steps]]
command = "domain organic"
args = { domain = "{{target}}", database = "{{database}}", limit = 500 }
output_key = "organic_keywords"

[[steps]]
command = "domain competitors organic"
args = { domain = "{{target}}", database = "{{database}}", limit = 20 }
output_key = "competitors"

[[steps]]
command = "backlink overview"
args = { target = "{{target}}", target_type = "root_domain" }
output_key = "backlinks"

[[steps]]
command = "trends summary"
args = { targets = "{{target}}", country = "{{country}}" }
output_key = "traffic"
```

```bash
# Execute with variables
$ semrush batch run competitor_analysis.toml \
    --var target=competitor.com \
    --var database=us \
    --var country=US

# Estimate cost first
$ semrush batch estimate competitor_analysis.toml --var target=competitor.com
Step 1 (domain overview):           10 units
Step 2 (domain organic, 500 lines): 5,000 units
Step 3 (domain competitors):        800 units
Step 4 (backlink overview):         40 units
Step 5 (trends summary):            1 unit
─────────────────────────────────────────────
Total estimated cost:                5,851 units
```

### F7: Configuration System

**Config file location**: `~/.config/semrush-rs/config.toml`

```toml
[auth]
api_key = "your-api-key-here"    # Or use SEMRUSH_API_KEY env var
# OAuth2 tokens stored in OS keychain via `keyring` crate

[defaults]
database = "us"
output = "json"
limit = 100
cache_ttl = 3600

[rate_limit]
requests_per_second = 10
max_concurrent = 10

[cache]
enabled = true
directory = "~/.cache/semrush-rs"
max_size_mb = 500

[mcp]
transport = "stdio"             # or "sse"
port = 8080                      # for SSE transport
```

**Auth priority order**: `--api-key` flag > `SEMRUSH_API_KEY` env var > config file > OS keychain

### F8: Filtering and Sorting

All Semrush API filters are exposed as intuitive CLI flags:

```bash
# Filter organic keywords by position and volume
$ semrush domain organic example.com \
    --filter "position<=10" \
    --filter "search_volume>=1000" \
    --sort "traffic desc" \
    --columns keyword,position,search_volume,traffic,url

# Filter backlinks by type
$ semrush backlink list example.com \
    --target-type root_domain \
    --filter "type=text" \
    --filter "newlink=true" \
    --sort "page_score desc"
```

### F9: Shell Completions

Auto-generated shell completions for Bash, Zsh, Fish, and PowerShell via `clap_complete`:

```bash
$ semrush completions bash > ~/.bash_completion.d/semrush
$ semrush completions zsh > ~/.zfunc/_semrush
$ semrush completions fish > ~/.config/fish/completions/semrush.fish
```

### F10: Error Handling and Diagnostics

**Agent-friendly error responses** (structured JSON):
```json
{
  "error": {
    "code": "RATE_LIMITED",
    "message": "Rate limit exceeded. Retrying in 2.1s.",
    "retry_after_ms": 2100,
    "api_status_code": 429
  }
}
```

**Error codes** the CLI surfaces:
- `AUTH_FAILED` — Invalid API key or expired OAuth token
- `RATE_LIMITED` — 429 from Semrush (auto-retry with backoff)
- `INSUFFICIENT_UNITS` — Not enough API units (dry-run warns about this)
- `INVALID_PARAMS` — Bad request parameters
- `API_ERROR` — Semrush server error (5xx)
- `PARSE_ERROR` — Unexpected response format
- `CACHE_ERROR` — Cache read/write failure (non-fatal, falls through)
- `NETWORK_ERROR` — Connection timeout or DNS failure

All errors return exit code 1 and structured JSON to stderr. Success returns exit code 0 and data to stdout. This separation ensures agents can reliably parse output.[^21]

***

## API Coverage Matrix

Complete mapping of CLI commands to Semrush API endpoints:

### Analytics v3 (Base: `https://api.semrush.com/`)

| CLI Command | API Type Parameter | Units/Line | Category |
|-------------|-------------------|------------|----------|
| `domain overview` | `domain_rank` | 10 | Overview[^8] |
| `domain overview --all-databases` | `domain_ranks` | 10 | Overview |
| `domain overview --history` | `domain_rank_history` | 10 | Overview |
| `domain organic` | `domain_organic` | 10 | Domain[^8] |
| `domain paid` | `domain_adwords` | 20 | Domain |
| `domain ads-copies` | `domain_adwords_unique` | 40 | Domain |
| `domain competitors organic` | `domain_organic_organic` | 40 | Domain |
| `domain competitors paid` | `domain_adwords_adwords` | 40 | Domain |
| `domain ad-history` | `domain_adwords_historical` | 100 | Domain |
| `domain compare` | `domain_domains` | 80 | Domain |
| `domain pla-keywords` | `domain_shopping` | 30 | Domain |
| `domain pla-copies` | `domain_shopping_unique` | 60 | Domain |
| `domain pla-competitors` | `domain_shopping_shopping` | 60 | Domain |
| `domain pages` | `domain_organic_unique` | 10 | Domain |
| `domain subdomains` | `domain_organic_subdomains` | 10 | Domain |
| `keyword overview` | `phrase_this` | 10 | Keyword[^8] |
| `keyword overview --all-databases` | `phrase_all` | 10 | Keyword |
| `keyword batch` | `phrase_these` | 10 | Keyword |
| `keyword organic` | `phrase_organic` | 10 | Keyword |
| `keyword paid` | `phrase_adwords` | 20 | Keyword |
| `keyword related` | `phrase_related` | 40 | Keyword |
| `keyword broad-match` | `phrase_fullsearch` | 20 | Keyword |
| `keyword questions` | `phrase_questions` | 40 | Keyword |
| `keyword difficulty` | `phrase_kdi` | 50 | Keyword |
| `keyword ad-history` | `phrase_adwords_historical` | 100 | Keyword |
| `overview rank` | `rank` | 10 | Overview |
| `overview winners-losers` | `rank_difference` | 20 | Overview |

### Backlinks v1 (Base: `https://api.semrush.com/analytics/v1/`)

| CLI Command | API Type Parameter | Units | Category |
|-------------|-------------------|-------|----------|
| `backlink overview` | `backlinks_overview` | 40/req | Backlinks[^8] |
| `backlink list` | `backlinks` | 40/line | Backlinks |
| `backlink referring-domains` | `backlinks_refdomains` | 40/line | Backlinks |
| `backlink referring-ips` | `backlinks_refips` | 40/line | Backlinks |
| `backlink tld-distribution` | `backlinks_tld` | 40/line | Backlinks |
| `backlink geo` | `backlinks_geo` | 40/line | Backlinks |
| `backlink anchors` | `backlinks_anchors` | 40/line | Backlinks |
| `backlink indexed-pages` | `backlinks_pages` | 40/line | Backlinks |
| `backlink competitors` | `backlinks_competitors` | 40/line | Backlinks |
| `backlink compare` | `backlinks_matrix` | 40/line | Backlinks |
| `backlink batch` | `backlinks_comparison` | 40/line | Backlinks |
| `backlink authority-score` | `backlinks_ascore_profile` | 100/req | Backlinks |
| `backlink categories` | `backlinks_categories` | 50/req | Backlinks |
| `backlink category-profile` | `backlinks_categories_profile` | 40/line | Backlinks |
| `backlink history` | `backlinks_historical` | 40/line | Backlinks |

### Trends v3 (Base: `https://api.semrush.com/analytics/ta/api/v3/`)

| CLI Command | API Endpoint | Units | Category |
|-------------|-------------|-------|----------|
| `trends summary` | `/summary` | 1/line | Traffic[^20] |
| `trends daily` | `/summary_by_day` | 1/req | Traffic |
| `trends weekly` | `/summary_by_week` | 1/req | Traffic |
| `trends sources` | `/sources` | 1/req | Traffic |
| `trends destinations` | `/destinations` | 1/req | Traffic |
| `trends geo` | `/geo` | 1/req | Traffic |
| `trends subdomains` | `/subdomains` | 1/req | Traffic |
| `trends top-pages` | `/toppages` | 1/req | Traffic |
| `trends rank` | `/rank` | 1/req | Traffic |
| `trends categories` | `/categories` | 500/req | Traffic |
| `trends conversion` | `/purchase_conversion` | 1/req | Traffic |

### Projects v4 (Base: OAuth2-authenticated, JSON responses)[^19]

| CLI Command | Method | Endpoint |
|-------------|--------|----------|
| `project list` | GET | `/projects` |
| `project get` | GET | `/projects/:id` |
| `project create` | POST | `/projects` |
| `project update` | PUT | `/projects/:id` |
| `project delete` | DELETE | `/projects/:id` |

### Local APIs v4[^22]

| CLI Command | API | Description |
|-------------|-----|-------------|
| `local listing *` | Listing Management | CRUD for local business listings |
| `local map-rank *` | Map Rank Tracker | Campaign rankings, heatmaps, keywords |

***

## Agent-Specific Design Decisions

These design choices prioritize AI agent usability over traditional CLI conventions:

### 1. JSON-First Output
Default output is JSON (not human tables). Agents can set `--output table` if needed, but the zero-config default is machine-parseable.

### 2. Structured Errors to Stderr
Data goes to stdout, errors go to stderr as structured JSON. This allows agents to pipe stdout directly while checking stderr for issues.

### 3. Exit Codes
- `0` — Success
- `1` — API or runtime error
- `2` — Invalid arguments / usage error
- `3` — Authentication error
- `4` — Insufficient API units

### 4. Idempotent Operations
All read operations are idempotent and safe to retry. The rate limiter handles retries transparently.

### 5. Cost Awareness
Every response includes a `_meta` field with cost information:
```json
{
  "_meta": {
    "api_units_used": 100,
    "cached": false,
    "database": "us",
    "timestamp": "2026-03-05T11:30:00Z",
    "rate_limit_remaining": 8
  },
  "data": [...]
}
```

### 6. Pipe-Friendly
Supports `--quiet` mode that suppresses all non-data output. Supports `--output jsonl` for streaming JSON Lines to other tools. Detects non-TTY (piped) output and auto-switches from table to JSON format.

### 7. MCP Server as Built-in Mode
No separate server binary needed. `semrush mcp serve` turns the CLI into an MCP server that any compatible agent can connect to. This provides the broadest Semrush MCP tool surface available (50+ tools vs. the ~7 in existing implementations).[^2][^1]

***

## Non-Functional Requirements

### Performance
- Cold start: < 50ms to first output
- API call overhead: < 10ms added latency per request
- Batch operations: Support 10 concurrent requests (Semrush limit)
- Cache lookup: < 1ms for hit, < 5ms for miss

### Reliability
- Automatic retry with exponential backoff on 429 and 5xx errors
- Graceful degradation when cache is unavailable
- No data loss on interrupted batch operations (checkpoint support)

### Security
- API keys stored in OS keychain (via `keyring` crate), never in plaintext config files by default
- OAuth2 tokens encrypted at rest
- `--api-key` flag value redacted in `--verbose` logs
- No telemetry or analytics

### Compatibility
- Rust edition: 2021
- MSRV (Minimum Supported Rust Version): 1.75.0
- Platforms: Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)
- Shell completions: Bash, Zsh, Fish, PowerShell

***

## Testing Strategy

| Layer | Tool | Coverage Target |
|-------|------|-----------------|
| Unit tests | `cargo test` | All CSV parsers, column mappers, cost calculators |
| Integration tests | `wiremock` (mock HTTP) | All API endpoint request/response flows |
| CLI tests | `assert_cmd` + `predicates` | All subcommands with expected output |
| MCP tests | JSON-RPC test harness | All MCP tool schemas and responses |
| E2E tests (optional) | Real Semrush API (gated by env var) | Smoke test critical paths |

***

## Distribution

| Channel | Method |
|---------|--------|
| GitHub Releases | Pre-built binaries for Linux/macOS/Windows |
| Homebrew | `brew install semrush-rs` (tap) |
| Cargo | `cargo install semrush-rs` |
| Docker | `ghcr.io/yourorg/semrush-rs` |
| npm (optional) | npx wrapper for MCP server use |

***

## Milestones

### Phase 1: Core CLI (Weeks 1-3)
- Project scaffolding, clap command tree, config system
- API client with rate limiting and auth (v3 API key)
- CSV parser and column code mapper
- JSON/table/CSV output formatters
- Domain and keyword commands (most-used endpoints)

### Phase 2: Full API Coverage (Weeks 4-6)
- Backlink analytics commands
- Overview reports (Rank, Winners/Losers)
- Trends / Traffic Analytics commands
- Response caching layer
- `--dry-run` cost estimation

### Phase 3: Agent Features (Weeks 7-9)
- MCP server mode (stdio + SSE transport)
- Batch recipe system
- OAuth2 device flow for v4 APIs
- Projects and Local SEO commands
- Shell completions

### Phase 4: Polish and Release (Weeks 10-11)
- Comprehensive test suite
- Documentation (README, USAGE.md, MCP.md, COLUMN_REFERENCE.md)
- CI/CD pipeline (GitHub Actions)
- Homebrew tap, Docker image, binary releases
- Beta testing with agent frameworks (Claude Desktop, Cursor)

***

## Success Criteria

- **API Coverage**: 100% of public Semrush API endpoints wrapped
- **Agent Compatibility**: Works as MCP server with Claude Desktop, Claude Code, Cursor, and any MCP-compatible agent
- **Performance**: Sub-50ms cold start, sub-10ms overhead per API call
- **Correctness**: All CSV column codes correctly mapped to human-readable names
- **Reliability**: Auto-retry on rate limits, graceful cache fallback
- **Documentation**: Every command has `--help` text, MCP tools have JSON schemas, COLUMN_REFERENCE.md maps all 80+ column codes

***

## Appendix A: Semrush Database Codes

Common regional databases used in the `--database` flag:

| Code | Region | Type |
|------|--------|------|
| us | United States | Desktop |
| uk | United Kingdom | Desktop |
| de | Germany | Desktop |
| fr | France | Desktop |
| es | Spain | Desktop |
| it | Italy | Desktop |
| br | Brazil | Desktop |
| au | Australia | Desktop |
| mobile-us | United States | Mobile |
| mobile-uk | United Kingdom | Mobile |
| us-ext | United States | Extended |

The full list of 142+ databases is available via `semrush account databases`.[^8]

## Appendix B: Search Intent Codes

| Code | Intent | Description |
|------|--------|-------------|
| 0 | Commercial | User wants to investigate brands or services |
| 1 | Informational | User wants to find an answer to a specific question |
| 2 | Navigational | User wants to find a specific page or site |
| 3 | Transactional | User wants to complete an action (purchase, download) |

## Appendix C: Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SEMRUSH_API_KEY` | Semrush API key for v3 endpoints | None (required) |
| `SEMRUSH_CONFIG` | Path to config file | `~/.config/semrush-rs/config.toml` |
| `SEMRUSH_CACHE_DIR` | Cache directory path | `~/.cache/semrush-rs/` |
| `SEMRUSH_OUTPUT` | Default output format | `json` |
| `SEMRUSH_DATABASE` | Default regional database | `us` |
| `SEMRUSH_RATE_LIMIT` | Requests per second | `10` |
| `NO_COLOR` | Disable colored output | Unset |
| `SEMRUSH_LOG` | Log level (trace, debug, info, warn, error) | `warn` |

---

## References

1. [GitHub - mrkooblu/semrush-mcp: A Model Context Protocol (MCP) server implementation that provides tools for accessing Semrush API data.](https://github.com/mrkooblu/semrush-mcp) - A Model Context Protocol (MCP) server implementation that provides tools for accessing Semrush API d...

2. [Semrush MCP Server - GitHub](https://github.com/metehan777/semrush-mcp) - Semrush MCP Server · Features · Prerequisites · Installation · Configuration · Usage with Claude Des...

3. [GitHub - arambert/semrush: Client for the SemRush API](https://github.com/arambert/semrush) - Client for the SemRush API. Contribute to arambert/semrush development by creating an account on Git...

4. [No API access without paying?? : r/SEMrush - Reddit](https://www.reddit.com/r/SEMrush/comments/ldijz7/no_api_access_without_paying/) - I only need API access to connect for reporting, everyone else gives at least limited API access wit...

5. [I'M OFFICIALLY DONE WITH SEMRUSH - What's a Good Alternative?](https://www.reddit.com/r/localseo/comments/1njgy2t/im_officially_done_with_semrush_whats_a_good/) - Pagespeed API data dont show any data at all. There are just to many problems with this service righ...

6. [What is frustrating you about SEMRush](https://www.reddit.com/r/SEO/comments/13m7lsn/what_is_frustrating_you_about_semrush/)

7. [Semrush's decline: A shift in focus and rising frustration - LinkedIn](https://www.linkedin.com/posts/ivanpalii_respect-to-semrush-for-not-hiding-criticism-activity-7393240511978815488-BnpN) - 60% of the posts there are about: - terrible cancellation experience - huge prices - lack of support...

8. [Semrush API Help](https://www.semrush.com/kb/5-api) - Application Programming Interface (API) is a method of extracting raw Semrush data without having to...

9. [Semrush MCP Integration | AI Agent Tools | Composio](https://mcp.composio.dev/semrush) - Semrush is a popular SEO tool suite that specializes in keyword research, competitor analysis, and G...

10. [Anyone else having issues with SemRush lately?](https://www.reddit.com/r/SEO/comments/wg3i5a/anyone_else_having_issues_with_semrush_lately/) - Anyone else having issues with SemRush lately?

11. [FAQ | Semrush API - Developer](https://developer.semrush.com/api/basics/faq/)

12. [SEMrush API Essential Guide - Rollout](https://rollout.com/integration-guides/semrush/api-essentials) - An essential reference guide to the SEMrush API

13. [API unit balance - Developer - Semrush](https://developer.semrush.com/api/basics/api-units-balance/) - The Trends API operates under a rate limit service, allowing up to 10 requests per second (RPS) per ...

14. [Automating SEO Reporting with SEMrush API and Custom Scripts](https://www.vocso.com/blog/automating-seo-reporting-with-semrush-api-and-custom-scripts/) - The Semrush API is a powerful tool that allows users to access a wealth of SEO data programmatically...

15. [SEMrush vs Ahrefs (Reddit Verdict): What Real Users Say](https://brightseotools.com/post/SEMrush-vs-Ahrefs-Reddit-Verdict-What-Real-Users-Say) - This article compiles the most consistent Reddit verdicts from 2024–2026, cross-referenced with real...

16. [How to Build CLI Applications with Clap in Rust - OneUptime](https://oneuptime.com/blog/post/2026-02-03-rust-clap-cli-applications/view) - Learn how to build powerful command-line applications in Rust using Clap. This guide covers argument...

17. [Building CLI tools with Rust (Clap) - DEV Community](https://dev.to/godofgeeks/building-cli-tools-with-rust-clap-4bo2) - Confused by the DevProd tool landscape? Here's a practical guide for engineering leaders comparing t...

18. [Rust实战：使用Clap和Tokio构建现代CLI应用 - 华为云](https://bbs.huaweicloud.com/blogs/470981) - 本文介绍了如何使用Rust构建一个简化的Redis命令行客户端(mini-redis-cli)。通过clap库实现命令行参数解析，支持get/set子命令...

19. [Basic docs | Semrush API - Developer](https://developer.semrush.com/api/v4/basic-docs/) - Project API lets you get basic information about your Semrush projects, as well as perform some basi...

20. [Trends API reference - Developer - Semrush](https://developer.semrush.com/api/v3/trends/api-reference/) - The Industry Categories report provides a list of all domains within specific industry categories. W...

21. [Code execution with MCP: building more efficient AI agents - Anthropic](https://www.anthropic.com/engineering/code-execution-with-mcp) - MCP provides a foundational protocol for agents to connect to many tools and systems. However, once ...

22. [Listing Management | Semrush API - Developer](https://developer.semrush.com/api/v4/listing-management/) - Jump to GetLocationGetLocation. Get one location by its ID. You can execute up to 10 requests per se...

