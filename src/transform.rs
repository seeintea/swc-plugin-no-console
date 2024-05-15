use serde::Deserialize;
use swc_core::{
    common::DUMMY_SP,
    ecma::{
        ast::{Callee, EmptyStmt, Expr, Ident, Stmt},
        visit::{Fold, FoldWith},
    },
};

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Config {
    Enable(bool),
}

pub struct NoConsoleVisitor {
    enable: bool,
}

impl NoConsoleVisitor {
    fn is_console(&self, ident: &Ident) -> bool {
        &ident.sym == "console"
    }

    fn will_remove_stmt(&mut self, stmt: &Stmt) -> bool {
        if let Stmt::Expr(expr) = stmt {
            if let Expr::Call(call) = &*expr.expr {
                if let Callee::Expr(expr) = &call.callee {
                    if let Expr::Member(member) = &**expr {
                        if let Expr::Ident(ident) = &*member.obj {
                            if self.is_console(ident) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

impl Fold for NoConsoleVisitor {
    fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
        if self.enable {
            if self.will_remove_stmt(&stmt) {
                return Stmt::Empty(EmptyStmt { span: DUMMY_SP });
            }
        }
        stmt.fold_children_with(self)
    }
}

pub fn no_console_visitor(config: Config) -> impl Fold {
    let enable = match config {
        Config::Enable(enable) => enable,
    };
    NoConsoleVisitor { enable }
}
