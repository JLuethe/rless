//! Syntax inventory, not a sound alias analysis or a complete indirect call graph.
use proc_macro2::Span;
use quote::ToTokens;
use std::{env, fs};
use syn::{spanned::Spanned, visit::Visit};

struct Inventory {
    file: String,
    context: Vec<String>,
}
impl Inventory {
    fn emit(&self, kind: &str, span: Span, symbol: impl AsRef<str>) {
        let pos = span.start();
        let symbol = symbol.as_ref().replace(['\t', '\n', '\r'], " ");
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            kind,
            self.file,
            pos.line,
            pos.column + 1,
            self.context.join("::"),
            symbol
        );
    }
    fn signature(&self, sig: &syn::Signature) {
        self.emit("function_definition", sig.span(), sig.ident.to_string());
        if sig.unsafety.is_some() {
            self.emit("unsafe_function", sig.span(), sig.ident.to_string());
        }
        if sig.abi.is_some() {
            self.emit("abi_function_definition", sig.span(), sig.ident.to_string());
        }
    }
}
impl<'a> Visit<'a> for Inventory {
    fn visit_item_mod(&mut self, item: &'a syn::ItemMod) {
        self.context.push(item.ident.to_string());
        syn::visit::visit_item_mod(self, item);
        self.context.pop();
    }
    fn visit_item_fn(&mut self, item: &'a syn::ItemFn) {
        self.signature(&item.sig);
        if item
            .attrs
            .iter()
            .any(|a| a.path().is_ident("no_mangle") || a.path().is_ident("export_name"))
        {
            self.emit("exported_symbol", item.span(), item.sig.ident.to_string());
        }
        self.context.push(item.sig.ident.to_string());
        syn::visit::visit_item_fn(self, item);
        self.context.pop();
    }
    fn visit_impl_item_fn(&mut self, item: &'a syn::ImplItemFn) {
        self.signature(&item.sig);
        self.context.push(item.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, item);
        self.context.pop();
    }
    fn visit_item_impl(&mut self, item: &'a syn::ItemImpl) {
        if item.unsafety.is_some() {
            self.emit(
                "unsafe_impl",
                item.span(),
                item.self_ty.to_token_stream().to_string(),
            );
        }
        self.context
            .push(item.self_ty.to_token_stream().to_string());
        syn::visit::visit_item_impl(self, item);
        self.context.pop();
    }
    fn visit_item_trait(&mut self, item: &'a syn::ItemTrait) {
        if item.unsafety.is_some() {
            self.emit("unsafe_trait", item.span(), item.ident.to_string());
        }
        syn::visit::visit_item_trait(self, item);
    }
    fn visit_expr_unsafe(&mut self, item: &'a syn::ExprUnsafe) {
        self.emit("unsafe_block", item.span(), "unsafe");
        syn::visit::visit_expr_unsafe(self, item);
    }
    fn visit_item_static(&mut self, item: &'a syn::ItemStatic) {
        if matches!(item.mutability, syn::StaticMutability::Mut(_)) {
            self.emit(
                "mutable_static_definition",
                item.span(),
                item.ident.to_string(),
            );
        }
        syn::visit::visit_item_static(self, item);
    }
    fn visit_foreign_item_static(&mut self, item: &'a syn::ForeignItemStatic) {
        self.emit(
            "extern_static_declaration",
            item.span(),
            item.ident.to_string(),
        );
        syn::visit::visit_foreign_item_static(self, item);
    }
    fn visit_foreign_item_fn(&mut self, item: &'a syn::ForeignItemFn) {
        self.emit(
            "extern_function_declaration",
            item.span(),
            item.sig.ident.to_string(),
        );
        for attr in &item.attrs {
            if attr.path().is_ident("link_name") {
                self.emit(
                    "native_link_name",
                    attr.span(),
                    attr.meta.to_token_stream().to_string(),
                );
            }
        }
        syn::visit::visit_foreign_item_fn(self, item);
    }
    fn visit_type_ptr(&mut self, item: &'a syn::TypePtr) {
        self.emit(
            "raw_pointer_type",
            item.span(),
            item.to_token_stream().to_string(),
        );
        syn::visit::visit_type_ptr(self, item);
    }
    fn visit_type_bare_fn(&mut self, item: &'a syn::TypeBareFn) {
        self.emit(
            "function_pointer_type",
            item.span(),
            item.to_token_stream().to_string(),
        );
        syn::visit::visit_type_bare_fn(self, item);
    }
    fn visit_expr_call(&mut self, item: &'a syn::ExprCall) {
        if let syn::Expr::Path(path) = item.func.as_ref() {
            self.emit(
                "direct_call",
                item.span(),
                path.to_token_stream().to_string(),
            );
        } else {
            self.emit(
                "indirect_call",
                item.span(),
                item.func.to_token_stream().to_string(),
            );
        }
        syn::visit::visit_expr_call(self, item);
    }
    fn visit_expr_method_call(&mut self, item: &'a syn::ExprMethodCall) {
        self.emit("method_call", item.span(), item.method.to_string());
        syn::visit::visit_expr_method_call(self, item);
    }
    fn visit_macro(&mut self, item: &'a syn::Macro) {
        self.emit(
            "macro_invocation",
            item.span(),
            item.path.to_token_stream().to_string(),
        );
        syn::visit::visit_macro(self, item);
    }
}
fn main() {
    for file in env::args().skip(1) {
        let source = fs::read_to_string(&file).unwrap_or_else(|e| panic!("{file}: {e}"));
        let parsed = syn::parse_file(&source).unwrap_or_else(|e| panic!("{file}: {e}"));
        Inventory {
            file,
            context: Vec::new(),
        }
        .visit_file(&parsed);
    }
}
