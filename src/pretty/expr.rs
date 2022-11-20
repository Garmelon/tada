use pretty::{DocAllocator, DocBuilder, Pretty};

use crate::ast::{Assoc, BinOp, Expr, Field, Var};

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for BinOp {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        allocator.text(match self {
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Eq => "==",
            Self::Neq => "!=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::And => "and",
            Self::Or => "or",
        })
    }
}

impl<'a, D: DocAllocator<'a>> Pretty<'a, D> for Expr {
    fn pretty(self, allocator: &'a D) -> DocBuilder<'a, D> {
        match self {
            Self::Lit(lit) => lit.pretty(allocator),
            Self::Call(call) => call.pretty(allocator),
            Self::Field(field) => field.pretty(allocator),
            Self::Var(var) => var.pretty(allocator),
            Self::TableConstr(constr) => constr.pretty(allocator),
            Self::TableDestr(destr) => allocator.text("<destr>"),
            Self::FuncDef(def) => allocator.text("<def>"),
            Self::Paren {
                s0,
                inner,
                s1,
                span: _,
            } => inner.pretty(allocator).parens(),

            Self::Neg {
                minus: _,
                s0,
                expr,
                span: _,
            } => {
                let parenthesize = matches!(*expr, Self::BinOp { .. });
                let inner = expr.pretty(allocator);
                allocator
                    .text("-")
                    .append(if parenthesize { inner.parens() } else { inner })
            }

            Self::Not {
                not: _,
                s0,
                expr,
                span: _,
            } => {
                let parenthesize = matches!(*expr, Self::BinOp { .. });
                let inner = expr.pretty(allocator);
                allocator
                    .text("not ")
                    .append(if parenthesize { inner.parens() } else { inner })
            }

            // TODO Add newlines and group properly
            Self::BinOp {
                left,
                s0,
                op,
                s1,
                right,
                span: _,
            } => {
                // If we're left-associative, then the left subexpression can be
                // at the same precedence and the right subexpression must be at
                // a higher precedence.

                // If we're right-associative, then the left subexpression must
                // be at a higher precedence and the right subexpression can be
                // at the same precedence.

                // Minimum precedence that the left subexpression can be at
                // without requiring parentheses.
                let min_left_prec = match op.assoc() {
                    Assoc::Left => op.precedence(),
                    Assoc::Right => op.precedence() + 1,
                };

                // Minimum precedence that the right subexpression can be at
                // without requiring parentheses.
                let min_right_prec = match op.assoc() {
                    Assoc::Left => op.precedence() + 1,
                    Assoc::Right => op.precedence(),
                };

                let left_paren = match *left {
                    // These end with an arbitrary expression on the right. If
                    // we don't add parentheses, we'll be assimilated into that
                    // expression.
                    Self::Field(Field::Assign { .. } | Field::AssignIdent { .. }) => true,
                    Self::Var(Var::Assign { .. } | Var::AssignIdent { .. }) => true,

                    Self::BinOp { op, .. } if op.precedence() < min_left_prec => true,

                    _ => false,
                };

                let right_paren =
                    matches!(*right, Self::BinOp { op, .. } if op.precedence() < min_right_prec);

                let left = left.pretty(allocator);
                let left = if left_paren { left.parens() } else { left };

                let right = right.pretty(allocator);
                let right = if right_paren { right.parens() } else { right };

                left.append(allocator.space())
                    .append(op.pretty(allocator))
                    .append(allocator.space())
                    .append(right)
            }
        }
    }
}
