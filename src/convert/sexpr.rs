/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::api::ParseTree;
use crate::error::Error;
use std::io::Write;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn write_as_sexpr<W: Write>(tree: &ParseTree<'_>, w: &mut W) -> Result<(), Error> {
    w.write_all(tree.node().to_sexp().as_bytes())?;
    Ok(())
}

write_to_string!(to_sexpr_string, write_as_sexpr);

write_to_file!(to_sexpr_file, write_as_sexpr);

print_to_stdout!(print_sexpr, write_as_sexpr);

// ------------------------------------------------------------------------------------------------
// Private Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
