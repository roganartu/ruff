use itertools::Itertools;
use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::is_const_none;
use ruff_python_ast::{self as ast, CmpOp, Expr};
use ruff_python_semantic::SemanticModel;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;

/// ## What it does
/// Checks for object type comparisons using `==` and other comparison
/// operators.
///
/// ## Why is this bad?
/// When comparing types, prefer `is` and `is not`, or `isinstance()` for
/// object inheritance checks.
///
/// Unlike a direct type comparison, `isinstance` will also check if an object
/// is an instance of a class or a subclass thereof.
///
/// ## Example
/// ```python
/// if type(obj) == type(1):
///     pass
///
/// if type(obj) == int:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if isinstance(obj, int):
///     pass
/// ```
#[violation]
pub struct TypeComparison;

impl Violation for TypeComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Use `is` and `is not` for type comparisons, or `isinstance()` for isinstance checks"
        )
    }
}

/// E721
pub(crate) fn type_comparison(checker: &mut Checker, compare: &ast::ExprCompare) {
    for (left, right) in std::iter::once(compare.left.as_ref())
        .chain(compare.comparators.iter())
        .tuple_windows()
        .zip(compare.ops.iter())
        .filter(|(_, op)| matches!(op, CmpOp::Eq | CmpOp::NotEq))
        .map(|((left, right), _)| (left, right))
    {
        if is_type(left, checker.semantic()) || is_type(right, checker.semantic()) {
            checker
                .diagnostics
                .push(Diagnostic::new(TypeComparison, compare.range()));
        }
    }
}

/// Returns `true` if the [`Expr`] is known to evaluate to a type (e.g., `int`, or `type(1)`).
fn is_type(expr: &Expr, semantic: &SemanticModel) -> bool {
    match expr {
        Expr::Call(ast::ExprCall {
            func, arguments, ..
        }) => {
            // Ex) `type(obj) == type(1)`
            let Expr::Name(ast::ExprName { id, .. }) = func.as_ref() else {
                return false;
            };

            if !(id == "type" && semantic.is_builtin("type")) {
                return false;
            };

            // Allow comparison for types which are not obvious.
            arguments
                .args
                .first()
                .is_some_and(|arg| !arg.is_name_expr() && !is_const_none(arg))
        }
        Expr::Name(ast::ExprName { id, .. }) => {
            // Ex) `type(obj) == int`
            matches!(
                id.as_str(),
                "int" | "str" | "float" | "bool" | "complex" | "bytes" | "list" | "dict" | "set"
            ) && semantic.is_builtin(id)
        }
        _ => false,
    }
}
