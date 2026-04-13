use crate::ast::*;
use super::{IndentWriter, Renderer, render_cond_op};

pub struct PythonRenderer {
    w: IndentWriter,
}

impl PythonRenderer {
    pub fn new() -> Self {
        PythonRenderer {
            w: IndentWriter::new("    "),
        }
    }

    fn render_expr(&self, expr: &Expression) -> String {
        match expr {
            Expression::Arithmetic(l, op, r) => {
                format!("({} {} {})", self.render_expr(l), Self::arith_op(*op), self.render_expr(r))
            }
            Expression::Unary(op, e) => match op {
                UnaryOperator::Negate => format!("(-{})", self.render_expr(e)),
                UnaryOperator::BitwiseNot => format!("(~{})", self.render_expr(e)),
            },
            Expression::Boolean(l, op, r) => {
                let op_str = match op {
                    BooleanOperator::And => "and",
                    BooleanOperator::Or => "or",
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
            Expression::BoolLiteral(v) => if *v { "True".into() } else { "False".into() },
        }
    }

    fn arith_op(op: ArithmeticOperator) -> &'static str {
        match op {
            ArithmeticOperator::Add => "+",
            ArithmeticOperator::Subtract => "-",
            ArithmeticOperator::Multiply => "*",
            ArithmeticOperator::Divide => "/",
            ArithmeticOperator::Modulo => "%",
            ArithmeticOperator::Power => "**",
            ArithmeticOperator::BitwiseAnd => "&",
            ArithmeticOperator::BitwiseOr => "|",
            ArithmeticOperator::BitwiseXor => "^",
            ArithmeticOperator::ShiftLeft => "<<",
            ArithmeticOperator::ShiftRight => ">>",
        }
    }

    fn render_block(&mut self, block: &[Statement], out: &mut String) {
        if block.is_empty() {
            self.w.write_line(out, "pass");
        } else {
            for stmt in block {
                self.render_stmt(stmt, out);
            }
        }
    }

    fn render_stmt(&mut self, stmt: &Statement, out: &mut String) {
        match stmt {
            Statement::Assignment(name, expr) => {
                self.w.write_line(out, &format!("{} = {}", name, self.render_expr(expr)));
            }
            Statement::If(cond, then_b, else_b) => {
                self.w.write_line(out, &format!("if {}:", self.render_expr(cond)));
                self.w.inc();
                self.render_block(then_b, out);
                self.w.dec();
                if !else_b.is_empty() {
                    self.w.write_line(out, "else:");
                    self.w.inc();
                    self.render_block(else_b, out);
                    self.w.dec();
                }
            }
            Statement::While(cond, body) => {
                self.w.write_line(out, &format!("while {}:", self.render_expr(cond)));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
            }
            Statement::For(var, count, body) => {
                self.w.write_line(out, &format!("for {} in range({}):", var, self.render_expr(count)));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
            }
            Statement::FunctionDef(name, _params, body) => {
                self.w.write_line(out, &format!("def {}():", name));
                self.w.inc();
                self.render_block(body, out);
                self.w.dec();
                self.w.write_empty_line(out);
            }
            Statement::Return(expr) => {
                self.w.write_line(out, &format!("return {}", self.render_expr(expr)));
            }
            Statement::Expression(expr) => {
                self.w.write_line(out, &self.render_expr(expr));
            }
            Statement::Comment(text) => {
                self.w.write_line(out, &format!("# {}", text));
            }
        }
    }
}

impl Renderer for PythonRenderer {
    fn render_program(&mut self, program: &[Statement]) -> String {
        let mut out = String::new();
        for stmt in program {
            self.render_stmt(stmt, &mut out);
            out.push('\n');
        }
        out
    }
}
