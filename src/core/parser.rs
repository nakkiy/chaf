use crate::core::ast::{AstNode, Pattern};
use anyhow::{bail, Result};

/// DSL文字列を構文解析して AST（抽象構文木）を構築する
pub fn parse_query(query: &str) -> Result<AstNode> {
    if query.trim().is_empty() {
        bail!("クエリが空です");
    }

    let mut parser = Parser::new(query);
    let ast = parser.parse_expr()?;

    parser.consume_whitespace();
    if parser.peek().is_some() {
        bail!("構文エラー（未解析のトークンあり）: pos={}", parser.pos);
    }

    Ok(ast)
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn parse_expr(&mut self) -> Result<AstNode> {
        let mut node = self.parse_and()?;

        loop {
            self.consume_whitespace();
            if !self.consume_char('|') {
                break;
            }

            self.consume_whitespace();
            let rhs = self.parse_and()?;
            node = AstNode::OrNode(Box::new(node), Box::new(rhs));
        }

        Ok(node)
    }

    fn parse_and(&mut self) -> Result<AstNode> {
        let mut node = self.parse_not()?;

        loop {
            self.consume_whitespace();
            if !self.consume_char('&') {
                break;
            }

            self.consume_whitespace();
            let rhs = self.parse_not()?;
            node = AstNode::AndNode(Box::new(node), Box::new(rhs));
        }

        Ok(node)
    }

    fn parse_not(&mut self) -> Result<AstNode> {
        self.consume_whitespace();

        if self.consume_char('!') {
            let node = self.parse_not()?;
            Ok(AstNode::NotNode(Box::new(node)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<AstNode> {
        self.consume_whitespace();

        if self.consume_char('(') {
            let expr = self.parse_expr()?;
            if !self.consume_char(')') {
                bail!("括弧の対応が取れていません: pos={}", self.pos);
            }
            Ok(expr)
        } else {
            self.parse_term()
        }
    }

    fn parse_term(&mut self) -> Result<AstNode> {
        self.consume_whitespace();
        let mut pattern = String::new();

        while let Some(c) = self.peek() {
            if c == '&' || c == '|' || c == ')' {
                break;
            }
            if c.is_whitespace() {
                self.advance();
                continue;
            }
            pattern.push(c);
            self.advance();
        }

        if pattern.is_empty() {
            bail!("空のパターンです: pos={}", self.pos);
        }

        // 現在は Literal のみ（将来の拡張に備えてコメント残し）
        Ok(AstNode::Match(Pattern::Literal(pattern)))
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn peek_is(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    fn consume_char(&mut self, expected: char) -> bool {
        if self.peek_is(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::AstNode;

    #[test]
    fn test_single_literal() {
        let ast = parse_query("ERROR").unwrap();
        assert!(matches!(ast, AstNode::Match(Pattern::Literal(ref s)) if s == "ERROR"));
    }

    #[test]
    fn test_and_operator() {
        let ast = parse_query("foo & bar").unwrap();
        match ast {
            AstNode::AndNode(left, right) => {
                assert!(matches!(*left, AstNode::Match(Pattern::Literal(ref s)) if s == "foo"));
                assert!(matches!(*right, AstNode::Match(Pattern::Literal(ref s)) if s == "bar"));
            }
            _ => panic!("Expected AndNode"),
        }
    }

    #[test]
    fn test_or_operator_with_not() {
        let ast = parse_query("!foo | bar").unwrap();
        match ast {
            AstNode::OrNode(left, right) => {
                assert!(matches!(*left, AstNode::NotNode(_)));
                assert!(matches!(*right, AstNode::Match(Pattern::Literal(ref s)) if s == "bar"));
            }
            _ => panic!("Expected OrNode"),
        }
    }

    #[test]
    fn test_parentheses() {
        let ast = parse_query("foo & (bar | baz)").unwrap();
        // Just validate it parses without error
        assert!(matches!(ast, AstNode::AndNode(_, _)));
    }

    #[test]
    fn test_empty_query() {
        let err = parse_query("   ").unwrap_err();
        assert!(err.to_string().contains("クエリが空"));
    }

    #[test]
    fn test_invalid_token() {
        let err = parse_query("foo & & bar").unwrap_err();
        assert!(err.to_string().contains("空のパターン"));
    }


    fn literal(s: &str) -> AstNode {
        AstNode::Match(Pattern::Literal(s.to_string()))
    }

    #[test]
    fn test_operator_precedence() {
        let ast = parse_query("a & b | c").unwrap();

        // AND が OR より優先される
        match ast {
            AstNode::OrNode(lhs, rhs) => {
                match *lhs {
                    AstNode::AndNode(ll, lr) => {
                        assert_eq!(format!("{:?}", *ll), format!("{:?}", literal("a")));
                        assert_eq!(format!("{:?}", *lr), format!("{:?}", literal("b")));
                    }
                    _ => panic!("左側が AndNode ではありません"),
                }
                assert_eq!(format!("{:?}", *rhs), format!("{:?}", literal("c")));
            }
            _ => panic!("ORノードではありません"),
        }
    }

    #[test]
    fn test_not_has_highest_precedence() {
        let ast = parse_query("!a & b").unwrap();

        // NOT が AND より優先される
        match ast {
            AstNode::AndNode(lhs, rhs) => {
                match *lhs {
                    AstNode::NotNode(inner) => {
                        assert_eq!(format!("{:?}", *inner), format!("{:?}", literal("a")));
                    }
                    _ => panic!("左側が NotNode ではありません"),
                }
                assert_eq!(format!("{:?}", *rhs), format!("{:?}", literal("b")));
            }
            _ => panic!("ANDノードではありません"),
        }
    }

    #[test]
    fn test_parentheses_override_precedence() {
        let ast = parse_query("a & (b | c)").unwrap();

        // 括弧により OR が先に評価される
        match ast {
            AstNode::AndNode(lhs, rhs) => {
                assert_eq!(format!("{:?}", *lhs), format!("{:?}", literal("a")));

                match *rhs {
                    AstNode::OrNode(rl, rr) => {
                        assert_eq!(format!("{:?}", *rl), format!("{:?}", literal("b")));
                        assert_eq!(format!("{:?}", *rr), format!("{:?}", literal("c")));
                    }
                    _ => panic!("右側が OrNode ではありません"),
                }
            }
            _ => panic!("ANDノードではありません"),
        }
    }
}
