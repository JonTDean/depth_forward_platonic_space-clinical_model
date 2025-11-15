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
