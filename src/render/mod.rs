mod python;
mod javascript;
mod typescript;
mod cpp;
mod rust;

use crate::ast::*;

pub use python::PythonRenderer;
pub use javascript::JavaScriptRenderer;
pub use typescript::TypeScriptRenderer;
pub use cpp::CppRenderer;
pub use rust::RustRenderer;

pub trait Renderer {
    fn render_program(&mut self, program: &[Statement]) -> String;
}

// Shared helper for indentation-based renderers
pub struct IndentWriter {
    pub indent: usize,
    pub indent_str: &'static str,
}

impl IndentWriter {
    pub fn new(indent_str: &'static str) -> Self {
        IndentWriter { indent: 0, indent_str }
    }

    pub fn write_line(&self, output: &mut String, line: &str) {
        for _ in 0..self.indent {
            output.push_str(self.indent_str);
        }
        output.push_str(line);
        output.push('\n');
    }

    pub fn write_empty_line(&self, output: &mut String) {
        output.push('\n');
    }

    pub fn inc(&mut self) {
        self.indent += 1;
    }

    pub fn dec(&mut self) {
        self.indent -= 1;
    }
}

// Shared expression rendering for C-like languages (JS, TS, C++, Rust)
pub fn render_arith_op_c(op: ArithmeticOperator) -> &'static str {
    match op {
        ArithmeticOperator::Add => "+",
        ArithmeticOperator::Subtract => "-",
        ArithmeticOperator::Multiply => "*",
        ArithmeticOperator::Divide => "/",
        ArithmeticOperator::Modulo => "%",
        ArithmeticOperator::Power => "**", // JS/TS; C++ and Rust will override
        ArithmeticOperator::BitwiseAnd => "&",
        ArithmeticOperator::BitwiseOr => "|",
        ArithmeticOperator::BitwiseXor => "^",
        ArithmeticOperator::ShiftLeft => "<<",
        ArithmeticOperator::ShiftRight => ">>",
    }
}

pub fn render_bool_op_c(op: BooleanOperator) -> &'static str {
    match op {
        BooleanOperator::And => "&&",
        BooleanOperator::Or => "||",
    }
}

pub fn render_cond_op(op: Condition) -> &'static str {
    match op {
        Condition::Equals => "==",
        Condition::NotEquals => "!=",
        Condition::GreaterThan => ">",
        Condition::LessThan => "<",
        Condition::GreaterThanOrEqual => ">=",
        Condition::LessThanOrEqual => "<=",
    }
}

pub fn render_source_file(program: &[Statement], extension: &str) -> Result<String, String> {
    match extension {
        "py" => {
            let mut r = PythonRenderer::new();
            Ok(r.render_program(program))
        }
        "js" => {
            let mut r = JavaScriptRenderer::new();
            Ok(r.render_program(program))
        }
        "ts" => {
            let mut r = TypeScriptRenderer::new();
            Ok(r.render_program(program))
        }
        "cpp" | "cc" | "cxx" => {
            let mut r = CppRenderer::new();
            Ok(r.render_program(program))
        }
        "rs" => {
            let mut r = RustRenderer::new();
            Ok(r.render_program(program))
        }
        other => Err(format!("unsupported extension: {}", other)),
    }
}
