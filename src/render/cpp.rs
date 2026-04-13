use crate::ast::*;
use rand::RngExt;
use super::{IndentWriter, Renderer, render_bool_op_c, render_cond_op};

pub struct CppRenderer {
    w: IndentWriter,
}

impl CppRenderer {
    pub fn new() -> Self {
        CppRenderer {
            w: IndentWriter::new("    "),
        }
    }

    fn rand_type() -> &'static str {
        let types = ["int", "long", "double", "float", "unsigned int", "long long", "short", "auto"];
        types[rand::rng().random_range(0..types.len())]
    }

    fn arith_op(op: ArithmeticOperator) -> String {
        match op {
            ArithmeticOperator::Power => {
                // C++ doesn't have **, handled at expression level
                "+".to_string()
            }
            ArithmeticOperator::Add => "+".into(),
            ArithmeticOperator::Subtract => "-".into(),
            ArithmeticOperator::Multiply => "*".into(),
            ArithmeticOperator::Divide => "/".into(),
            ArithmeticOperator::Modulo => "%".into(),
            ArithmeticOperator::BitwiseAnd => "&".into(),
            ArithmeticOperator::BitwiseOr => "|".into(),
            ArithmeticOperator::BitwiseXor => "^".into(),
            ArithmeticOperator::ShiftLeft => "<<".into(),
            ArithmeticOperator::ShiftRight => ">>".into(),
        }
    }

    fn render_expr(&self, expr: &Expression) -> String {
        match expr {
            Expression::Arithmetic(l, ArithmeticOperator::Power, r) => {
                format!("pow({}, {})", self.render_expr(l), self.render_expr(r))
            }
            Expression::Arithmetic(l, op, r) => {
                format!("({} {} {})", self.render_expr(l), Self::arith_op(*op), self.render_expr(r))
            }
            Expression::Unary(op, e) => match op {
                UnaryOperator::Negate => format!("(-{})", self.render_expr(e)),
                UnaryOperator::BitwiseNot => format!("(~{})", self.render_expr(e)),
            },
            Expression::Boolean(l, op, r) => {
                format!("({} {} {})", self.render_expr(l), render_bool_op_c(*op), self.render_expr(r))
            }
            Expression::Condition(l, op, r) => {
                format!("({} {} {})", self.render_expr(l), render_cond_op(*op), self.render_expr(r))
            }
            Expression::FunctionCall(name) => format!("{}()", name),
            Expression::Variable(name) => name.clone(),
            Expression::StringLiteral(s) => format!("\"{}\"", s),
            Expression::IntLiteral(v) => v.to_string(),
            Expression::FloatLiteral(v) => format!("{:.4}", v),
            Expression::BoolLiteral(v) => if *v { "true".into() } else { "false".into() },
        }
    }

    fn render_block(&mut self, block: &[Statement], out: &mut String) {
        for stmt in block {
            self.render_stmt(stmt, out);
        }
    }

    fn render_stmt(&mut self, stmt: &Statement, out: &mut String) {
        match stmt {
            Statement::Assignment(name, expr) => {
                self.w.write_line(
                    out,
                    &format!("{} {} = {};", Self::rand_type(), name, self.render_expr(expr)),
                );
            }
            Statement::If(cond, then_b, else_b) => {
                self.w.write_line(out, &format!("if ({}) {{", self.render_expr(cond)));
                self.w.inc();
                self.render_block(then_b, out);
                self.w.dec();
                if !else_b.is_empty() {
                    self.w.write_line(out, "} else {");
                    self.w.inc();
                    self.render_block(else_b, out);
                    self.w.dec();
                }
                self.w.write_line(out, "}");
            }
            Statement::While(cond, body) => {
                self.w.write_line(out, &format!("while ({}) {{", self.render_expr(cond)));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_line(out, "}");
            }
            Statement::For(var, count, body) => {
                self.w.write_line(
                    out,
                    &format!(
                        "for (int {} = 0; {} < {}; {}++) {{",
                        var, var, self.render_expr(count), var
                    ),
                );
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_line(out, "}");
            }
            Statement::FunctionDef(name, _params, body) => {
                self.w.write_line(out, &format!("{} {}() {{", Self::rand_type(), name));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_line(out, "}");
                self.w.write_empty_line(out);
            }
            Statement::Return(expr) => {
                self.w.write_line(out, &format!("return {};", self.render_expr(expr)));
            }
            Statement::Expression(expr) => {
                self.w.write_line(out, &format!("{};", self.render_expr(expr)));
            }
            Statement::Comment(text) => {
                self.w.write_line(out, &format!("// {}", text));
            }
        }
    }
}

impl Renderer for CppRenderer {
    fn render_program(&mut self, program: &[Statement]) -> String {
        let mut out = String::from("#include <cmath>\n#include <iostream>\n#include <string>\n\nusing namespace std;\n\n");
        // Forward declarations
        for stmt in program {
            if let Statement::FunctionDef(name, _, _) = stmt {
                out.push_str(&format!("auto {}();\n", name));
            }
        }
        out.push('\n');

        for stmt in program {
            self.render_stmt(stmt, &mut out);
            out.push('\n');
        }
        out
    }
}
