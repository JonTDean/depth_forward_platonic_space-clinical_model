# Kanban edits -> Branch binding & light version rules

**Scope:** Files in `code/docs/kanban/**` only.  
**How to load these rules:**
```bash
codex --cd code/docs/kanban --add-dir ../../
```

---

## 0) Definitions

- **Epic Kanban** = the `.md` file for the epic you’re editing.
- **Branch target version** = the *intended* SemVer for this branch when it ships (recorded in the epic; not a forced Cargo bump yet).

---

## 1) Branch ↔ Epic binding (required)

- Read the current branch:
  ```bash
  git rev-parse --abbrev-ref HEAD
  ```
- The epic header **must** contain a `Branch:` line with this exact name.
  - If missing, insert it.
  - If the branch name does **not** include the epic ID (e.g., `FP-07`, `MAP-05`), either rename the branch or add the ID to the epic’s `Branches:` list and note the exception.

**Recommended header shape:**
```markdown
> Epic: FP-07 – Ingestion error surface
> Branch: feature/FP-07-ingestion-error-surface
> Branch target version: v0.2.1
> Status: DOING
> Introduced in: _TBD_
> Last updated in: v0.2.1
```

---

## 2) When a card leaves “DOING” (to REVIEW or DONE)

1. **Pick a SemVer bump for this branch’s *target* version** (don’t change Cargo’s workspace version yet):
   - **PATCH** - routine checklist completion, no user‑visible change.
   - **MINOR** - the card is DONE and adds user‑visible behavior.
   - **MAJOR** - breaking change (rare here).
2. **Update the epic header:**
   - Set/adjust `Branch target version: vX.Y.Z`.
   - Update `Last updated in: vX.Y.Z`.
3. **Changelog (Unreleased):**
   - Add/refresh an entry that references the epic ID and the `Branch target version`.

> SemVer rationale: MAJOR.MINOR.PATCH communicates scope clearly; we’re only recording the intent here, not bumping the workspace yet. 

---

## 3) When the branch merges to `main`

- In the epic header, mark **“Included in upcoming release”** (or add `Release: Upcoming`) and keep `Introduced in: _TBD_` until you cut the release.
- Move the card to **DONE**.
- Ensure `CHANGELOG.md` has an **Unreleased** entry listing this epic and its `Branch target version`.

*(Actual SemVer bump of the workspace version and final “Introduced in: vX.Y.Z” happen in the release PR.)*

---

## 4) Quick checks (Codex should run)

- Confirm the binding and target version:
  ```bash
  git rev-parse --abbrev-ref HEAD
  rg -n "^(Branch:|Branch target version:|Introduced in:|Last updated in:)" .
  ```
- If you’re cutting a release later, apply the real bump in `../../Cargo.toml` once, then replace `Introduced in: _TBD_` with that version across included epics and roll **Unreleased** -> `vX.Y.Z` in `CHANGELOG.md`.

---

## 5) Commit guidance

- For Kanban-only progress:
  ```
  docs(FP-07): move card to REVIEW; set Branch target version to v0.2.1; update Unreleased changelog
  ```
- For merge commits to main (if you keep them):
  ```
  chore(FP-07): merge feature/FP-07-ingestion-error-surface -> main (mark epic as Upcoming)
  ```

---
