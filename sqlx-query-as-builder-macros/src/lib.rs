use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use sqlx_macros_core::*;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, Token};

#[proc_macro]
pub fn expand_query_as_builder(input: TokenStream) -> TokenStream {
    let BuilderQueryInput {
        builder_expr,
        query_input,
    } = syn::parse_macro_input!(input as BuilderQueryInput);

    let query_macro_input: query::QueryMacroInput = match syn::parse2(query_input) {
        Ok(input) => input,
        Err(e) => return e.to_compile_error().into(),
    };

    let sqlx_output = match query::expand_input(query_macro_input, FOSS_DRIVERS) {
        Ok(ts) => ts,
        Err(e) => {
            if let Some(parse_err) = e.downcast_ref::<syn::Error>() {
                return parse_err.to_compile_error().into();
            } else {
                let msg = e.to_string();
                return quote!(::std::compile_error!(#msg)).into();
            }
        }
    };

    transform_query_output(builder_expr, sqlx_output).into()
}

fn transform_query_output(builder_expr: Expr, sqlx_output: TokenStream2) -> TokenStream2 {
    let mut expr: Expr = syn::parse2(sqlx_output).expect("Failed to parse sqlx output");

    let mut replacer = StructLiteralReplacer {
        builder_expr,
        replaced: false,
    };
    replacer.visit_expr_mut(&mut expr);

    if !replacer.replaced {
        return quote! {
            ::std::compile_error!("query_as_builder: no struct literal found in sqlx output to replace")
        };
    }

    quote! { #expr }
}

struct BuilderQueryInput {
    builder_expr: Expr,
    query_input: TokenStream2,
}

impl Parse for BuilderQueryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let builder_expr = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let query_input = input.parse()?;

        Ok(BuilderQueryInput {
            builder_expr,
            query_input,
        })
    }
}

struct StructLiteralReplacer {
    builder_expr: Expr,
    replaced: bool,
}

impl VisitMut for StructLiteralReplacer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Struct(expr_struct) = expr
            && !self.replaced
        {
            let struct_path = &expr_struct.path;
            let struct_fields = &expr_struct.fields;
            let builder = &self.builder_expr;

            let mut builder_calls = quote! { #builder };
            for field in struct_fields {
                let field_name = &field.member;
                builder_calls = quote! { #builder_calls.#field_name(record.#field_name) };
            }

            let block = quote! {
                {
                    let record = #struct_path { #struct_fields };
                    #builder_calls
                }
            };

            *expr = syn::parse2(block).unwrap();
            self.replaced = true;
            return;
        }

        visit_mut::visit_expr_mut(self, expr);
    }
}
