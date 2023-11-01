use super::ParseContext;
use crate::parse::annotations::parse_annotation;
use sdml_core::error::Error;
use sdml_core::model::annotations::{AnnotationOnlyBody, HasAnnotations};
use sdml_core::model::definitions::Definition;
use sdml_core::model::HasSourceSpan;
use sdml_core::syntax::{
    NODE_KIND_ANNOTATION, NODE_KIND_DATA_TYPE_DEF, NODE_KIND_ENTITY_DEF, NODE_KIND_ENUM_DEF,
    NODE_KIND_EVENT_DEF, NODE_KIND_LINE_COMMENT, NODE_KIND_PROPERTY_DEF, NODE_KIND_STRUCTURE_DEF,
    NODE_KIND_TYPE_CLASS_DEF, NODE_KIND_UNION_DEF,
};
use tree_sitter::TreeCursor;

// ------------------------------------------------------------------------------------------------
// Parser Functions
// ------------------------------------------------------------------------------------------------

pub(crate) fn parse_definition<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<Definition, Error> {
    rule_fn!("definition", cursor.node());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_DATA_TYPE_DEF => {
                        return Ok(parse_data_type_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_ENTITY_DEF => {
                        return Ok(parse_entity_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_ENUM_DEF => {
                        return Ok(parse_enum_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_EVENT_DEF => {
                        return Ok(parse_event_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_PROPERTY_DEF => {
                        return Ok(parse_property_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_STRUCTURE_DEF => {
                        return Ok(parse_structure_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_TYPE_CLASS_DEF => {
                        return Ok(parse_type_class_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_UNION_DEF => {
                        return Ok(parse_union_def(context, &mut node.walk())?.into());
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(
                            context,
                            RULE_NAME,
                            node,
                            [
                                NODE_KIND_DATA_TYPE_DEF,
                                NODE_KIND_ENTITY_DEF,
                                NODE_KIND_ENUM_DEF,
                                NODE_KIND_EVENT_DEF,
                                NODE_KIND_STRUCTURE_DEF,
                                NODE_KIND_UNION_DEF,
                            ]
                        );
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    rule_unreachable!(RULE_NAME, cursor);
}

pub(crate) fn parse_annotation_only_body<'a>(
    context: &mut ParseContext<'a>,
    cursor: &mut TreeCursor<'a>,
) -> Result<AnnotationOnlyBody, Error> {
    rule_fn!("annotation_only_body", cursor.node());
    let mut body = AnnotationOnlyBody::default().with_source_span(cursor.node().into());
    let mut has_next = cursor.goto_first_child();
    if has_next {
        while has_next {
            let node = cursor.node();
            context.check_if_error(&node, RULE_NAME)?;
            if node.is_named() {
                match node.kind() {
                    NODE_KIND_ANNOTATION => {
                        body.add_to_annotations(parse_annotation(context, &mut node.walk())?);
                    }
                    NODE_KIND_LINE_COMMENT => {}
                    _ => {
                        unexpected_node!(context, RULE_NAME, node, NODE_KIND_ANNOTATION);
                    }
                }
            }
            has_next = cursor.goto_next_sibling();
        }
        assert!(cursor.goto_parent());
    }
    Ok(body)
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod classes;
use classes::parse_type_class_def;

mod datatypes;
use datatypes::parse_data_type_def;

mod entities;
use entities::parse_entity_def;

mod enums;
use enums::parse_enum_def;

mod events;
use events::parse_event_def;

mod structures;
use structures::parse_structure_def;

mod unions;
use unions::parse_union_def;

mod properties;
use properties::parse_property_def;
