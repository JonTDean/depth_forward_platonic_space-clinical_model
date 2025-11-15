# Codex Compatibility Notes

- **Discovery & precedence.** Codex concatenates instruction files in order: global (~/.codex) then project root ? subdirs on the path to CWD. Per directory Codex includes at most **one** file, preferring `AGENTS.override.md` then `AGENTS.md`, then fallback names. (Codex docs)
- **Size limits.** The combined project instruction slice is capped (`project_doc_max_bytes`, 32 KiB by default). Split large guidance across nested directories or raise the cap in config. (Codex docs)
- **Fallback names.** Customize with `project_doc_fallback_filenames` in `~/.codex/config.toml` (or your chosen `CODEX_HOME`).
- **Launching with a local profile.** You can point `CODEX_HOME` at `code/.codex` for repo-specific config.
- **Prompting best practices for agents.** Provide clear file/symbol pointers; include verification steps (`fmt`, `clippy`, `test`, mdBook build); split large tasks. (Codex prompting guide)

References:
- Custom instructions & discovery: Codex �Custom instructions with AGENTS.md�.
- Prompting patterns: Codex �Prompting guide�.
