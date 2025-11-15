# Kanban Version Sync (Lightweight, Docs‑Only)

**Scope:** applies when modifying any file under `docs/kanban/**`.  
**Goal:** keep **epic-level** versioning in sync **without** bumping Cargo versions on every checkbox.

---

## When you check off a Kanban item

Every time you change a checklist line from `- [ ]` to `- [x]` in `docs/kanban/**`:

1) **Read the current workspace version** from `code/Cargo.toml` -> `[workspace.package].version`.  
   If missing, write `Unreleased` instead of a version number for the steps below.

2) **Update the epic header** in the Kanban you touched:
   - If the epic is newly started and its “Introduced in” is `2025-11-15`, set it to the current workspace version (or `Unreleased` if no version chosen yet).
   - Always set **“Last updated in”** to the current workspace version (or `Unreleased`).

   Example:
   ```markdown
   > Status: **In progress**  
   > Introduced in: `v0.2.0`  
   > Last updated in: `v0.2.1`
   ```

3) **Update the cross‑epic index** at `docs/kanban/_epic_versions.yaml`.  
   Ensure the epic ID is present with `introduced_in` and `last_updated_in`.

   ```yaml
   epics:
     FP-07:
       introduced_in: v0.2.0
       last_updated_in: v0.2.1
     MAP-09:
       introduced_in: v0.2.0
       last_updated_in: v0.2.1
   ```

4) **Add/refresh an Unreleased changelog entry** in `CHANGELOG.md` referencing the epic and the exact checklist line you checked:
   ```markdown
   ## [Unreleased]
   ### Changed
   - FP-07 – “Normalize ServiceRequest.status casing” (checkbox completed)
   ```

5) **Commit message**  
   Use a docs‑scoped commit that cites the epic:
   ```
   docs(FP-07): check off “Normalize status casing”; update epic header, versions index, changelog
   ```

> **No code version bump here.** Actual **SemVer** bumps (patch/minor/major) only happen in release PRs or when the change meets your breaking/feature criteria. See “Release flow” below.

---

## Release flow (when you *do* bump versions)

1) Decide the bump: **patch** for backward‑compatible fixes, **minor** for added functionality, **major** for breaking changes.  
2) Update `[workspace.package].version` at `code/Cargo.toml` and switch all `Unreleased` epic headers/entries that shipped in this release to that version.  
3) Convert `## [Unreleased]` bullets into a final `## [X.Y.Z] – YYYY‑MM‑DD` section.  
4) Tag: `git tag -a vX.Y.Z -m "..."; git push origin vX.Y.Z`.

*(SemVer definitions; Keep‑a‑Changelog section structure.)*
