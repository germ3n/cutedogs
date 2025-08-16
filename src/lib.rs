#![allow(unused_assignments)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn document(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let args_str = args.to_string();
    let mut summary = String::new();
    let mut returns = String::new();
    let mut params = Vec::<(String, String)>::new();
    let mut deprecated = String::new();
    let mut deprecated_since = String::new();
    let mut since = String::new();
    let mut example = String::new();
    let mut panics = String::new();
    let mut safety = String::new();
    let mut unimplemented_reason = String::new();
    let mut see_also = String::new();
    let mut invariants = String::new();
    let mut note = String::new();
    let is_unimplemented = args_str.trim() == "unimplemented" || args_str.trim().starts_with("unimplemented");
    
    if is_unimplemented {
        if args_str.trim() == "unimplemented" {
            summary = "⚠️ **NOT IMPLEMENTED** - This function is not yet implemented".to_string();
            returns = "This function will panic with `unimplemented!()` when called".to_string();
        } else {
            if let Some(eq_pos) = args_str.find('=') {
                let after_eq = args_str[eq_pos + 1..].trim();
                if after_eq.starts_with('"') {
                    if let Some(end_quote) = after_eq[1..].find('"') {
                        unimplemented_reason = after_eq[1..end_quote + 1].to_string();
                    }
                }
            }
            
            if !unimplemented_reason.is_empty() {
                summary = format!("⚠️ **NOT IMPLEMENTED** - {}", unimplemented_reason);
                returns = "This function will panic with `unimplemented!()` when called".to_string();
            } else {
                summary = "⚠️ **NOT IMPLEMENTED** - This function is not yet implemented".to_string();
                returns = "This function will panic with `unimplemented!()` when called".to_string();
            }
        }
    } else {
        let parse_quoted_value = |field_name: &str| -> String {
            if let Some(start) = args_str.find(field_name) {
                if let Some(eq_pos) = args_str[start..].find('=') {
                    let after_eq = start + eq_pos + 1;
                    let remaining = args_str[after_eq..].trim();
                    if remaining.starts_with('"') {
                        if let Some(end_quote) = remaining[1..].find('"') {
                            return remaining[1..end_quote + 1].to_string();
                        }
                    }
                }
            }
            String::new()
        };

        summary = parse_quoted_value("summary");
        returns = parse_quoted_value("returns");
        deprecated = parse_quoted_value("deprecated");
        deprecated_since = parse_quoted_value("deprecated_since");
        since = parse_quoted_value("since");
        example = parse_quoted_value("example");
        panics = parse_quoted_value("panics");
        safety = parse_quoted_value("safety");
        see_also = parse_quoted_value("see_also");
        invariants = parse_quoted_value("invariants");
        note = parse_quoted_value("note");
        
        // Parse params - improved to handle whitespace properly
        if let Some(params_start) = args_str.find("params") {
            if let Some(brace_start) = args_str[params_start..].find('{') {
                let brace_start = params_start + brace_start + 1;
                if let Some(brace_end) = args_str[brace_start..].find('}') {
                    let params_content = &args_str[brace_start..brace_start + brace_end];
                    
                    let mut current_pos = 0;
                    while current_pos < params_content.len() {
                        if let Some(colon_pos) = params_content[current_pos..].find(':') {
                            let param_name = params_content[current_pos..current_pos + colon_pos]
                                .trim()
                                .trim_matches('"')
                                .trim()
                                .to_string();
                            
                            let after_colon = current_pos + colon_pos + 1;
                            let remaining = params_content[after_colon..].trim();
                            
                            if remaining.starts_with('"') {
                                if let Some(end_quote) = remaining[1..].find('"') {
                                    let param_desc = remaining[1..end_quote + 1].to_string();
                                    params.push((param_name, param_desc));
                                    
                                    current_pos = after_colon + end_quote + 2;
                                    while current_pos < params_content.len() && 
                                          (params_content.chars().nth(current_pos).unwrap().is_whitespace() || 
                                           params_content.chars().nth(current_pos) == Some(',')) {
                                        current_pos += 1;
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    
    let mut doc_parts = vec![];
    
    if !deprecated.is_empty() || !deprecated_since.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **DEPRECATED**"] });
        
        if !deprecated.is_empty() && !deprecated_since.is_empty() {
            let deprecation_msg = format!("**Deprecated since {}:** {}", deprecated_since, deprecated);
            doc_parts.push(quote! { #[doc = #deprecation_msg] });
        } else if !deprecated.is_empty() {
            let deprecation_msg = format!("{}", deprecated);
            doc_parts.push(quote! { #[doc = #deprecation_msg] });
        } else if !deprecated_since.is_empty() {
            let deprecation_msg = format!("**Deprecated since:** {}", deprecated_since);
            doc_parts.push(quote! { #[doc = #deprecation_msg] });
        }
        
        doc_parts.push(quote! { #[doc = ""] });
    }
    
    if is_unimplemented {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "⚠️ **WARNING: NOT IMPLEMENTED**"] });
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "This function is a placeholder and will panic when called."] });
        doc_parts.push(quote! { #[doc = "It should not be used in production code."] });
        doc_parts.push(quote! { #[doc = ""] });
    }
    
    if !summary.is_empty() {
        doc_parts.push(quote! { #[doc = #summary] });
    }

    if !since.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        let since_msg = format!("**Since:** {}", since);
        doc_parts.push(quote! { #[doc = #since_msg] });
    }
    
    if !params.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Parameters"] });
        for (name, desc) in &params {
            let param_doc = format!("* `{}` - {}", name, desc);
            doc_parts.push(quote! { #[doc = #param_doc] });
        }
    }
    
    if !returns.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Returns"] });
        doc_parts.push(quote! { #[doc = #returns] });
    }

    if !example.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Example"] });
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "```rust"] });
        doc_parts.push(quote! { #[doc = #example] });
        doc_parts.push(quote! { #[doc = "```"] });
    }

    if !panics.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Panics"] });
        doc_parts.push(quote! { #[doc = #panics] });
    }

    if !safety.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Safety"] });
        doc_parts.push(quote! { #[doc = #safety] });
    }

    if !see_also.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# See Also"] });
        
        let functions: Vec<&str> = see_also.split(',').map(|s| s.trim()).collect();
        for func in functions {
            let link_doc = if func.contains("::") {
                format!("* [`{}`]", func)
            } else if func.starts_with("crate::") {
                format!("* [`{}`]", func)
            } else {
                format!("* [`{}`](crate::{}) | [`Self::{}`] | [`{}`]", func, func, func, func)
            };
            doc_parts.push(quote! { #[doc = #link_doc] });
        }
    }

    if !invariants.is_empty() {
        doc_parts.push(quote! { #[doc = ""] });
        doc_parts.push(quote! { #[doc = "# Invariants"] });
        doc_parts.push(quote! { #[doc = #invariants] });
    }

    if !note.is_empty() {
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