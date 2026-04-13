use crate::ast::*;
use rand::RngExt;
use super::{IndentWriter, Renderer, render_cond_op};

pub struct RustRenderer {
    w: IndentWriter,
}

impl RustRenderer {
    pub fn new() -> Self {
        RustRenderer {
            w: IndentWriter::new("    "),
        }
    }

    fn rand_type() -> &'static str {
        let types = ["i32", "i64", "u32", "u64", "f64", "f32", "isize", "usize"];
        types[rand::rng().random_range(0..types.len())]
    }

    fn arith_op(op: ArithmeticOperator) -> &'static str {
        match op {
            ArithmeticOperator::Add => "+",
            ArithmeticOperator::Subtract => "-",
            ArithmeticOperator::Multiply => "*",
            ArithmeticOperator::Divide => "/",
            ArithmeticOperator::Modulo => "%",
            ArithmeticOperator::Power => "+", // Rust has no ** operator; just degrade to +
            ArithmeticOperator::BitwiseAnd => "&",
            ArithmeticOperator::BitwiseOr => "|",
            ArithmeticOperator::BitwiseXor => "^",
            ArithmeticOperator::ShiftLeft => "<<",
            ArithmeticOperator::ShiftRight => ">>",
        }
    }

    fn render_expr(&self, expr: &Expression) -> String {
        match expr {
            Expression::Arithmetic(l, op, r) => {
                format!("({} {} {})", self.render_expr(l), Self::arith_op(*op), self.render_expr(r))
            }
            Expression::Unary(op, e) => match op {
                UnaryOperator::Negate => format!("(-{})", self.render_expr(e)),
                UnaryOperator::BitwiseNot => format!("(!{})", self.render_expr(e)),
            },
            Expression::Boolean(l, op, r) => {
                let op_str = match op {
                    BooleanOperator::And => "&&",
                    BooleanOperator::Or => "||",
                };
                format!("({} {} {})", self.render_expr(l), op_str, self.render_expr(r))
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
                    &format!("let mut {}: {} = {};", name, Self::rand_type(), self.render_expr(expr)),
                );
            }
            Statement::If(cond, then_b, else_b) => {
                self.w.write_line(out, &format!("if {} {{", self.render_expr(cond)));
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
                self.w.write_line(out, &format!("while {} {{", self.render_expr(cond)));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_line(out, "}");
            }
            Statement::For(var, count, body) => {
                self.w.write_line(
                    out,
                    &format!("for {} in 0..{} {{", var, self.render_expr(count)),
                );
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_line(out, "}");
            }
            Statement::FunctionDef(name, _params, body) => {
                self.w.write_line(out, &format!("fn {}() {{", name));
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

impl Renderer for RustRenderer {
    fn render_program(&mut self, program: &[Statement]) -> String {
        let mut out = String::from("#![allow(unused, dead_code, unreachable_code)]\n\n");
        for stmt in program {
            self.render_stmt(stmt, &mut out);
            out.push('\n');
        }
        out
    }
}
