#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Boolean,
    CloseParen,
    CodeSectionEof,
    Comma,
    Comment,
    DoubleQuotedString,
    Eof,
    Float,
    FunctionDefinition,
    Integer,
    Newline,
    OpenParen,
    Operator,
    Reference,
    UseModule,
    VarAssign,
}
