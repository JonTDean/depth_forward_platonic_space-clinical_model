# Branching & Commit Conventions

## Branching
- Base: `main`; prefer 1 card ? 1 feature branch

**Name:**
```
<kind>/<card-id>-kebab-summary
```
where `<kind>` ? {`feature`, `bugfix`, `chore`, `docs`, `spike`}

**Examples:**
- `feature/FP-07-ingestion-error-surface`
- `bugfix/MAP-03-fix-ncit-concept-id`
- `docs/DM-05-align-fhir-docs`

**Workflow**
1. Move card to **DOING**; branch from `main`
2. Implement code + tests + docs
3. All checks pass ? move to **REVIEW**
4. Merge to `main` ? **DONE**

## Commits
**Format:**
```
<type>(<card-id>[:<scope>]): short imperative summary
```
`<type>` ? {`feat`, `fix`, `refactor`, `chore`, `docs`, `test`, `ci`}

**Examples**
```
feat(FP-07:ingestion): add IngestionError enum and error mapping
fix(MAP-05:mapping): correct NCIt concept id for PET code 78815
docs(DM-05): document core domain model and invariants
```

**Body tips**
- Bullet points of cross-cutting changes
- Mention updated docs/fixtures
- `Tests:` line with what ran
