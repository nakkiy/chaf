// Represents a node in the abstract syntax tree (AST) for the logical query DSL.
#[derive(Debug, Clone)]
pub enum AstNode {
    AndNode(Box<AstNode>, Box<AstNode>),
    OrNode(Box<AstNode>, Box<AstNode>),
    NotNode(Box<AstNode>),
    Match(Pattern),
}

// Represents a leaf pattern in the query DSL.
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(String), // eg: "log"
                     //    Wildcard(String),     // eg: "*.log"(Planned additions in the future)
                     //    Regex(regex::Regex),  // Compiled Regular Expressions(Planned additions in the future)
}
