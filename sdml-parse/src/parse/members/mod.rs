use super::ParseContext;
use crate::parse::identifiers::{parse_identifier, parse_identifier_reference};
use sdml_core::error::{invalid_value_for_type, Error};
use sdml_core::model::identifiers::{Identifier, QualifiedIdentifier};
use sdml_core::model::members::{Cardinality, MappingType, Ordering, TypeReference, Uniqueness};
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    FIELD_NAME_DOMAIN, FIELD_NAME_MAX, FIELD_NAME_MIN, FIELD_NAME_ORDERING, FIELD_NAME_RANGE,
    FIELD_NAME_UNIQUENESS, NAME_SDML, NODE_KIND_BUILTIN_SIMPLE_TYPE,
    NODE_KIND_IDENTIFIER_REFERENCE, NODE_KIND_MAPPING_TYPE, NODE_KIND_UNKNOWN_TYPE,
    NODE_KIND_UNSIGNED,
};
use std::str::FromStr;
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_cardinality_expression<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Cardinality, Error> {
    let node = cursor.node();
    rule_fn!("cardinality_expression", node);

    let ordering = if let Some(child) = node.child_by_field_name(FIELD_NAME_ORDERING) {
        context.check_if_error(&child, RULE_NAME)?;
        Ordering::from_str(context.node_source(&child)?)?
    } else {
        Ordering::default()
    };

    let uniqueness = if let Some(child) = node.child_by_field_name(FIELD_NAME_UNIQUENESS) {
        context.check_if_error(&child, RULE_NAME)?;
        Uniqueness::from_str(context.node_source(&child)?)?
    } else {
        Uniqueness::default()
    };

    let child = node.child_by_field_name(FIELD_NAME_MIN).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let text = context.node_source(&child)?;
    let min = u32::from_str(text).map_err(|_| invalid_value_for_type(text, NODE_KIND_UNSIGNED))?;

    let expr = if let Some(child) = node.child_by_field_name("range") {
        if let Some(child) = child.child_by_field_name(FIELD_NAME_MAX) {
            context.check_if_error(&child, RULE_NAME)?;
            let text = context.node_source(&child)?;
            let max = u32::from_str(text)
                .map_err(|_| invalid_value_for_type(text, NODE_KIND_UNSIGNED))?;
            Cardinality::new_range(min, max)
        } else {
            Cardinality::new_unbounded(min)
        }
    } else {
        Cardinality::new_single(min)
    };
    Ok(expr
        .with_source_span(node.into())
        .with_ordering(ordering)
        .with_uniqueness(uniqueness))
}

pub(crate) fn parse_type_reference<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
    feature_set: bool,
) -> Result<TypeReference, Error> {
    rule_fn!("type_reference", cursor.node());
    let mut has_next = cursor.goto_first_child();
    while has_next {
        let node = cursor.node();
        context.check_if_error(&node, RULE_NAME)?;
        if node.is_named() {
            match node.kind() {
                NODE_KIND_UNKNOWN_TYPE => {
                    return Ok(TypeReference::Unknown);
                }
                NODE_KIND_IDENTIFIER_REFERENCE => {
                    let reference = parse_identifier_reference(context, &mut node.walk())?;
                    if feature_set {
                        return Ok(TypeReference::FeatureSet(reference));
                    } else {
                        return Ok(TypeReference::Type(reference));
                    }
                }
                NODE_KIND_BUILTIN_SIMPLE_TYPE => {
                    let module = Identifier::new_unchecked(NAME_SDML);
                    let member = parse_identifier(context, &node)?.with_source_span(node.into());
                    return Ok(TypeReference::Type(
                        QualifiedIdentifier::new(module, member)
                            .with_source_span(node.into())
                            .into(),
                    ));
                }
                NODE_KIND_MAPPING_TYPE => {
                    let mapping = parse_mapping_type(context, &mut node.walk())?;
                    return Ok(TypeReference::MappingType(mapping));
                }
                _ => {
                    unexpected_node!(
                        context,
                        RULE_NAME,
                        node,
                        [
                            NODE_KIND_UNKNOWN_TYPE,
                            NODE_KIND_IDENTIFIER_REFERENCE,
                            NODE_KIND_BUILTIN_SIMPLE_TYPE,
                        ]
                    );
                }
            }
        }
        has_next = cursor.goto_next_sibling();
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_mapping_type<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<MappingType, Error> {
    let node = cursor.node();
    rule_fn!("mapping_type", node);

    let child = node.child_by_field_name(FIELD_NAME_DOMAIN).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let domain = parse_type_reference(context, &mut child.walk(), false)?;

    let child = node.child_by_field_name(FIELD_NAME_RANGE).unwrap();
    context.check_if_error(&child, RULE_NAME)?;
    let range = parse_type_reference(context, &mut child.walk(), false)?;

    Ok(MappingType::new(domain, range).with_source_span(node.into()))
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod groups;
pub(crate) use groups::parse_member_group;

mod member;
pub(crate) use member::parse_member;