#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Boolean,
    CloseParen,
    CodeSectionEof,
    Comma,
    Comment,
    DateTime,
    DoubleQuotedString,
    Eof,
    Float,
    FunctionDefinition,
    InfixOperator,
    Integer,
    Newline,
    OpenParen,
    Reference,
    VarAssign,
}