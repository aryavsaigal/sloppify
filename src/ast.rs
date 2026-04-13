#[derive(Clone, Copy, Debug)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Clone, Copy, Debug)]
pub enum BooleanOperator {
    And,
    Or,
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Negate,
    BitwiseNot,
}

#[derive(Clone, Copy, Debug)]
pub enum Condition {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

pub enum Expression {
    Arithmetic(Box<Expression>, ArithmeticOperator, Box<Expression>),
    Unary(UnaryOperator, Box<Expression>),
    Boolean(Box<Expression>, BooleanOperator, Box<Expression>),
    Condition(Box<Expression>, Condition, Box<Expression>),
    FunctionCall(String),
    Variable(String),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
}

pub enum Statement {
    Assignment(String, Expression),
    If(Expression, Vec<Statement>, Vec<Statement>),
    While(Expression, Vec<Statement>),
    For(String, Expression, Vec<Statement>),
    FunctionDef(String, Vec<String>, Vec<Statement>),
    Return(Expression),
    Expression(Expression),
    Comment(String),
}
