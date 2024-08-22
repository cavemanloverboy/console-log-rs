extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, visit_mut::VisitMut, Expr, ExprCall, ExprMethodCall,
    ExprPath, Ident, ItemMod,
};

#[proc_macro_attribute]
pub fn console_log(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut module = parse_macro_input!(item as ItemMod);
    let replace_with = if attr.is_empty() {
        quote! { println! }
    } else {
        attr.into()
    };
    let mut replacer = ConsoleLogReplacer { replace_with };

    // Print the transformed module for debugging
    replacer.visit_item_mod_mut(&mut module);

    TokenStream::from(quote! {
        #module
    })
}

struct ConsoleLogReplacer {
    replace_with: proc_macro2::TokenStream,
}

impl VisitMut for ConsoleLogReplacer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::MethodCall(ExprMethodCall {
            receiver,
            method,
            args,
            attrs,
            ..
        }) = expr
        {
            let Expr::Path(ExprPath { path, .. }) = &**receiver else {
                return;
            };

            let is_console = path.is_ident(&Ident::new("console", path.span()));
            let is_log = method.to_string() == "log";

            if is_console & is_log {
                // Replace this with println!
                *expr = Expr::Call(ExprCall {
                    attrs: attrs.clone(),
                    func: Box::new(Expr::Verbatim(self.replace_with.clone())),
                    paren_token: Default::default(),
                    args: args.clone(),
                });
            }
        }
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}
