/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    color::rdf::{
        self as color, bnode_predicate_with_value, collection_element, end_bnode, end_collection,
        format_str, format_type_constructor, format_url, module_ref_qname, module_subject, mv_name,
        predicate_no_value, predicate_qname, predicate_with_value, predicate_with_value_list,
        property_subject, start_bnode, start_collection, thing_qname, thing_subject,
        type_ref_qname, type_subject, Separator, INDENT_PREDICATE,
    },
    GenerateToWriter,
};
use sdml_core::{
    cache::ModuleCache,
    error::Error,
    model::{
        annotations::{Annotation, AnnotationProperty, HasAnnotations},
        constraints::Constraint,
        definitions::{
            DatatypeDef, Definition, EntityDef, EnumDef, EventDef, HasMembers, HasVariants,
            PropertyDef, RdfDef, StructureDef, TypeClassDef, TypeVariant, UnionDef, ValueVariant,
        },
        identifiers::{Identifier, IdentifierReference},
        members::{HasCardinality, Member, Ordering, Uniqueness, DEFAULT_CARDINALITY},
        members::{HasType, TypeReference},
        modules::Module,
        values::{
            MappingValue, SequenceMember, SequenceOfValues, SimpleValue, Value, ValueConstructor,
        },
        HasBody, HasName, HasNameReference, HasOptionalBody,
    },
    stdlib,
};
use std::{fmt::Display, io::Write};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct RdfModelGenerator {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RdfRepresentation {
    NTriples,
    Turtle,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

macro_rules! write_annotations {
    ($self:expr, $anns:expr, $module_name:expr, $writer:expr) => {
        for annotation in $anns {
            match &annotation {
                Annotation::Property(me) => {
                    $self.write_annotation_property(me, $module_name, $writer)?
                }
                Annotation::Constraint(me) => $self.write_constraint(me, $module_name, $writer)?,
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl GenerateToWriter<RdfRepresentation> for RdfModelGenerator {
    fn write_in_format<W>(
        &mut self,
        module: &Module,
        cache: &ModuleCache,
        writer: &mut W,
        _format: RdfRepresentation,
    ) -> Result<(), Error>
    where
        W: Write + Sized,
    {
        let module_name = module.name();

        if let Some(base) = module.base_uri() {
            writer.write_all(color::base_directive(base.as_str()).as_bytes())?;
            writer.write_all(
                color::prefix_directive(module_name.as_ref(), base.as_str()).as_bytes(),
            )?;
        }

        let body = module.body();
        let mut imported_modules = body.imported_modules();

        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME)) {
            writer.write_all(
                color::prefix_directive(stdlib::owl::MODULE_NAME, stdlib::owl::MODULE_URL)
                    .as_bytes(),
            )?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME)) {
            writer.write_all(
                color::prefix_directive(stdlib::rdf::MODULE_NAME, stdlib::rdf::MODULE_URL)
                    .as_bytes(),
            )?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME)) {
            writer.write_all(
                color::prefix_directive(stdlib::rdfs::MODULE_NAME, stdlib::rdfs::MODULE_URL)
                    .as_bytes(),
            )?;
        }
        if !imported_modules.contains(&Identifier::new_unchecked(stdlib::sdml::MODULE_NAME)) {
            writer.write_all(
                color::prefix_directive(stdlib::sdml::MODULE_NAME, stdlib::sdml::MODULE_URL)
                    .as_bytes(),
            )?;
        }

        for import in &imported_modules {
            if let Some(uri) = cache.url_for_identifier(import) {
                writer
                    .write_all(color::prefix_directive(import.as_ref(), uri.as_str()).as_bytes())?;
            }
        }

        writer.write_all(b"\n")?;

        writer.write_all(module_subject(module_name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    color::type_ref_qname(
                        stdlib::owl::MODULE_NAME,
                        stdlib::owl::CLASS_ONTOLOGY_NAME,
                    ),
                    color::type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_MODULE_NAME,
                    ),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;
        if let Some(version_info) = module.version_info() {
            writer.write_all(
                color::predicate_with_value(
                    stdlib::owl::MODULE_NAME,
                    stdlib::owl::PROP_VERSION_INFO_NAME,
                    format_str(version_info),
                    Separator::Predicate,
                )
                .as_bytes(),
            )?;
        }
        if let Some(version_uri) = module.version_uri() {
            writer.write_all(
                color::predicate_with_value(
                    stdlib::owl::MODULE_NAME,
                    stdlib::owl::PROP_VERSION_IRI_NAME,
                    format_url(version_uri),
                    Separator::Predicate,
                )
                .as_bytes(),
            )?;
        }

        imported_modules.remove(&Identifier::new_unchecked(stdlib::owl::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::rdf::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::rdfs::MODULE_NAME));
        imported_modules.remove(&Identifier::new_unchecked(stdlib::xsd::MODULE_NAME));
        for import in &imported_modules {
            if let Some(url) = cache.url_for_identifier(import) {
                writer.write_all(
                    color::predicate_with_value(
                        stdlib::owl::MODULE_NAME,
                        stdlib::owl::PROP_IMPORTS_NAME,
                        format_url(url),
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        write_annotations!(self, body.annotations(), module_name, writer);

        writer.write_all(
            predicate_with_value(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::PROP_SRC_LABEL_NAME,
                color::format_str(module_name),
                Separator::Statement,
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;

        for definition in body.definitions() {
            match &definition {
                Definition::Datatype(v) => self.write_datatype(v, module_name, writer)?,
                Definition::Entity(v) => self.write_entity(v, module_name, writer)?,
                Definition::Enum(v) => self.write_enumeration(v, module_name, writer)?,
                Definition::Event(v) => self.write_event(v, module_name, writer)?,
                Definition::Property(v) => self.write_property(v, module_name, writer)?,
                Definition::Rdf(v) => self.write_rdf(v, module_name, writer)?,
                Definition::Structure(v) => self.write_structure(v, module_name, writer)?,
                Definition::TypeClass(v) => self.write_type_class(v, module_name, writer)?,
                Definition::Union(v) => self.write_union(v, module_name, writer)?,
            }
        }

        Ok(())
    }
}

impl RdfModelGenerator {
    fn write_datatype(
        &mut self,
        me: &DatatypeDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                type_ref_qname(stdlib::rdfs::MODULE_NAME, stdlib::rdfs::CLASS_DATATYPE_NAME),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        let (base_module, base_type) = self.qualified_idref(module_name, me.base_type());
        writer.write_all(
            predicate_with_value(
                stdlib::owl::MODULE_NAME,
                stdlib::owl::PROP_ON_DATATYPE_NAME,
                type_ref_qname(base_module, base_type),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            let (facets, other): (Vec<_>, Vec<_>) = body.annotations().partition(|ann| {
                if let Annotation::Property(prop) = ann {
                    prop.is_datatype_facet()
                } else {
                    false
                }
            });

            if !facets.is_empty() {
                writer.write_all(
                    format!(
                        "{} {}{}",
                        predicate_no_value(
                            stdlib::owl::MODULE_NAME,
                            stdlib::owl::PROP_WITH_RESTRICTIONS_NAME,
                            Separator::InlineNone,
                        ),
                        start_collection(),
                        Separator::None
                    )
                    .as_bytes(),
                )?;
                let last_facet = facets.len() - 1;
                for (i, facet) in facets.iter().enumerate() {
                    if let Some(facet) = facet.as_annotation_property() {
                        self.write_facet_property(
                            facet,
                            module_name,
                            if i < last_facet {
                                Separator::Object
                            } else {
                                Separator::None
                            },
                            writer,
                        )?;
                    } else {
                        unreachable!()
                    }
                }
                writer.write_all(
                    format!(
                        "{INDENT_PREDICATE}{}{}",
                        end_collection(),
                        Separator::Predicate
                    )
                    .as_bytes(),
                )?;
            }

            write_annotations!(self, other.iter(), module_name, writer);
        }

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_entity(
        &mut self,
        me: &EntityDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_ENTITY_NAME),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            if body.has_members() {
                let member_list = body
                    .members()
                    .map(|m| thing_qname(module_name, mv_name(name, m.name())))
                    .collect::<Vec<_>>();
                writer.write_all(
                    predicate_with_value_list(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::PROP_HAS_MEMBER_NAME,
                        &member_list,
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        self.write_defn_end(module_name, name, writer)?;

        if let Some(body) = me.body() {
            for member in body.members() {
                self.write_member(member, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_member(
        &mut self,
        me: &Member,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(property_subject(module_name, name).as_bytes())?;

        let more = if let Some(_property) = me.as_property_reference() {
            writer.write_all(
                predicate_with_value_list(
                    stdlib::rdf::MODULE_NAME,
                    stdlib::rdf::PROP_TYPE_NAME,
                    &[
                        type_ref_qname(stdlib::rdf::MODULE_NAME, stdlib::rdf::CLASS_PROPERTY_NAME),
                        type_ref_qname(
                            stdlib::sdml::MODULE_NAME,
                            stdlib::sdml::CLASS_ROLE_REFERENCE_NAME,
                        ),
                    ],
                    Separator::Predicate,
                )
                .as_bytes(),
            )?;

            self.write_member_type(me, module_name, writer)?
        } else if let Some(def) = me.as_definition() {
            writer.write_all(
                predicate_with_value_list(
                    stdlib::rdf::MODULE_NAME,
                    stdlib::rdf::PROP_TYPE_NAME,
                    &[
                        type_ref_qname(stdlib::rdf::MODULE_NAME, stdlib::rdf::CLASS_PROPERTY_NAME),
                        type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_MEMBER_NAME),
                    ],
                    Separator::Predicate,
                )
                .as_bytes(),
            )?;

            writer.write_all(
                predicate_with_value(
                    stdlib::rdfs::MODULE_NAME,
                    stdlib::rdfs::PROP_DOMAIN_NAME,
                    type_ref_qname(module_name, parent),
                    Separator::Predicate,
                )
                .as_bytes(),
            )?;
            let more = self.write_member_type(me, module_name, writer)?;

            if let Some(body) = def.body() {
                write_annotations!(self, body.annotations(), module_name, writer);
            }

            more
        } else {
            unreachable!();
        };

        self.write_defn_end(module_name, name, writer)?;

        if !more.is_empty() {
            writer.write_all(more.as_bytes())?;
        }

        Ok(())
    }

    fn write_member_type(
        &mut self,
        me: &Member,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<String, Error> {
        let mut more = String::new();

        if let Some(def) = me.as_definition() {
            match def.target_type() {
                TypeReference::Unknown => {
                    writer.write_all(
                        predicate_with_value(
                            stdlib::rdfs::MODULE_NAME,
                            stdlib::rdfs::PROP_RANGE_NAME,
                            type_ref_qname(
                                stdlib::sdml::MODULE_NAME,
                                stdlib::sdml::CLASS_UNKNOWN_NAME,
                            ),
                            Separator::Predicate,
                        )
                        .as_bytes(),
                    )?;
                }
                TypeReference::Type(name) => {
                    let (ty_module, ty_name) = self.qualified_idref(module_name, name);
                    writer.write_all(
                        predicate_with_value(
                            stdlib::rdfs::MODULE_NAME,
                            stdlib::rdfs::PROP_RANGE_NAME,
                            type_ref_qname(ty_module, ty_name),
                            Separator::Predicate,
                        )
                        .as_bytes(),
                    )?;
                    let card = def.target_cardinality();
                    if card != &DEFAULT_CARDINALITY {
                        if let Some(ordering) = card.ordering() {
                            writer.write_all(
                                predicate_with_value(
                                    stdlib::sdml::MODULE_NAME,
                                    stdlib::sdml::PROP_ORDERING_NAME,
                                    if ordering == Ordering::Ordered {
                                        thing_qname(
                                            stdlib::sdml::MODULE_NAME,
                                            stdlib::sdml::IND_ORDERED_NAME,
                                        )
                                    } else {
                                        thing_qname(
                                            stdlib::sdml::MODULE_NAME,
                                            stdlib::sdml::IND_UNORDERED_NAME,
                                        )
                                    },
                                    Separator::Predicate,
                                )
                                .as_bytes(),
                            )?;
                        }
                        if let Some(uniqueness) = card.uniqueness() {
                            writer.write_all(
                                predicate_with_value(
                                    stdlib::sdml::MODULE_NAME,
                                    stdlib::sdml::PROP_UNIQUENESS_NAME,
                                    if uniqueness == Uniqueness::Unique {
                                        thing_qname(
                                            stdlib::sdml::MODULE_NAME,
                                            stdlib::sdml::IND_UNIQUE_NAME,
                                        )
                                    } else {
                                        thing_qname(
                                            stdlib::sdml::MODULE_NAME,
                                            stdlib::sdml::IND_NONUNIQUE_NAME,
                                        )
                                    },
                                    Separator::Predicate,
                                )
                                .as_bytes(),
                            )?;
                        }
                        let range = card.range();
                        writer.write_all(
                            predicate_with_value(
                                stdlib::owl::MODULE_NAME,
                                stdlib::owl::PROP_MIN_CARDINALITY_NAME,
                                format_type_constructor(
                                    stdlib::xsd::MODULE_NAME,
                                    stdlib::xsd::DT_NONNEGATIVE_INTEGER_NAME,
                                    range.min_occurs().to_string(),
                                ),
                                Separator::Predicate,
                            )
                            .as_bytes(),
                        )?;
                        if let Some(max) = range.max_occurs() {
                            writer.write_all(
                                predicate_with_value(
                                    stdlib::owl::MODULE_NAME,
                                    stdlib::owl::PROP_MAX_CARDINALITY_NAME,
                                    format_type_constructor(
                                        stdlib::xsd::MODULE_NAME,
                                        stdlib::xsd::DT_NONNEGATIVE_INTEGER_NAME,
                                        max.to_string(),
                                    ),
                                    Separator::Predicate,
                                )
                                .as_bytes(),
                            )?;
                        }
                    }
                }
                TypeReference::FeatureSet(name) => {
                    let (fs_module, fs_name) = self.qualified_idref(module_name, name);
                    writer.write_all(
                        predicate_with_value(
                            stdlib::rdfs::MODULE_NAME,
                            stdlib::rdfs::PROP_RANGE_NAME,
                            type_ref_qname(fs_module, fs_name),
                            Separator::Predicate,
                        )
                        .as_bytes(),
                    )?;
                    // TODO cardinality
                    more = format!(
                        "{}{}{}",
                        thing_subject(fs_module, fs_name),
                        predicate_with_value(
                            stdlib::rdf::MODULE_NAME,
                            stdlib::rdf::PROP_TYPE_NAME,
                            type_ref_qname(
                                stdlib::sdml::MODULE_NAME,
                                stdlib::sdml::CLASS_FEATURE_SET_NAME
                            ),
                            Separator::Statement
                        ),
                        Separator::None
                    )
                }
                TypeReference::MappingType(_map) => {
                    // 1. throw hands in the air, this is a mess.
                    // TODO cardinality
                }
            }
        } else if let Some(_property) = me.as_property_reference() {
            // 1. lookup `property` in cache
            // 2. find member name as `role` in property
            // 3. call self with member type of property
        } else {
            unreachable!()
        }

        Ok(more)
    }

    fn write_enumeration(
        &mut self,
        me: &EnumDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_ENUMERATION_NAME,
                    ),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            if body.has_variants() {
                let variant_list = body
                    .variants()
                    .map(|v| thing_qname(module_name, mv_name(name, v.name())))
                    .collect::<Vec<_>>();
                writer.write_all(
                    predicate_with_value_list(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::PROP_HAS_VALUE_VARIANT_NAME,
                        &variant_list,
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        self.write_defn_end(module_name, name, writer)?;

        if let Some(body) = me.body() {
            for variant in body.variants() {
                self.write_value_variant(variant, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_value_variant(
        &mut self,
        me: &ValueVariant,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = mv_name(parent, me.name());

        writer.write_all(thing_subject(module_name, name.clone()).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(
                        stdlib::owl::MODULE_NAME,
                        stdlib::owl::CLASS_NAMED_INDIVIDUAL_NAME,
                    ),
                    type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_VALUE_VARIANT_NAME,
                    ),
                    type_ref_qname(module_name, parent),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);
        }

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_event(
        &mut self,
        me: &EventDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_EVENT_NAME),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        let (source_module, source_name) = self.qualified_idref(module_name, me.event_source());
        writer.write_all(
            predicate_with_value(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::PROP_HAS_SOURCE_ENTITY_NAME,
                predicate_qname(source_module, source_name),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            if body.has_members() {
                let member_list = body
                    .members()
                    .map(|m| thing_qname(module_name, mv_name(name, m.name())))
                    .collect::<Vec<_>>();
                writer.write_all(
                    predicate_with_value_list(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::PROP_HAS_MEMBER_NAME,
                        &member_list,
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        self.write_defn_end(module_name, name, writer)?;

        if let Some(body) = me.body() {
            for member in body.members() {
                self.write_member(member, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_structure(
        &mut self,
        me: &StructureDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_STRUCTURE_NAME,
                    ),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            if body.has_members() {
                let member_list = body
                    .members()
                    .map(|m| thing_qname(module_name, mv_name(name, m.name())))
                    .collect::<Vec<_>>();
                writer.write_all(
                    predicate_with_value_list(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::PROP_HAS_MEMBER_NAME,
                        &member_list,
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        self.write_defn_end(module_name, name, writer)?;

        if let Some(body) = me.body() {
            for member in body.members() {
                self.write_member(member, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_union(
        &mut self,
        me: &UnionDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_UNION_NAME),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            if body.has_variants() {
                let variant_list = body
                    .variants()
                    .map(|v| thing_qname(module_name, mv_name(name, v.name())))
                    .collect::<Vec<_>>();
                writer.write_all(
                    predicate_with_value_list(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::PROP_HAS_TYPE_VARIANT_NAME,
                        &variant_list,
                        Separator::Predicate,
                    )
                    .as_bytes(),
                )?;
            }
        }

        self.write_defn_end(module_name, name, writer)?;

        if let Some(body) = me.body() {
            for variant in body.variants() {
                self.write_type_variant(variant, module_name, name, writer)?;
            }
        }

        Ok(())
    }

    fn write_type_variant(
        &mut self,
        me: &TypeVariant,
        module_name: &Identifier,
        parent: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = format!("{parent}__{}", me.name());

        writer.write_all(type_subject(module_name, name.clone()).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_TYPE_VARIANT_NAME,
                    ),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;
        writer.write_all(
            predicate_with_value(
                stdlib::rdfs::MODULE_NAME,
                stdlib::rdfs::PROP_SUB_CLASS_OF_NAME,
                type_ref_qname(module_name, parent),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        let (ty_module, ty_name) = self.qualified_idref(module_name, me.name_reference());
        writer.write_all(
            predicate_with_value(
                stdlib::owl::MODULE_NAME,
                stdlib::owl::PROP_EQUIVALENT_CLASS_NAME,
                predicate_qname(ty_module, ty_name),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);
        }

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_property(
        &mut self,
        me: &PropertyDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_PROPERTY_NAME),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        if let Some(body) = me.body() {
            write_annotations!(self, body.annotations(), module_name, writer);

            // TODO: roles
        }

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_rdf(
        &mut self,
        me: &RdfDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();

        writer.write_all(thing_subject(module_name, name).as_bytes())?;

        writer.write_all(type_ref_qname(module_name, name).as_bytes())?;

        write_annotations!(self, me.body().annotations(), module_name, writer);

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_type_class(
        &mut self,
        me: &TypeClassDef,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let name = me.name();
        writer.write_all(type_subject(module_name, name).as_bytes())?;
        writer.write_all(
            predicate_with_value_list(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                &[
                    type_ref_qname(stdlib::owl::MODULE_NAME, stdlib::owl::CLASS_CLASS_NAME),
                    type_ref_qname(
                        stdlib::sdml::MODULE_NAME,
                        stdlib::sdml::CLASS_TYPE_CLASS_NAME,
                    ),
                ],
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        self.write_defn_end(module_name, name, writer)?;

        Ok(())
    }

    fn write_defn_end<S>(
        &mut self,
        module_name: &Identifier,
        name: S,
        writer: &mut dyn Write,
    ) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        writer.write_all(
            predicate_with_value(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::PROP_SRC_LABEL_NAME,
                format_str(name),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;
        writer.write_all(
            predicate_with_value(
                stdlib::rdfs::MODULE_NAME,
                stdlib::rdfs::PROP_IS_DEFINED_BY_NAME,
                module_ref_qname(module_name),
                Separator::Statement,
            )
            .as_bytes(),
        )?;
        writer.write_all(b"\n")?;
        Ok(())
    }

    fn write_annotation_property(
        &mut self,
        me: &AnnotationProperty,
        module_name: &Identifier,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let (module, name) = self.qualified_idref(module_name, me.name_reference());

        writer.write_all(
            predicate_with_value(
                module,
                name,
                self.value_to_string(me.value(), module_name),
                Separator::Predicate,
            )
            .as_bytes(),
        )?;

        Ok(())
    }

    fn write_facet_property(
        &mut self,
        me: &AnnotationProperty,
        module_name: &Identifier,
        sep: Separator,
        writer: &mut dyn Write,
    ) -> Result<(), Error> {
        let (module, name) = self.qualified_idref(module_name, me.name_reference());
        let value = self.value_to_string(me.value(), module_name);
        writer.write_all(bnode_predicate_with_value(module, name, value, sep).as_bytes())?;

        Ok(())
    }

    fn write_constraint(
        &mut self,
        _me: &Constraint,
        _module_name: &Identifier,
        _writer: &mut dyn Write,
    ) -> Result<(), Error> {
        todo!();
    }

    fn qualified_idref_string(
        &self,
        module_name: &Identifier,
        idref: &IdentifierReference,
    ) -> String {
        let (module, ty_name) = self.qualified_idref(module_name, idref);
        color::type_ref_qname(module, ty_name)
    }

    fn qualified_idref<'a>(
        &self,
        module_name: &'a Identifier,
        idref: &'a IdentifierReference,
    ) -> (&'a Identifier, &'a Identifier) {
        match idref {
            IdentifierReference::Identifier(name) => (module_name, name),
            IdentifierReference::QualifiedIdentifier(name) => (name.module(), name.member()),
        }
    }

    fn value_to_string(&mut self, me: &Value, module_name: &Identifier) -> String {
        match me {
            Value::Simple(v) => self.simple_value_to_string(v),
            Value::ValueConstructor(v) => self.value_constructor_to_string(v, module_name),
            Value::Mapping(v) => self.mapping_value_to_string(v, module_name),
            Value::Reference(v) => self.type_reference_to_string(v, module_name),
            Value::List(v) => self.list_value_to_string(v, module_name),
        }
    }

    fn simple_value_to_string(&mut self, me: &SimpleValue) -> String {
        match me {
            SimpleValue::Boolean(v) => v.to_string(),
            SimpleValue::Double(v) => color::format_type_constructor(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::DT_DOUBLE_NAME,
                &v.to_string(),
            ),
            SimpleValue::Decimal(v) => color::format_type_constructor(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::DT_DECIMAL_NAME,
                &v.to_string(),
            ),
            SimpleValue::Integer(v) => color::format_number(&v.to_string()),
            SimpleValue::Unsigned(v) => color::format_type_constructor(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::DT_UNSIGNED_NAME,
                &v.to_string(),
            ),
            SimpleValue::String(v) => color::format_lang_str(v),
            SimpleValue::IriReference(v) => color::format_url(v),
            SimpleValue::Binary(_) => todo!(),
        }
    }

    fn value_constructor_to_string(
        &mut self,
        me: &ValueConstructor,
        module_name: &Identifier,
    ) -> String {
        let (module_name, ty_name) = self.qualified_idref(module_name, me.type_name());
        color::format_type_constructor(module_name, ty_name, &me.value().to_string())
    }

    fn mapping_value_to_string(&mut self, me: &MappingValue, module_name: &Identifier) -> String {
        format!(
            "{INDENT_PREDICATE}{}
{}{}{}
{INDENT_PREDICATE}{}",
            start_bnode(),
            collection_element(predicate_with_value(
                stdlib::rdf::MODULE_NAME,
                stdlib::rdf::PROP_TYPE_NAME,
                type_ref_qname(stdlib::sdml::MODULE_NAME, stdlib::sdml::CLASS_MAP_TYPE_NAME,),
                Separator::Predicate,
            )),
            collection_element(predicate_with_value(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::PROP_HAS_DOMAIN_VALUE_NAME,
                self.simple_value_to_string(me.domain()),
                Separator::Predicate,
            )),
            collection_element(predicate_with_value(
                stdlib::sdml::MODULE_NAME,
                stdlib::sdml::PROP_HAS_RANGE_VALUE_NAME,
                self.value_to_string(me.range(), module_name),
                Separator::None,
            )),
            end_bnode(),
        )
    }

    fn type_reference_to_string(
        &mut self,
        me: &IdentifierReference,
        module_name: &Identifier,
    ) -> String {
        self.qualified_idref_string(module_name, me)
    }

    fn list_value_to_string(&mut self, me: &SequenceOfValues, module_name: &Identifier) -> String {
        let mut buffer = format!(" {}\n", start_collection());

        for member in me.iter() {
            let value = match member {
                SequenceMember::Simple(v) => self.simple_value_to_string(v),
                SequenceMember::ValueConstructor(v) => {
                    self.value_constructor_to_string(v, module_name)
                }
                SequenceMember::Reference(v) => self.type_reference_to_string(v, module_name),
                SequenceMember::Mapping(v) => self.mapping_value_to_string(v, module_name),
            };
            buffer.push_str(&collection_element(value));
        }

        buffer.push_str(INDENT_PREDICATE);
        buffer.push_str(&end_collection());
        buffer
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RdfRepresentation {
    fn default() -> Self {
        Self::Turtle
    }
}

impl Display for RdfRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self, f.alternate()) {
                (Self::NTriples, false) => "NTriples",
                (Self::NTriples, true) => "nt",
                (Self::Turtle, false) => "Turtle",
                (Self::Turtle, true) => "ttl",
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
