# Changelog

## [0.1.1]

- Add README with agent integration docs and full command reference
- Add man page generation (`cargo run --bin gen-man`)
- Add library crate for programmatic access
- Fix clippy and formatting for CI compliance

## [0.1.0]

- Initial release
- Full Semrush API coverage: domain, keyword, backlink, overview, trends
- v4 API support: projects, local SEO (OAuth2)
- Batch recipe system with TOML workflows
- Disk caching with SHA256 keys and TTL expiry
- Rate limiting (10 req/s) with exponential backoff
- Output formats: JSON, table, CSV, JSONL
- Cost estimation with --dry-run
- CI/CD: test, auto-tag, cross-platform release builds
