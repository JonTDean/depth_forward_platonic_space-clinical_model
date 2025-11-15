use serde::{Deserialize, Serialize};

/// Minimal metadata for an OBO Foundry ontology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OboOntology {
    pub id: &'static str,
    pub name: &'static str,
    pub iri: &'static str,
    pub description: &'static str,
}

pub const NCIT_OBO: OboOntology = OboOntology {
    id: "NCIT",
    name: "NCI Thesaurus OBO",
    iri: "http://purl.obolibrary.org/obo/ncit.owl",
    description: "National Cancer Institute Thesaurus (OBO distribution).",
};

pub const MONDO_OBO: OboOntology = OboOntology {
    id: "MONDO",
    name: "Mondo Disease Ontology",
    iri: "http://purl.obolibrary.org/obo/mondo.owl",
    description: "Unified disease ontology combining multiple sources.",
};

static ONTOLOGIES: [OboOntology; 2] = [NCIT_OBO, MONDO_OBO];

pub fn list_ontologies() -> &'static [OboOntology] {
    &ONTOLOGIES
}

pub fn lookup_ontology(value: &str) -> Option<&'static OboOntology> {
    ONTOLOGIES.iter().find(|ont| {
        ont.id.eq_ignore_ascii_case(value)
            || ont.iri.eq_ignore_ascii_case(value)
            || value.eq_ignore_ascii_case(ont.name)
    })
}
