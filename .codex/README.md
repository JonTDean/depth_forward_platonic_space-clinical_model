# .codex — Agent Guidance Pack

This folder shards the former `code/AGENT.md` into small, ownable files so humans can maintain them easily.  
Codex still reads a single file (`code/AGENTS.md`) that we **generate** by concatenating `*.md` here in numeric order.

- Authoritative aggregator (generated): `code/AGENTS.md`
- Human-maintained sources: `code/.codex/*.md` and `code/.codex/50-crates/*.md`

Tips for Codex compatibility:

- Codex loads at most **one file per directory** (`AGENTS.override.md` > `AGENTS.md` > fallbacks).
- Use `CODEX_HOME=$(pwd)/code/.codex` if you want per-repo config (see `config.example.toml`).
- Keep the aggregator under ~32–64 KiB; if you exceed the cap, split further or raise `project_doc_max_bytes`.  
  (Defaults and discovery rules: Codex docs “Custom instructions with AGENTS.md”).
