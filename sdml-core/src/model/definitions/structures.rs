use crate::{
    error::Error,
    model::{
        annotations::{Annotation, HasAnnotations},
        check::Validate,
        definitions::{HasGroups, HasMembers},
        identifiers::{Identifier, IdentifierReference},
        members::{Member, MemberGroup},
        modules::Module,
        References, Span,
    },
};
use std::{collections::HashSet, fmt::Debug};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `structure_def`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureDef {
    span: Option<Span>,
    name: Identifier,
    body: Option<StructureBody>,
}

/// Corresponds to the grammar rule `structure_body`.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StructureBody {
    span: Option<Span>,
    annotations: Vec<Annotation>,
    members: Vec<Member>,
    groups: Vec<MemberGroup>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Type Definitions ❱ Structures
// ------------------------------------------------------------------------------------------------

impl_has_name_for!(StructureDef);

impl_has_optional_body_for!(StructureDef, StructureBody);

impl_has_source_span_for!(StructureDef);

impl_references_for!(StructureDef => delegate optional body);

impl_validate_for!(StructureDef => delegate optional body, false, true);

impl StructureDef {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new(name: Identifier) -> Self {
        Self {
            span: None,
            name,
            body: None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_annotations_for!(StructureBody);

impl_has_groups_for!(StructureBody);

impl_has_members_for!(StructureBody);

impl_has_source_span_for!(StructureBody);

impl_validate_for_annotations_and_members!(StructureBody);

impl References for StructureBody {
    fn referenced_types<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members().for_each(|m| m.referenced_types(names));
    }

    fn referenced_annotations<'a>(&'a self, names: &mut HashSet<&'a IdentifierReference>) {
        self.flat_members()
            .for_each(|m| m.referenced_annotations(names));
    }
}

impl StructureBody {
    // --------------------------------------------------------------------------------------------
    // Helpers
    // --------------------------------------------------------------------------------------------

    pub fn flat_members(&self) -> impl Iterator<Item = &Member> {
        self.members()
            .chain(self.groups().flat_map(|g| g.members()))
    }
}

// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------