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
semrush domain overview example.com

# Organic keywords for a domain
semrush domain organic example.com --limit 20

# Keyword research
semrush keyword overview "rust programming"

# Related keywords
semrush keyword related "machine learning" --limit 50

# Backlink profile
semrush backlink overview example.com

# Traffic trends
semrush trends summary example.com competitor.com

# Different regional database
semrush domain overview example.de --database de

# CSV output for spreadsheets
semrush domain organic example.com --output csv > keywords.csv

# Dry run -- estimate API cost without making the call
semrush domain organic example.com --dry-run
```

## AI Agent Integration

semrush is designed as a first-class tool for AI agents (Claude, GPT, etc.) and automation pipelines.

### Structured output

Output auto-detects: **table** for terminals, **JSON** for pipes. Force a format with `--output`:

```bash
# JSON with metadata envelope (default when piped)
semrush domain overview example.com | jq '.data[0].organic_keywords'

# JSON Lines for streaming processing
semrush domain organic example.com --output jsonl

# CSV for data pipelines
semrush domain organic example.com --output csv

# Table for human reading
semrush domain overview example.com --output table
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
OVERVIEW=$(semrush domain overview competitor.com --output json)

# Step 2: Get their top organic keywords
KEYWORDS=$(semrush domain organic competitor.com --limit 50 --output json)

# Step 3: Check difficulty of a specific keyword
DIFFICULTY=$(semrush keyword difficulty "target keyword")

# Step 4: Analyze backlink profile
BACKLINKS=$(semrush backlink overview competitor.com)
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
semrush batch run competitor-audit.toml --var domain=example.com --var database=us

# Estimate cost first
semrush batch estimate competitor-audit.toml --var domain=example.com
```

## Commands

### Domain Analytics

```bash
semrush domain overview     <DOMAIN>       # Traffic, rank, keywords count
semrush domain organic      <DOMAIN>       # Organic keyword positions
semrush domain paid         <DOMAIN>       # Paid search keywords
semrush domain competitors  organic <DOMAIN>  # Organic competitors
semrush domain ads-copies   <DOMAIN>       # Ad copy texts
semrush domain ad-history   <DOMAIN>       # Historical ad data
semrush domain pla-keywords <DOMAIN>       # Product listing ad keywords
semrush domain pla-copies   <DOMAIN>       # PLA ad copies
semrush domain pla-competitors <DOMAIN>    # PLA competitors
semrush domain pages        <DOMAIN>       # Top pages by traffic
semrush domain subdomains   <DOMAIN>       # Subdomain breakdown
semrush domain compare      <D1> <D2>      # Compare domains
```

### Keyword Research

```bash
semrush keyword overview    <KEYWORD>      # Volume, CPC, difficulty
semrush keyword batch       <K1> <K2>      # Bulk keyword lookup
semrush keyword organic     <KEYWORD>      # Domains ranking for keyword
semrush keyword paid        <KEYWORD>      # Paid results for keyword
semrush keyword related     <KEYWORD>      # Related keywords
semrush keyword broad-match <KEYWORD>      # Broad match keywords
semrush keyword questions   <KEYWORD>      # Question-based keywords
semrush keyword difficulty  <KEYWORD>      # Difficulty score (0-100)
semrush keyword ad-history  <KEYWORD>      # Historical ad data
```

### Backlink Analytics

```bash
semrush backlink overview           <DOMAIN>      # Total backlinks, authority
semrush backlink list               <DOMAIN>      # Individual backlinks
semrush backlink referring-domains  <DOMAIN>      # Referring domains
semrush backlink referring-ips      <DOMAIN>      # Referring IPs
semrush backlink anchors            <DOMAIN>      # Anchor text distribution
semrush backlink tld-distribution   <DOMAIN>      # TLD breakdown
semrush backlink geo                <DOMAIN>      # Geographic distribution
semrush backlink indexed-pages      <DOMAIN>      # Indexed pages
semrush backlink competitors        <DOMAIN>      # Backlink competitors
semrush backlink compare            <D1> <D2>     # Compare targets
semrush backlink batch              <D1> <D2>     # Bulk overview
semrush backlink authority-score    <DOMAIN>      # Authority score
semrush backlink categories         <DOMAIN>      # Category distribution
semrush backlink category-profile   <DOMAIN>      # Category profile
semrush backlink history            <DOMAIN>      # Historical data
```

### Traffic Trends

```bash
semrush trends summary      <DOMAIN1> [DOMAIN2]  # Visits, bounce rate, pages/visit
semrush trends daily        <DOMAIN>             # Daily traffic data
semrush trends weekly       <DOMAIN>             # Weekly traffic data
semrush trends sources      <DOMAIN>             # Traffic source breakdown
semrush trends destinations <DOMAIN>             # Outgoing traffic destinations
semrush trends geo          <DOMAIN>             # Geographic distribution
semrush trends subdomains   <DOMAIN>             # Subdomain traffic
semrush trends top-pages    <DOMAIN>             # Top pages by traffic
semrush trends rank                              # Traffic rank
semrush trends categories   <CATEGORY>           # Category breakdown
semrush trends conversion   <DOMAIN>             # Conversion data
```

### Project Management (v4 API)

Requires OAuth2 token (`SEMRUSH_OAUTH_TOKEN` env var):

```bash
semrush project list
semrush project get <PROJECT_ID>
semrush project create --name "My Project" --domain example.com
semrush project update <PROJECT_ID> --name "New Name"
semrush project delete <PROJECT_ID>
```

### Local SEO (v4 API)

```bash
semrush local listing list
semrush local listing get <LISTING_ID>
semrush local listing create --json '{"name": "Business"}'
semrush local map-rank campaigns
semrush local map-rank keywords <CAMPAIGN_ID>
semrush local map-rank heatmap <CAMPAIGN_ID>
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
