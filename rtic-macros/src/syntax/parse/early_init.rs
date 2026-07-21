use syn::{parse, ForeignItemFn, ItemFn, Stmt};

use crate::syntax::{ast::EarlyInit, parse::util};

impl EarlyInit {
    pub(crate) fn parse(item: ItemFn) -> parse::Result<Self> {
        let valid_signature = util::check_fn_signature(&item, false)
            && item.sig.inputs.is_empty()
            && util::type_is_unit(&item.sig.output);

        if valid_signature {
            return Ok(EarlyInit {
                attrs: item.attrs,
                name: item.sig.ident,
                stmts: item.block.stmts,
                is_extern: false,
            });
        }

        Err(parse::Error::new(
            item.sig.ident.span(),
            "this `#[early_init]` function must have signature `fn()`".to_string(),
        ))
    }

    pub(crate) fn parse_foreign(item: ForeignItemFn) -> parse::Result<Self> {
        let valid_signature = util::check_foreign_fn_signature(&item, false)
            && item.sig.inputs.is_empty()
            && util::type_is_unit(&item.sig.output);

        if valid_signature {
            return Ok(EarlyInit {
                attrs: item.attrs,
                name: item.sig.ident,
                stmts: Vec::<Stmt>::new(),
                is_extern: true,
            });
        }

        Err(parse::Error::new(
            item.sig.ident.span(),
            "this `#[early_init]` function must have signature `fn()`".to_string(),
        ))
    }
}
