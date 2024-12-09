use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_redscript() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_redscript) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Swift parser");
    }

    #[test]
    fn test_can_parse_basic_file() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Swift parser");

        let tree = parser
            .parse("_ = \"Hello!\"\n", None)
            .expect("Unable to parse!");

        assert_eq!(
            "(source_file (assignment target: (directly_assignable_expression (simple_identifier)) result: (line_string_literal text: (line_str_text))))",
            tree.root_node().to_sexp(),
        );
    }
}
