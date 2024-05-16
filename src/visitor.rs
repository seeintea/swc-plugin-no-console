use swc_core::{
    common::util::take::Take,
    ecma::{
        ast::{CallExpr, Callee, Expr, Lit, MemberExpr, MemberProp, Stmt},
        visit::{VisitMut, VisitMutWith},
    },
};

use crate::config::Config;

pub struct TransformVisitor {
    config: Config,
}

impl TransformVisitor {
    pub fn new(config: Config) -> Self {
        TransformVisitor { config }
    }

    pub fn will_retain_by_value(&mut self, expr: &CallExpr) -> bool {
        let mut has_target = false;

        for expr_or_spread in &expr.args {
            if has_target {
                break;
            }
            if let Expr::Lit(lit) = &*expr_or_spread.expr {
                has_target = match lit {
                    Lit::Str(str) => {
                        let value = str.value.to_string();
                        self.config.includes_value.contains(&value)
                    }
                    _ => false,
                }
            }
        }
        has_target
    }

    pub fn will_remove_console(&mut self, expr: &MemberExpr) -> bool {
        let target_sym = match &*expr.obj {
            Expr::Ident(ident) => ident.sym == "console",
            _ => false,
        };

        let mut target_prop = true;
        if self.config.excludes.len() > 0 {
            target_prop = match &expr.prop {
                MemberProp::Ident(ident) => !self.config.excludes.contains(&ident.sym.to_string()),
                _ => false,
            };
        }

        return target_sym && target_prop;
    }

    pub fn will_remove(&mut self, stmt: &mut Stmt) -> bool {
        if let Stmt::Expr(expr) = stmt {
            if let Expr::Call(call_expr) = &mut *expr.expr {
                let callee = &call_expr.callee;
                let member_expr = match callee {
                    Callee::Expr(expr) => match &**expr {
                        Expr::Member(member_expr) => member_expr,
                        _ => return false,
                    },
                    _ => return false,
                };

                let mut has_retain_value = false;

                if self.config.includes_value.len() > 0 {
                    has_retain_value = self.will_retain_by_value(&call_expr);
                }

                return self.will_remove_console(&member_expr) && !has_retain_value;
            }
        }
        false
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        if self.will_remove(stmt) {
            stmt.take();
        }
        stmt.visit_mut_children_with(self);
    }
}
