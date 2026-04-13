use crate::ast::*;
use rand::prelude::*;
use std::collections::HashSet;

const SLOPPY_COMMENTS: &[&str] = &[
    "TODO: fix this later",
    "HACK: this works somehow",
    "FIXME: no idea why this is here",
    "NOTE: do not touch this",
    "TEMP: temporary workaround",
    "XXX: refactor needed",
    "this is fine",
    "idk what this does",
    "legacy code, do not remove",
    "trust me bro",
    "works on my machine",
    "please dont delete",
    "might cause segfault lol",
    "copilot wrote this",
    "stolen from stackoverflow",
    "IMPORTANT: not actually important",
    "why does removing this break everything",
    "magic numbers ahead",
    "i wrote this at 3am",
    "REVIEW: never gonna happen",
    "TODO: add error handling",
    "TODO: add tests",
    "this variable does nothing",
    "no one knows what this function does",
    "do NOT refactor",
    "here be dragons",
];

const SLOPPY_STRINGS: &[&str] = &[
    "hello world",
    "foo",
    "bar",
    "baz",
    "qux",
    "asdf",
    "test",
    "temp",
    "data",
    "result",
    "output",
    "input",
    "value",
    "thing",
    "stuff",
    "undefined",
    "null",
    "error",
    "success",
    "placeholder",
    "TODO",
    "FIXME",
    "hack",
    "yolo",
    "bruh",
    "aaaaa",
    "12345",
    "password123",
    "admin",
    "root",
    "debug",
    "prod",
    "staging",
    "localhost",
    "0.0.0.0",
    "NaN",
    "infinity",
];

pub struct RandSet<T> {
    pub set: HashSet<T>,
    pub vec: Vec<T>,
}

impl<T: Eq + std::hash::Hash + Clone> RandSet<T> {
    pub fn new() -> RandSet<T> {
        RandSet {
            set: HashSet::new(),
            vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, item: T) {
        if self.set.insert(item.clone()) {
            self.vec.push(item);
        }
    }

    pub fn random(&self) -> Option<&T> {
        self.vec.choose(&mut rand::rng())
    }
}

pub struct GeneratorState {
    pub max_depth: usize,
    pub scopes: Vec<RandSet<String>>,
    pub functions: RandSet<String>,
    pub names: Vec<&'static str>,
    pub name_counter: usize,
    pub keywords: HashSet<&'static str>,
}

impl GeneratorState {
    pub fn new(max_depth: usize, names: Vec<&'static str>) -> Self {
        let mut keywords = HashSet::new();
        // Union of keywords across all target languages
        for kw in &[
            // Python
            "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class",
            "continue", "def", "del", "elif", "else", "except", "finally", "for", "from",
            "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass",
            "raise", "return", "try", "while", "with", "yield",
            // JS/TS
            "var", "let", "const", "function", "new", "delete", "typeof", "instanceof", "void",
            "this", "super", "switch", "case", "default", "throw", "catch", "debugger", "export",
            "extends", "implements", "interface", "package", "private", "protected", "public",
            "static", "enum", "type", "namespace", "abstract", "declare", "readonly",
            // C++
            "auto", "bool", "char", "double", "float", "int", "long", "short", "signed",
            "unsigned", "void", "struct", "union", "template", "typename", "virtual", "override",
            "final", "nullptr", "sizeof", "alignof", "constexpr", "noexcept", "mutable",
            "volatile", "register", "extern", "inline", "goto", "do", "using", "namespace",
            // Rust
            "fn", "let", "mut", "ref", "self", "Self", "mod", "pub", "crate", "use", "impl",
            "trait", "where", "loop", "match", "move", "box", "unsafe", "dyn",
        ] {
            keywords.insert(*kw);
        }

        GeneratorState {
            max_depth,
            scopes: Vec::new(),
            functions: RandSet::new(),
            names,
            name_counter: 0,
            keywords,
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(RandSet::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn all_names(&self) -> HashSet<String> {
        let mut all = HashSet::new();
        for scope in &self.scopes {
            for name in &scope.vec {
                all.insert(name.clone());
            }
        }
        for name in &self.functions.vec {
            all.insert(name.clone());
        }
        all
    }

    fn all_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        for scope in &self.scopes {
            vars.extend(scope.vec.iter().cloned());
        }
        vars
    }

    fn is_valid_ident(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        let mut chars = name.chars();
        let first = chars.next().unwrap();
        if !(first == '_' || first.is_ascii_alphabetic()) {
            return false;
        }
        if !chars.all(|c| c == '_' || c.is_ascii_alphanumeric()) {
            return false;
        }
        !self.keywords.contains(name)
    }

    pub fn generate_name(&mut self) -> String {
        let taken = self.all_names();
        let mut rng = rand::rng();
        for _ in 0..200 {
            if let Some(candidate) = self.names.choose(&mut rng) {
                let s = candidate.to_string();
                if !taken.contains(&s) && self.is_valid_ident(&s) {
                    return s;
                }
            }
        }
        // Fallback: generate a unique name
        loop {
            self.name_counter += 1;
            let s = format!("var_{}", self.name_counter);
            if !taken.contains(&s) {
                return s;
            }
        }
    }

    fn rand_var(&self) -> Option<String> {
        let vars = self.all_vars();
        if vars.is_empty() {
            return None;
        }
        vars.choose(&mut rand::rng()).cloned()
    }

    fn rand_function(&self) -> Option<String> {
        self.functions.random().cloned()
    }

    fn rand_comment(&self) -> String {
        SLOPPY_COMMENTS
            .choose(&mut rand::rng())
            .unwrap()
            .to_string()
    }

    fn rand_string(&self) -> String {
        SLOPPY_STRINGS
            .choose(&mut rand::rng())
            .unwrap()
            .to_string()
    }

    fn gen_arith_op(&self) -> ArithmeticOperator {
        let ops = [
            ArithmeticOperator::Add,
            ArithmeticOperator::Subtract,
            ArithmeticOperator::Multiply,
            ArithmeticOperator::Divide,
            ArithmeticOperator::Modulo,
            ArithmeticOperator::Power,
            ArithmeticOperator::BitwiseAnd,
            ArithmeticOperator::BitwiseOr,
            ArithmeticOperator::BitwiseXor,
            ArithmeticOperator::ShiftLeft,
            ArithmeticOperator::ShiftRight,
        ];
        *ops.choose(&mut rand::rng()).unwrap()
    }

    fn gen_bool_op(&self) -> BooleanOperator {
        let ops = [BooleanOperator::And, BooleanOperator::Or];
        *ops.choose(&mut rand::rng()).unwrap()
    }

    fn gen_cond_op(&self) -> Condition {
        let ops = [
            Condition::Equals,
            Condition::NotEquals,
            Condition::GreaterThan,
            Condition::LessThan,
            Condition::GreaterThanOrEqual,
            Condition::LessThanOrEqual,
        ];
        *ops.choose(&mut rand::rng()).unwrap()
    }

    pub fn gen_numeric_expr(&mut self, depth: usize) -> Expression {
        let mut rng = rand::rng();
        if depth >= self.max_depth || rng.random_bool(0.25) {
            return match rng.random_range(0..5) {
                0 => self
                    .rand_var()
                    .map(Expression::Variable)
                    .unwrap_or(Expression::IntLiteral(rng.random_range(0..1000))),
                1 => self
                    .rand_function()
                    .map(Expression::FunctionCall)
                    .unwrap_or(Expression::IntLiteral(rng.random_range(0..1000))),
                2 => Expression::IntLiteral(rng.random_range(-999..1000)),
                3 => Expression::FloatLiteral(
                    (rng.random_range(-999..1000) as f64) / (rng.random_range(1..100) as f64),
                ),
                4 => Expression::StringLiteral(self.rand_string()),
                _ => unreachable!(),
            };
        }

        match rng.random_range(0..3) {
            0 => Expression::Arithmetic(
                Box::new(self.gen_numeric_expr(depth + 1)),
                self.gen_arith_op(),
                Box::new(self.gen_numeric_expr(depth + 1)),
            ),
            1 => Expression::Unary(
                if rng.random_bool(0.5) {
                    UnaryOperator::Negate
                } else {
                    UnaryOperator::BitwiseNot
                },
                Box::new(self.gen_numeric_expr(depth + 1)),
            ),
            2 => Expression::Arithmetic(
                Box::new(self.gen_numeric_expr(depth + 1)),
                self.gen_arith_op(),
                Box::new(self.gen_numeric_expr(depth + 1)),
            ),
            _ => unreachable!(),
        }
    }

    pub fn gen_cond_expr(&mut self, depth: usize) -> Expression {
        Expression::Condition(
            Box::new(self.gen_numeric_expr(depth + 1)),
            self.gen_cond_op(),
            Box::new(self.gen_numeric_expr(depth + 1)),
        )
    }

    pub fn gen_bool_expr(&mut self, depth: usize) -> Expression {
        let mut rng = rand::rng();
        if depth >= self.max_depth || rng.random_bool(0.4) {
            return match rng.random_range(0..3) {
                0 => self.gen_cond_expr(depth + 1),
                1 => Expression::BoolLiteral(rng.random_bool(0.5)),
                2 => self
                    .rand_function()
                    .map(Expression::FunctionCall)
                    .unwrap_or(Expression::BoolLiteral(true)),
                _ => unreachable!(),
            };
        }
        Expression::Boolean(
            Box::new(self.gen_bool_expr(depth + 1)),
            self.gen_bool_op(),
            Box::new(self.gen_bool_expr(depth + 1)),
        )
    }

    pub fn gen_expr(&mut self, depth: usize) -> Expression {
        if rand::rng().random_bool(0.65) {
            self.gen_numeric_expr(depth)
        } else {
            self.gen_bool_expr(depth)
        }
    }

    pub fn gen_statement(&mut self, depth: usize, in_function: bool) -> Statement {
        let mut rng = rand::rng();

        if depth >= self.max_depth {
            return if let Some(var) = self.rand_var() {
                Statement::Assignment(var, self.gen_expr(depth + 1))
            } else {
                Statement::Expression(self.gen_expr(depth + 1))
            };
        }

        // Inject sloppy comments ~20% of the time
        if rng.random_bool(0.18) {
            return Statement::Comment(self.rand_comment());
        }

        let choice = rng.random_range(0..8);
        match choice {
            0 | 1 => {
                let name = self.generate_name();
                self.scopes.last_mut().unwrap().insert(name.clone());
                Statement::Assignment(name, self.gen_expr(depth + 1))
            }
            2 => {
                if let Some(var) = self.rand_var() {
                    Statement::Assignment(var, self.gen_expr(depth + 1))
                } else {
                    let name = self.generate_name();
                    self.scopes.last_mut().unwrap().insert(name.clone());
                    Statement::Assignment(name, self.gen_expr(depth + 1))
                }
            }
            3 => Statement::If(
                self.gen_bool_expr(depth + 1),
                self.gen_scoped_block(depth + 1, in_function),
                if rng.random_bool(0.6) {
                    self.gen_scoped_block(depth + 1, in_function)
                } else {
                    vec![]
                },
            ),
            4 => Statement::While(
                self.gen_bool_expr(depth + 1),
                self.gen_scoped_block(depth + 1, in_function),
            ),
            5 => {
                let loop_var = self.generate_name();
                self.scopes.last_mut().unwrap().insert(loop_var.clone());
                Statement::For(
                    loop_var,
                    self.gen_numeric_expr(depth + 1),
                    self.gen_scoped_block(depth + 1, in_function),
                )
            }
            6 => {
                // Function def
                let name = self.generate_name();
                self.functions.insert(name.clone());
                Statement::FunctionDef(name, vec![], self.gen_function_body(depth + 1))
            }
            7 if in_function => Statement::Return(self.gen_expr(depth + 1)),
            _ => {
                // expression
                Statement::Expression(self.gen_expr(depth + 1))
            }
        }
    }

    fn gen_scoped_block(&mut self, depth: usize, in_function: bool) -> Vec<Statement> {
        self.push_scope();
        let block = self.gen_block(depth, in_function);
        self.pop_scope();
        block
    }

    fn gen_block(&mut self, depth: usize, in_function: bool) -> Vec<Statement> {
        let count = rand::rng().random_range(1..=5);
        let mut block = Vec::with_capacity(count);
        for _ in 0..count {
            block.push(self.gen_statement(depth, in_function));
        }
        block
    }

    fn gen_function_body(&mut self, depth: usize) -> Vec<Statement> {
        self.push_scope();
        let mut body = self.gen_block(depth, true);
        if !matches!(body.last(), Some(Statement::Return(_))) {
            body.push(Statement::Return(self.gen_expr(depth + 1)));
        }
        self.pop_scope();
        body
    }

    pub fn generate_program(&mut self) -> Vec<Statement> {
        self.push_scope();
        let mut program = Vec::new();
        let mut rng = rand::rng();

        let fn_count = rng.random_range(2..=4);
        for _ in 0..fn_count {
            let name = self.generate_name();
            self.functions.insert(name.clone());
            program.push(Statement::FunctionDef(
                name,
                vec![],
                self.gen_function_body(0),
            ));
        }

        let stmt_count = rng.random_range(8..=25);
        for _ in 0..stmt_count {
            program.push(self.gen_statement(0, false));
        }

        self.pop_scope();
        program
    }
}
