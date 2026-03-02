#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_hir;

use rustc_lint::LintContext;

dylint_linting::impl_late_lint! {
    pub APIGEN_SPEC,
    Deny,
    "missing required struct `PolymarketCommand`",
    ApigenSpec
}

pub struct ApigenSpec;

impl<'tcx> rustc_lint::LateLintPass<'tcx> for ApigenSpec {
    fn check_crate(&mut self, cx: &rustc_lint::LateContext<'tcx>) {
        let is_present = cx
            .tcx
            .hir_crate_items(())
            .definitions()
            .any(|def_id| matches!(cx.tcx.def_kind(def_id), rustc_hir::def::DefKind::Struct) && cx.tcx.item_name(def_id.to_def_id()).as_str() == "PolymarketCommand");

        if !is_present {
            cx.lint(APIGEN_SPEC, |lint| {
                lint.primary_message("crate must define `struct PolymarketCommand`");
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
    }
}
