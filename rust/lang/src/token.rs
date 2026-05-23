#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
    // VARIABLE
    Var(String),
    None,

    // TYPES
    Char,
    Str(String),
    Bool(bool),
    Number(i64),
    Float(f64),

    // SMY
    PLUS,
    MINUS,
    STAR,
    SLASH,
    SQ,

    LET,
    ASSIGN,

    // CLAUSES
    EQ,
    UNEQ,
    GT,
    GE,
    LT,
    LE,

    IF,
    ELSEIF,
    ELSE,

    THEN,

    WHILE,
    DO,

    FOR,
    IN,

    END,

    // BRACKETS
    SPARENT,
    EPARENT,

    CurlSbracket,
    CurlEbreacket,

    SquareSbracket,
    SquareEbracket,


    // FUNC
    Func,
}
