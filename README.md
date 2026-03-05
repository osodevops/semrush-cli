# semrush

A fast, single-binary CLI for the [Semrush API](https://developer.semrush.com/), built in Rust.

Full access to domain analytics, keyword research, backlink data, traffic trends, and more -- with structured JSON output designed for AI agents and automation.

[![CI](https://github.com/osodevops/semrush-cli/actions/workflows/test.yml/badge.svg)](https://github.com/osodevops/semrush-cli/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Install

```bash
# Homebrew (macOS/Linux)
brew install osodevops/tap/semrush

# Pre-built binaries -- download from GitHub Releases
# https://github.com/osodevops/semrush-cli/releases

# From source
cargo install --git https://github.com/osodevops/semrush-cli
```

## Setup

Set your [Semrush API key](https://www.semrush.com/api-analytics/):

```bash
export SEMRUSH_API_KEY="your-api-key"
```

Or create a config file at `~/.config/semrush/config.toml`:

```toml
[auth]
api_key = "your-api-key"

[defaults]
database = "us"
output = "json"
cache_ttl = 3600

[rate_limit]
requests_per_second = 10

[cache]
enabled = true
```

**Precedence:** `--api-key` flag > `SEMRUSH_API_KEY` env var > config file

## Quick Start

```bash
# Domain overview
semrush domain overview --domain example.com

# Organic keywords for a domain
semrush domain organic --domain example.com --limit 20

# Keyword research
semrush keyword overview --phrase "rust programming"

# Related keywords
semrush keyword related --phrase "machine learning" --limit 50

# Backlink profile
semrush backlink overview --target example.com

# Traffic trends
semrush trends summary --targets "example.com,competitor.com"

# Different regional database
semrush domain overview --domain example.de --database de

# CSV output for spreadsheets
semrush domain organic --domain example.com --output csv > keywords.csv

# Dry run -- estimate API cost without making the call
semrush domain organic --domain example.com --dry-run
```

## AI Agent Integration

semrush is designed as a first-class tool for AI agents (Claude, GPT, etc.) and automation pipelines.

### Structured output

Output auto-detects: **table** for terminals, **JSON** for pipes. Force a format with `--output`:

```bash
# JSON with metadata envelope (default when piped)
semrush domain overview --domain example.com | jq '.data[0].organic_keywords'

# JSON Lines for streaming processing
semrush domain organic --domain example.com --output jsonl

# CSV for data pipelines
semrush domain organic --domain example.com --output csv

# Table for human reading
semrush domain overview --domain example.com --output table
```

JSON output includes a `_meta` envelope with timing, cache status, and cost:

```json
{
  "_meta": {
    "report_type": "domain_overview",
    "database": "us",
    "timestamp": "2025-03-05T12:00:00Z",
    "cached": false,
    "api_units_estimated": 10
  },
  "data": [...]
}
```

### Agent-friendly design

- **Deterministic output** -- same input always produces same structure, safe for parsing
- **Structured errors** -- errors go to stderr as JSON with exit codes (0-4)
- **Exit codes** -- 0 success, 1 auth, 2 rate limit, 3 input error, 4 API/network error
- **Pipe-friendly** -- auto-detects TTY vs pipe, no colour codes in piped output
- **Rate limiting built-in** -- 10 req/s token bucket, auto-retry on 429 with exponential backoff
- **Local caching** -- SHA256-keyed disk cache with configurable TTL, use `--no-cache` to bypass
- **Cost awareness** -- `--dry-run` shows estimated API unit cost before executing
- **Quiet mode** -- `--quiet` suppresses all non-data output

### Example: agent workflow

```bash
# Step 1: Get domain overview
OVERVIEW=$(semrush domain overview --domain competitor.com --output json)

# Step 2: Get their top organic keywords
KEYWORDS=$(semrush domain organic --domain competitor.com --limit 50 --output json)

# Step 3: Check difficulty of a specific keyword
DIFFICULTY=$(semrush keyword difficulty --phrase "target keyword")

# Step 4: Analyze backlink profile
BACKLINKS=$(semrush backlink overview --target competitor.com)
```

### Batch recipes

Run multi-step workflows from TOML recipe files:

```toml
# competitor-audit.toml
[meta]
name = "Competitor Audit"
description = "Full competitive analysis"

[[steps]]
command = "domain_overview"
output_key = "overview"
[steps.args]
domain = "{{domain}}"
database = "{{database}}"

[[steps]]
command = "domain_organic"
output_key = "keywords"
[steps.args]
domain = "{{domain}}"
database = "{{database}}"
limit = 50
```

```bash
# Run the recipe
semrush batch run --file competitor-audit.toml --var domain=example.com --var database=us

# Estimate cost first
semrush batch estimate --file competitor-audit.toml --var domain=example.com
```

## Commands

### Domain Analytics

```bash
semrush domain overview     --domain <DOMAIN>   # Traffic, rank, keywords count
semrush domain organic      --domain <DOMAIN>   # Organic keyword positions
semrush domain paid         --domain <DOMAIN>   # Paid search keywords
semrush domain competitors  organic --domain <DOMAIN>  # Organic competitors
semrush domain ads-copies   --domain <DOMAIN>   # Ad copy texts
semrush domain ad-history   --domain <DOMAIN>   # Historical ad data
semrush domain pla-keywords --domain <DOMAIN>   # Product listing ad keywords
semrush domain pla-copies   --domain <DOMAIN>   # PLA ad copies
semrush domain pla-competitors --domain <DOMAIN> # PLA competitors
semrush domain pages        --domain <DOMAIN>   # Top pages by traffic
semrush domain subdomains   --domain <DOMAIN>   # Subdomain breakdown
semrush domain compare      --domains <D1,D2>   # Compare domains
```

### Keyword Research

```bash
semrush keyword overview    --phrase <KEYWORD>   # Volume, CPC, difficulty
semrush keyword batch       --phrases <K1,K2>    # Bulk keyword lookup
semrush keyword organic     --phrase <KEYWORD>   # Domains ranking for keyword
semrush keyword paid        --phrase <KEYWORD>   # Paid results for keyword
semrush keyword related     --phrase <KEYWORD>   # Related keywords
semrush keyword broad-match --phrase <KEYWORD>   # Broad match keywords
semrush keyword questions   --phrase <KEYWORD>   # Question-based keywords
semrush keyword difficulty  --phrase <KEYWORD>   # Difficulty score (0-100)
semrush keyword ad-history  --phrase <KEYWORD>   # Historical ad data
```

### Backlink Analytics

```bash
semrush backlink overview           --target <DOMAIN>  # Total backlinks, authority
semrush backlink list               --target <DOMAIN>  # Individual backlinks
semrush backlink referring-domains  --target <DOMAIN>  # Referring domains
semrush backlink referring-ips      --target <DOMAIN>  # Referring IPs
semrush backlink anchors            --target <DOMAIN>  # Anchor text distribution
semrush backlink tld-distribution   --target <DOMAIN>  # TLD breakdown
semrush backlink geo                --target <DOMAIN>  # Geographic distribution
semrush backlink indexed-pages      --target <DOMAIN>  # Indexed pages
semrush backlink competitors        --target <DOMAIN>  # Backlink competitors
semrush backlink compare            --targets <D1,D2>  # Compare targets
semrush backlink batch              --targets <D1,D2>  # Bulk overview
semrush backlink new                --target <DOMAIN>  # New backlinks
semrush backlink lost               --target <DOMAIN>  # Lost backlinks
semrush backlink categories         --target <DOMAIN>  # Category distribution
semrush backlink history            --target <DOMAIN>  # Historical data
```

### Traffic Trends

```bash
semrush trends summary      --targets <DOMAINS>  # Visits, bounce rate, pages/visit
semrush trends daily        --targets <DOMAINS>  # Daily traffic data
semrush trends weekly       --targets <DOMAINS>  # Weekly traffic data
semrush trends sources      --target <DOMAIN>    # Traffic source breakdown
semrush trends destinations --target <DOMAIN>    # Outgoing traffic destinations
semrush trends geo          --target <DOMAIN>    # Geographic distribution
semrush trends subdomains   --target <DOMAIN>    # Subdomain traffic
semrush trends top-pages    --target <DOMAIN>    # Top pages by traffic
semrush trends rank         --target <DOMAIN>    # Traffic rank
semrush trends categories   --target <DOMAIN>    # Category breakdown
semrush trends conversion   --target <DOMAIN>    # Conversion data
```

### Project Management (v4 API)

Requires OAuth2 token (`SEMRUSH_OAUTH_TOKEN` env var):

```bash
semrush project list
semrush project get --id <PROJECT_ID>
semrush project create --name "My Project" --domain example.com
semrush project update --id <PROJECT_ID> --name "New Name"
semrush project delete --id <PROJECT_ID>
```

### Local SEO (v4 API)

```bash
semrush local listing list
semrush local listing get --id <LISTING_ID>
semrush local listing create --json '{"name": "Business"}'
semrush local map-rank campaigns
semrush local map-rank keywords --campaign-id <ID>
semrush local map-rank heatmap --campaign-id <ID> --keyword-id <KID>
```

### Utility

```bash
semrush account balance          # Check API key status
semrush account auth setup       # Setup instructions
semrush cache clear              # Clear response cache
semrush cache stats              # Cache statistics
semrush completions <SHELL>      # Generate shell completions (bash/zsh/fish)
```

## Global Options

| Option | Description | Default |
|--------|-------------|---------|
| `--api-key` | Semrush API key | `SEMRUSH_API_KEY` env |
| `--output` | Format: `json`, `table`, `csv`, `jsonl` | Auto-detect |
| `--database` | Regional DB: `us`, `uk`, `de`, `mobile-us`, etc. | `us` |
| `--limit` | Max results | `100` |
| `--offset` | Skip first N results | `0` |
| `--no-cache` | Bypass disk cache | `false` |
| `--cache-ttl` | Cache TTL in seconds | `3600` |
| `--verbose` | Debug logging | `false` |
| `--quiet` | Suppress non-data output | `false` |
| `--dry-run` | Estimate API cost only | `false` |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SEMRUSH_API_KEY` | API key (required for v3 endpoints) |
| `SEMRUSH_OAUTH_TOKEN` | OAuth2 token (required for v4 endpoints) |
| `SEMRUSH_DATABASE` | Default regional database |
| `SEMRUSH_OUTPUT` | Default output format |

## Shell Completions

```bash
# Zsh
semrush completions zsh > ~/.zsh/completions/_semrush

# Bash
semrush completions bash > /etc/bash_completion.d/semrush

# Fish
semrush completions fish > ~/.config/fish/completions/semrush.fish
```

## Man Pages

```bash
# Generate man page
cargo run --bin gen-man

# View it
man ./man/semrush.1
```

## Building from Source

```bash
git clone https://github.com/osodevops/semrush-cli
cd semrush-cli
cargo build --release
# Binary at target/release/semrush
```

**Requirements:** Rust 1.80+

```bash
# Run tests
cargo test --all-targets

# Lint
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt --all -- --check
```

## License

[MIT](LICENSE)
