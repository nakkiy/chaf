use crate::core::ast::{AstNode, Pattern};

pub enum Evaluator {
    And(Box<Evaluator>, Box<Evaluator>),
    Or(Box<Evaluator>, Box<Evaluator>),
    Not(Box<Evaluator>),
    Contains(String),
}

impl Evaluator {
    pub fn evaluate(&self, line: &str) -> bool {
        match self {
            Evaluator::And(lhs, rhs) => lhs.evaluate(line) && rhs.evaluate(line),
            Evaluator::Or(lhs, rhs) => lhs.evaluate(line) || rhs.evaluate(line),
            Evaluator::Not(inner) => !inner.evaluate(line),
            Evaluator::Contains(s) => line.contains(s),
        }
    }

    // Converts an AST node into an Evaluator structure
    pub fn from_ast(ast: &AstNode) -> Self {
        match ast {
            AstNode::AndNode(lhs, rhs) => {
                Evaluator::And(Box::new(Self::from_ast(lhs)), Box::new(Self::from_ast(rhs)))
            }
            AstNode::OrNode(lhs, rhs) => {
                Evaluator::Or(Box::new(Self::from_ast(lhs)), Box::new(Self::from_ast(rhs)))
            }
            AstNode::NotNode(inner) => Evaluator::Not(Box::new(Self::from_ast(inner))),
            AstNode::Match(Pattern::Literal(s)) => Evaluator::Contains(s.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::{AstNode, Pattern};

    fn literal(s: &str) -> AstNode {
        AstNode::Match(Pattern::Literal(s.to_string()))
    }

    #[test]
    fn test_literal_match() {
        let ast = literal("ERROR");
        let eval = Evaluator::from_ast(&ast);
        assert!(eval.evaluate("this is ERROR"));
        assert!(!eval.evaluate("this is OK"));
    }

    #[test]
    fn test_and_match() {
        let ast = AstNode::AndNode(Box::new(literal("foo")), Box::new(literal("bar")));
        let eval = Evaluator::from_ast(&ast);
        assert!(eval.evaluate("foo bar"));
        assert!(!eval.evaluate("foo only"));
    }

    #[test]
    fn test_or_match() {
        let ast = AstNode::OrNode(Box::new(literal("foo")), Box::new(literal("bar")));
        let eval = Evaluator::from_ast(&ast);
        assert!(eval.evaluate("contains foo"));
        assert!(eval.evaluate("contains bar"));
        assert!(!eval.evaluate("neither"));
    }

    #[test]
    fn test_not_match() {
        let ast = AstNode::NotNode(Box::new(literal("DEBUG")));
        let eval = Evaluator::from_ast(&ast);
        assert!(!eval.evaluate("DEBUG line"));
        assert!(eval.evaluate("INFO line"));
    }
}
