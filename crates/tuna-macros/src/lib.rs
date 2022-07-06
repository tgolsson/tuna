use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Const},
    Attribute, Expr, Ident, Meta, Token, Visibility,
};

struct FieldLike {
    pub attrs: Vec<Meta>,
    pub vis: Visibility,
    pub constness: Option<Const>,
    pub ident: Ident,
    pub colon_token: Option<Token![:]>,
    pub ty: Ident,
    pub equals: Token![=],
    pub defaults: Expr,
}

impl quote::ToTokens for FieldLike {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let FieldLike {
            attrs,
            vis,
            constness,
            ident,
            colon_token,
            ty,
            equals,
            defaults,
        } = self;

        let ty = format!("{}", ty);

        let (variable_type, numeric) = match ty.as_str() {
            "f32" => ("Float32", true),
            "i32" => ("Int32", true),
            "f64" => ("Float64", true),
            "i64" => ("Int64", true),
            "bool" => ("Boolean", false),
            _ => panic!("unknown variable type"),
        };

        let ty = format_ident!("{}", variable_type);
        let default = quote! {
            #defaults
        };

        let out = if numeric {
            let min = attrs
                .iter()
                .find(|v| v.path().is_ident("min"))
                .map(|v| match v {
                    Meta::NameValue(v) => v,
                    _ => panic!("accepts only kv pairs"),
                })
                .map_or(quote! {None}, |v| {
                    let lit = &v.lit;
                    quote! { Some(#lit) }
                });

            let max = attrs
                .iter()
                .find(|v| v.path().is_ident("max"))
                .map(|v| match v {
                    Meta::NameValue(v) => v,
                    _ => panic!("accepts only kv pairs"),
                })
                .map_or(quote! {None}, |v| {
                    let lit = &v.lit;
                    quote! { Some(#lit) }
                });

            quote! {
                #vis #constness #ident #colon_token tuna::#ty #equals tuna::#ty::new(NAME, stringify!(#ident), #default, #min, #max)
            }
        } else {
            quote! {
                #vis #constness #ident #colon_token tuna::#ty #equals tuna::#ty::new(NAME, stringify!(#ident), #default)
            }
        };

        tokens.extend(out);
    }
}

impl Parse for FieldLike {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(FieldLike {
            attrs: input
                .call(Attribute::parse_outer)?
                .iter()
                .map(|a| a.parse_meta())
                .collect::<Result<Vec<_>, _>>()?,
            vis: input.parse()?,
            constness: input.parse()?,
            ident: if input.peek(Token![_]) {
                input.call(Ident::parse_any)
            } else {
                input.parse()
            }?,
            colon_token: Some(input.parse()?),
            ty: input.parse()?,
            equals: input.parse()?,
            defaults: input.parse()?,
        })
    }
}

#[allow(unused)]
struct Input {
    visibility: Visibility,
    struct_token: Token![mod],
    name: Ident,
    brace_token: Brace,
    fields: Punctuated<FieldLike, Token![;]>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let visibility = input.parse()?;
        let struct_token = input.parse()?;
        let name = input.parse()?;
        let brace_token = braced!(content in input);
        let fields = content.parse_terminated(FieldLike::parse)?;
        Ok(Input {
            visibility,
            struct_token,
            name,
            brace_token,
            fields,
        })
    }
}

#[proc_macro_attribute]
pub fn tuna(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let Input {
        name,
        fields,
        visibility,
        ..
    } = parse_macro_input!(item as Input);

    let fns = fields
        .iter()
        .map(|f| {
            let name = &f.ident;
            quote! {
                        #name.register();
            }
        })
        .collect::<Vec<_>>();
    let res = quote!(
        #visibility mod #name {
            const NAME: &str = stringify!(#name);
            #fields

            pub fn register() {
                #(#fns)*
            }
        }
    );

    res.into()
}
