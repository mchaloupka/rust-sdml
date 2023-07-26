use super::{
    Annotation, Comment, Definition, Identifier, IdentifierReference, QualifiedIdentifier, Span,
};
use std::{collections::HashSet, fmt::Debug, hash::Hash};
use url::Url;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules
// ------------------------------------------------------------------------------------------------

///
/// Corresponds the grammar rule `module`.
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Module {
    span: Option<Span>,
    comments: Vec<Comment>,
    name: Identifier,
    base: Option<Url>,
    body: ModuleBody,
}

///
/// Corresponds the grammar rule `module_body`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModuleBody {
    span: Option<Span>,
    comments: Vec<Comment>,
    imports: Vec<ImportStatement>,
    annotations: Vec<Annotation>,
    definitions: Vec<Definition>,
}

// ------------------------------------------------------------------------------------------------
// Public Types ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

///
/// Corresponds the grammar rule `import_statement`.
///
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImportStatement {
    span: Option<Span>,
    comments: Vec<Comment>,
    imports: Vec<Import>,
}

///
/// Corresponds the grammar rule `import`.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Import {
    /// Corresponds to the grammar rule `module_import`.
    Module(Identifier),
    /// Corresponds to the grammar rule `member_import`.
    Member(QualifiedIdentifier),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules
// ------------------------------------------------------------------------------------------------

impl Module {
    pub fn new(name: Identifier, body: ModuleBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            base: None,
            body,
        }
    }
    pub fn new_with_base(name: Identifier, base: Url, body: ModuleBody) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            name,
            base: Some(base),
            body,
        }
    }

    // --------------------------------------------------------------------------------------------

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate!(pub name => Identifier);

    get_and_mutate!(pub base => option Url);

    get_and_mutate!(pub body => ModuleBody);

    // --------------------------------------------------------------------------------------------

    is_complete_fn!(body);

    delegate!(imported_modules, HashSet<&Identifier>, body);

    delegate!(imported_types, HashSet<&QualifiedIdentifier>, body);

    delegate!(defined_names, HashSet<&Identifier>, body);

    delegate!(referenced_types, HashSet<&IdentifierReference>, body);

    delegate!(referenced_annotations, HashSet<&IdentifierReference>, body);
}

// ------------------------------------------------------------------------------------------------

impl ModuleBody {
    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    has_owned_annotations!();

    get_and_mutate_collection_of!(pub imports => Vec, ImportStatement);

    get_and_mutate_collection_of!(pub definitions => Vec, Definition);

    // --------------------------------------------------------------------------------------------

    pub fn is_complete(&self) -> bool {
        self.definitions().all(|d| d.is_complete())
    }

    // --------------------------------------------------------------------------------------------

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_modules())
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .flat_map(|stmt| stmt.imported_types())
            .collect()
    }

    pub fn defined_names(&self) -> HashSet<&Identifier> {
        self.definitions().map(|def| def.name()).collect()
    }

    pub fn referenced_types(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_types())
            .collect()
    }

    pub fn referenced_annotations(&self) -> HashSet<&IdentifierReference> {
        self.definitions()
            .flat_map(|def| def.referenced_annotations())
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ Modules ❱ Imports
// ------------------------------------------------------------------------------------------------

impl FromIterator<Import> for ImportStatement {
    fn from_iter<T: IntoIterator<Item = Import>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl ImportStatement {
    pub fn new(imports: Vec<Import>) -> Self {
        Self {
            span: None,
            comments: Default::default(),
            imports,
        }
    }

    with!(pub span (ts_span) => option Span);
    get_and_mutate!(pub span (ts_span) => option Span);

    get_and_mutate_collection_of!(pub comments => Vec, Comment);

    get_and_mutate_collection_of!(pub imports => Vec, Import);

    pub(crate) fn as_slice(&self) -> &[Import] {
        self.imports.as_slice()
    }

    pub fn imported_modules(&self) -> HashSet<&Identifier> {
        self.imports()
            .map(|imp| match imp {
                Import::Module(v) => v,
                Import::Member(v) => v.module(),
            })
            .collect()
    }

    pub fn imported_types(&self) -> HashSet<&QualifiedIdentifier> {
        self.imports()
            .filter_map(|imp| {
                if let Import::Member(imp) = imp {
                    Some(imp)
                } else {
                    None
                }
            })
            .collect()
    }
}

// ------------------------------------------------------------------------------------------------

impl From<Identifier> for Import {
    fn from(v: Identifier) -> Self {
        Self::Module(v)
    }
}

impl From<QualifiedIdentifier> for Import {
    fn from(v: QualifiedIdentifier) -> Self {
        Self::Member(v)
    }
}

enum_display_impl!(Import => Module, Member);

impl Import {
    pub fn has_ts_span(&self) -> bool {
        match self {
            Self::Module(v) => v.has_ts_span(),
            Self::Member(v) => v.has_ts_span(),
        }
    }

    pub fn ts_span(&self) -> Option<&Span> {
        match self {
            Self::Module(v) => v.ts_span(),
            Self::Member(v) => v.ts_span(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
