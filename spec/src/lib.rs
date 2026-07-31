#![feature(rustc_private)]
#![warn(unused_extern_crates)]
#![deny(clippy::arithmetic_side_effects)]

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use clippy_utils::diagnostics::span_lint;
use rustc_hir::def::DefKind;
use rustc_lint::{LateContext, LateLintPass, LintStore};
use rustc_session::Session;
use rustc_span::DUMMY_SP;

dylint_linting::dylint_library!();

rustc_session::declare_lint! {
    /// Ensures that the checked crate contains a struct named `PolymarketCommand`.
    pub MISSING_POLYMARKET_COMMAND_STRUCT,
    Deny,
    "missing required struct `PolymarketCommand`"
}

pub struct MissingPolymarketCommandStruct;

rustc_session::impl_lint_pass!(MissingPolymarketCommandStruct => [MISSING_POLYMARKET_COMMAND_STRUCT]);

#[unsafe(no_mangle)]
pub fn register_lints(session: &Session, lint_store: &mut LintStore) {
    dylint_linting::init_config(session);
    lint_store.register_lints(&[MISSING_POLYMARKET_COMMAND_STRUCT]);
    // Dylint 6.0.2 still calls the removed `register_late_pass` API, so register explicitly until Dylint supports nightly-2026-07-09.
    lint_store.register_late_lint_pass(Box::new(|_| Box::new(MissingPolymarketCommandStruct)));
}

impl<'tcx> LateLintPass<'tcx> for MissingPolymarketCommandStruct {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        if cx.tcx.entry_fn(()).is_some() {
            return;
        }

        let is_present = cx
            .tcx
            .hir_crate_items(())
            .definitions()
            .any(|def_id| matches!(cx.tcx.def_kind(def_id), DefKind::Struct) && cx.tcx.item_name(def_id.to_def_id()).as_str() == "PolymarketCommand");

        if is_present {
            return;
        }

        span_lint(cx, MISSING_POLYMARKET_COMMAND_STRUCT, DUMMY_SP, "crate must define `struct PolymarketCommand`");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
    }
}
