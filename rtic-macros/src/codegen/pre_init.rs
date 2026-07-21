use super::bindings::{pre_init_checks, pre_init_enable_interrupts};
use crate::analyze::Analysis;
use crate::syntax::ast::App;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code that runs before `#[init]`
pub fn codegen(app: &App, analysis: &Analysis) -> Vec<TokenStream2> {
    let mut stmts: Vec<TokenStream2> = vec![];

    // About as early as it gets, but still after #[pre_init]
    if let Some(early_init) = &app.early_init {
        let attrs = &early_init.attrs;
        let name = &early_init.name;
        let early_init_stmts = &early_init.stmts;
        if !early_init.is_extern {
            stmts.push(quote!(
                #(#attrs)*
                fn #name() {
                    #(#early_init_stmts)*
                }
            ))
        }
        // Call early_init
        stmts.push(quote!(let _: () = #name();));
    };

    // Disable interrupts -- `init` must run with interrupts disabled
    stmts.push(quote!(rtic::export::interrupt::disable();));

    if app.args.core {
        stmts.push(quote!(
            // To set the variable in cortex_m so the peripherals cannot be taken multiple times
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
        ));
    }

    stmts.append(&mut pre_init_checks(app, analysis));

    stmts.append(&mut pre_init_enable_interrupts(app, analysis));

    stmts
}
