#![allow(unused_assignments)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, ItemFn, LitStr, Result, Token,
};

#[derive(Default)]
struct DocArgs {
    summary: Option<String>,
    returns: Option<String>,
    params: Vec<(String, String)>,
    deprecated: Option<String>,
    deprecated_since: Option<String>,
    since: Option<String>,
    example: Option<String>,
    panics: Option<String>,
    safety: Option<String>,
    see_also: Option<String>,
    invariants: Option<String>,
    note: Option<String>,
    is_unimplemented: bool,
    unimplemented_reason: Option<String>,
}

impl Parse for DocArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let fork = input.fork();
        if let Ok(key) = fork.parse::<Ident>() {
            if key == "unimplemented" {
                input.parse::<Ident>()?;

                if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;
                    let reason = input.parse::<LitStr>()?.value();
                    return Ok(DocArgs {
                        is_unimplemented: true,
                        unimplemented_reason: Some(reason),
                        ..Default::default()
                    });
                } else if input.is_empty() {
                    return Ok(DocArgs {
                        is_unimplemented: true,
                        ..Default::default()
                    });
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "unexpected token after `unimplemented` flag",
                    ));
                }
            }
        }

        let mut args = DocArgs::default();
        let fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(input)?;

        for field in fields {
            match field {
                FieldValue::Summary(val) => args.summary = Some(val.value()),
                FieldValue::Returns(val) => args.returns = Some(val.value()),
                FieldValue::Deprecated(val) => args.deprecated = Some(val.value()),
                FieldValue::DeprecatedSince(val) => args.deprecated_since = Some(val.value()),
                FieldValue::Since(val) => args.since = Some(val.value()),
                FieldValue::Example(val) => args.example = Some(val.value()),
                FieldValue::Panics(val) => args.panics = Some(val.value()),
                FieldValue::Safety(val) => args.safety = Some(val.value()),
                FieldValue::SeeAlso(val) => args.see_also = Some(val.value()),
                FieldValue::Invariants(val) => args.invariants = Some(val.value()),
                FieldValue::Note(val) => args.note = Some(val.value()),
                FieldValue::Params(params) => args.params = params,
            }
        }
        Ok(args)
    }
}

enum FieldValue {
    Summary(LitStr),
    Returns(LitStr),
    Params(Vec<(String, String)>),
    Deprecated(LitStr),
    DeprecatedSince(LitStr),
    Since(LitStr),
    Example(LitStr),
    Panics(LitStr),
    Safety(LitStr),
    SeeAlso(LitStr),
    Invariants(LitStr),
    Note(LitStr),
}

impl Parse for FieldValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        match key.to_string().as_str() {
            "summary" => Ok(FieldValue::Summary(input.parse()?)),
            "returns" => Ok(FieldValue::Returns(input.parse()?)),
            "deprecated" => Ok(FieldValue::Deprecated(input.parse()?)),
            "deprecated_since" => Ok(FieldValue::DeprecatedSince(input.parse()?)),
            "since" => Ok(FieldValue::Since(input.parse()?)),
            "example" => Ok(FieldValue::Example(input.parse()?)),
            "panics" => Ok(FieldValue::Panics(input.parse()?)),
            "safety" => Ok(FieldValue::Safety(input.parse()?)),
            "see_also" => Ok(FieldValue::SeeAlso(input.parse()?)),
            "invariants" => Ok(FieldValue::Invariants(input.parse()?)),
            "note" => Ok(FieldValue::Note(input.parse()?)),
            "params" => {
                let content;
                syn::braced!(content in input);
                let mut params = Vec::new();
                let fields = Punctuated::<Param, Token![,]>::parse_terminated(&content)?;
                for param in fields {
                    params.push((param.name.value(), param.desc.value()));
                }
                Ok(FieldValue::Params(params))
            }
            _ => Err(syn::Error::new_spanned(
                key,
                "unexpected field, expected one of: summary, returns, params, etc.",
            )),
        }
    }
}

struct Param {
    name: LitStr,
    desc: LitStr,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let desc = input.parse()?;
        Ok(Param { name, desc })
    }
}

#[proc_macro_attribute]
pub fn document(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as DocArgs);
    let input_fn = parse_macro_input!(input as ItemFn);

    let mut doc_parts = vec![];

    if args.is_unimplemented {
        let (summary, returns) = if let Some(reason) = args.unimplemented_reason {
            (
                format!("⚠️ **NOT IMPLEMENTED** - {}", reason),
                "This function will panic with `unimplemented!()` when called".to_string(),
            )
        } else {
            (
                "⚠️ **NOT IMPLEMENTED** - This function is not yet implemented".to_string(),
                "This function will panic with `unimplemented!()` when called".to_string(),
            )
        };

        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **WARNING: NOT IMPLEMENTED**"] });
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = #summary]});
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = #returns]});
        doc_parts.push(quote! { #[doc = ""] });
    }

    if let (Some(deprecated), Some(deprecated_since)) = (args.deprecated.as_ref(), args.deprecated_since.as_ref()) {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **DEPRECATED**"] });
        let msg = format!("**Deprecated since {}:** {}", deprecated_since, deprecated);
        doc_parts.push(quote! { #[doc = #msg] });
        doc_parts.push(quote! { #[doc = ""] });
    } else if let Some(deprecated) = args.deprecated.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **DEPRECATED**"] });
        doc_parts.push(quote! { #[doc = #deprecated] });
        doc_parts.push(quote! { #[doc = ""] });
    } else if let Some(deprecated_since) = args.deprecated_since.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **DEPRECATED**"] });
        let msg = format!("**Deprecated since:** {}", deprecated_since);
        doc_parts.push(quote! { #[doc = #msg] });
        doc_parts.push(quote! { #[doc = ""] });
    }


    if let Some(summary) = args.summary.as_ref() {
        doc_parts.push(quote! { #[doc = #summary] });
    }

    if let Some(since) = args.since.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        let since_msg = format!("**Since:** {}", since);
        doc_parts.push(quote! { #[doc = #since_msg] });
    }

    if !args.params.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Parameters"] });
        for (name, desc) in &args.params {
            let param_doc = format!("* `{}` - {}", name, desc);
            doc_parts.push(quote! { #[doc = #param_doc] });
        }
    }

    if let Some(returns) = args.returns.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Returns"] });
        doc_parts.push(quote! { #[doc = #returns] });
    }

    if let Some(example) = args.example.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Example"] });
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "```rust"] });
        doc_parts.push(quote! { #[doc = #example] });
        doc_parts.push(quote! { #[doc = "```"] });
    }

    if let Some(panics) = args.panics.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Panics"] });
        doc_parts.push(quote! { #[doc = #panics] });
    }

    if let Some(safety) = args.safety.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Safety"] });
        doc_parts.push(quote! { #[doc = #safety] });
    }

    if let Some(see_also) = args.see_also.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# See Also"] });

        for func in see_also.split(',').map(|s| s.trim()) {
            let link_doc = format!("* [`{}`]", func);
            doc_parts.push(quote! { #[doc = #link_doc] });
        }
    }

    if let Some(invariants) = args.invariants.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Invariants"] });
        doc_parts.push(quote! { #[doc = #invariants] });
    }

    if let Some(note) = args.note.as_ref() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Note"] });
        let note_msg = format!("⚠️ {}", note);
        doc_parts.push(quote! { #[doc = #note_msg] });
    }

    let result = quote! {
        #(#doc_parts)*
        #input_fn
    };

    result.into()
}
