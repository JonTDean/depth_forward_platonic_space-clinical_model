# Kanban - feature/obo-import-and-reasoning (019)

**Theme:** Spec-complete semantics - full(er) OBO import & lightweight reasoning  
**Branch:** `feature/domain/obo-import-and-reasoning`  
**Goal:** Import NCIt OBO (and at least one companion OBO ontology) into a graph representation and expose simple reasoning utilities (transitive closure, synonym expansion) for mapping and analytics.

### Columns
* **TODO** – Not started yet  
* **INPROGRESS** – In progress  
* **REVIEW** – Needs code review / refactor / docs polish  
* **DONE** – Completed  

---

## TODO

### OBO-01 – OBO graph crate

- [ ] Add a new crate `lib/domain/obo_graph` (`dfps_obo_graph`):

  - [ ] Types:

    - `OntologyGraph { id, nodes: Vec<Node>, edges: Vec<Edge> }`
    - `Node { iri, label, synonyms: Vec<String>, xref_ncit_ids: Vec<String> }`
    - `Edge { from, to, relation }` with focus on `is_a`/`part_of`.

  - [ ] Parsers for NCIt OBO and MONDO:

    - Either via an OBO/OWL parsing library or a minimal line-based parser for a curated slice.

- [ ] Provide helpers:

  - [ ] `load_ontology_graph(id: &str) -> Result<OntologyGraph, OboError>` using embedded or on-disk `.obo` / `.owl`.

### OBO-02 – Reasoning utilities

- [ ] Implement basic utilities on `OntologyGraph`:

  - [ ] `ancestors(node_iri)`, `descendants(node_iri)` via transitive closure.
  - [ ] `synonym_set(ncit_id)` giving canonical label + synonyms from OBO.
  - [ ] `related_concepts(ncit_id)` limited to a few hops in the graph.

- [ ] Add a small caching layer so repeated queries are fast.

### OBO-03 – Integration with terminology & mapping

- [ ] Extend `dfps_terminology::obo` to:

  - [ ] Expose a bridge from `DimNCITConcept.ncit_id` to `OntologyGraph` node IRIs.

- [ ] In `dfps_mapping`:

  - [ ] Optionally call `synonym_set` / `ancestors` when building lexical features:

    - Expand candidate synonyms used by `LexicalRanker`.
    - Optionally adjust mapping scores based on ontology distance.

- [ ] Ensure behavior is gated behind a feature flag (`obo-graph`) and remains deterministic when disabled.

### OBO-04 – Tests & fixtures

- [ ] Add small clipped NCIt OBO fixtures under `data/obo/ncit-mini.obo`:

  - [ ] Include nodes for PET/CT and immediate neighbors.

- [ ] Tests in `dfps_obo_graph`:

  - [ ] Parse the mini graph.
  - [ ] Verify ancestor/descendant relationships for known NCIt IDs.

- [ ] Mapping tests in `dfps_test_suite`:

  - [ ] Confirm synonym expansion from OBO does not reduce scores (monotonicity) and improves coverage for close variants of PET/CT codes.

### OBO-05 – Docs

- [ ] Add `docs/system-design/clinical/ncit/concepts/obo-graph.md` describing:

  - [ ] How NCIt OBO / MONDO graphs are loaded.
  - [ ] What reasoning capabilities are supported and how they influence mapping.

- [ ] Update `docs/system-design/clinical/fhir/concepts/terminology-layer.md` to reflect that OBO-backed concepts now have graph context, not just static metadata.

---

## INPROGRESS
- _Empty_

---

## REVIEW
- _Empty_

---

## DONE
- _Empty_

---

## Acceptance Criteria

- NCIt OBO (and at least one other OBO ontology) can be imported into a graph representation.
- Mapping code can optionally use OBO-derived synonyms/relations to improve lexical matching.
- Tests prove that OBO integration is deterministic and does not break existing golden mappings.

## Out of Scope

- Full DL reasoning (e.g., complete OWL2 DL reasoners).
- Supporting arbitrary OBO ontologies beyond the curated set used by DFPS.
