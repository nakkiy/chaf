use crate::core::ast::AstNode;
use crate::core::evaluator::Evaluator;
use anyhow::{Context, Result};

// Converts an AST node into an executable filter closure
pub fn build_filter(
    ast: &AstNode,
    invert: bool,
) -> Result<impl Fn(&[u8]) -> Result<bool> + Send + Sync + 'static> {
    let evaluator = Evaluator::from_ast(ast);

    Ok(move |input: &[u8]| {
        let text = std::str::from_utf8(input).with_context(|| "Input is not valid UTF-8")?;
        let matched = evaluator.evaluate(text);
        Ok(if invert { matched } else { !matched })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::{AstNode, Pattern};

    fn lit(s: &str) -> AstNode {
        AstNode::Match(Pattern::Literal(s.to_string()))
    }

    #[test]
    fn test_match_without_invert() {
        let ast = lit("ERROR");
        let filter = build_filter(&ast, false).expect("filter build failed");

        // Contains "ERROR" → matched=true → invert=false → returns false (excluded)
        assert_eq!(filter(b"this is ERROR").unwrap(), false);

        // Does not contain → matched=false → invert=false → returns true (included)
        assert_eq!(filter(b"all good").unwrap(), true);
    }

    #[test]
    fn test_match_with_invert() {
        let ast = lit("ERROR");
        let filter = build_filter(&ast, true).expect("filter build failed");

        // Contains "ERROR" → matched=true → invert=true → returns true (included)
        assert_eq!(filter(b"this is ERROR").unwrap(), true);

        // Does not contain → matched=false → invert=true → returns false (excluded)
        assert_eq!(filter(b"all good").unwrap(), false);
    }

    #[test]
    fn test_and_expression_invert() {
        let ast = AstNode::AndNode(Box::new(lit("foo")), Box::new(lit("bar")));

        let f1 = build_filter(&ast, false).unwrap();
        let f2 = build_filter(&ast, true).unwrap();

        assert_eq!(f1(b"foo bar").unwrap(), false); // matched → invert=false → false
        assert_eq!(f1(b"foo only").unwrap(), true); // not matched → true

        assert_eq!(f2(b"foo bar").unwrap(), true); // matched → invert=true → true
        assert_eq!(f2(b"foo only").unwrap(), false); // not matched → invert=true → false
    }

    #[test]
    fn test_not_expression_invert() {
        let ast = AstNode::NotNode(Box::new(lit("DEBUG")));

        let f = build_filter(&ast, false).unwrap();
        let g = build_filter(&ast, true).unwrap();

        assert_eq!(f(b"DEBUG").unwrap(), true); // matched=false → ! → false → invert=false → true
        assert_eq!(f(b"INFO").unwrap(), false); // matched=true → invert=false → false

        assert_eq!(g(b"DEBUG").unwrap(), false); // matched=false → ! → false → invert=true → false
        assert_eq!(g(b"INFO").unwrap(), true); // matched=true → invert=true → true
    }

    #[test]
    fn test_invalid_utf8() {
        let ast = lit("foo");
        let filter = build_filter(&ast, false).unwrap();

        let result = filter(&[0xff, 0xfe, 0xfd]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("UTF-8"));
    }
}
