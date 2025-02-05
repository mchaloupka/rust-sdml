use crate::model::{constraints::QuantifiedSentence, identifiers::Identifier, Span};
use std::collections::HashSet;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Formal Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

/// Corresponds to the grammar rule `sequence_comprehension`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SequenceBuilder {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    variables: Variables,
    body: QuantifiedSentence,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Variables {
    Named(NamedVariables),
    Mapping(MappingVariable),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct NamedVariables {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    names: HashSet<Identifier>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MappingVariable {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    span: Option<Box<Span>>,
    domain: Identifier,
    range: Identifier,
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Formal Constraints ❱  Sequence Comprehensions
// ------------------------------------------------------------------------------------------------

impl_has_body_for!(SequenceBuilder, QuantifiedSentence);

impl_has_source_span_for!(SequenceBuilder);

impl SequenceBuilder {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub fn new<V, S>(variables: V, body: S) -> Self
    where
        V: Into<Variables>,
        S: Into<QuantifiedSentence>,
    {
        Self {
            span: Default::default(),
            variables: variables.into(),
            body: body.into(),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    pub fn set_variables<V>(&mut self, variables: V)
    where
        V: Into<Variables>,
    {
        self.variables = variables.into();
    }
}

// ------------------------------------------------------------------------------------------------

impl From<NamedVariables> for Variables {
    fn from(value: NamedVariables) -> Self {
        Self::Named(value)
    }
}

impl From<MappingVariable> for Variables {
    fn from(value: MappingVariable) -> Self {
        Self::Mapping(value)
    }
}

impl Variables {
    // --------------------------------------------------------------------------------------------
    // Variants
    // --------------------------------------------------------------------------------------------

    is_as_variant!(Named (NamedVariables) => is_named_set, as_named_set);

    is_as_variant!(Mapping (MappingVariable) => is_mapping, as_mapping);
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(NamedVariables);

impl FromIterator<Identifier> for NamedVariables {
    fn from_iter<T: IntoIterator<Item = Identifier>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl AsRef<HashSet<Identifier>> for NamedVariables {
    fn as_ref(&self) -> &HashSet<Identifier> {
        &self.names
    }
}

impl AsMut<HashSet<Identifier>> for NamedVariables {
    fn as_mut(&mut self) -> &mut HashSet<Identifier> {
        &mut self.names
    }
}

impl NamedVariables {
    pub fn new(names: HashSet<Identifier>) -> Self {
        Self {
            span: Default::default(),
            names,
        }
    }
}

impl NamedVariables {
    pub fn names(&self) -> impl Iterator<Item = &Identifier> {
        self.names.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl_has_source_span_for!(MappingVariable);

impl MappingVariable {
    // --------------------------------------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------------------------------------

    pub const fn new(domain: Identifier, range: Identifier) -> Self {
        Self {
            span: None,
            domain,
            range,
        }
    }

    // --------------------------------------------------------------------------------------------
    // Fields
    // --------------------------------------------------------------------------------------------

    get_and_set!(pub domain, set_domain => Identifier);

    get_and_set!(pub range, set_range => Identifier);
}
