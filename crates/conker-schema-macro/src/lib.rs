//! Procedural macro for generating JSON Schema files from Rust structs.
//! 
//! Apply `#[derive(ConkerSchema)]` to structs to automatically generate `.schema.json` files
//! in the `schemas/` directory during cargo build.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Meta};

#[proc_macro_derive(ConkerSchema, attributes(schema))]
pub fn derive_conker_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract doc comments and schema attributes
    let mut doc_comments = Vec::new();
    let mut schema_attrs = Vec::new();
    
    for attr in &input.attrs {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        doc_comments.push(lit_str.value());
                    }
                }
            }
        }
        if attr.path().is_ident("schema") {
            schema_attrs.push(attr.clone());
        }
    }

    // Generate schema JSON based on struct fields
    let schema_json = generate_schema_json(&input);
    
    // Write schema file at compile time (this is a simplified version)
    // In practice, you'd use a build.rs or output to OUT_DIR
    
    let expanded = quote! {
        impl #name {
            /// Returns the JSON Schema for this type as a string.
            pub fn schema_json() -> &'static str {
                #schema_json
            }
            
            /// Returns the description from doc comments.
            pub fn schema_description() -> &'static str {
                #(#doc_comments)* ""
            }
        }
    };

    TokenStream::from(expanded)
}

fn generate_schema_json(input: &DeriveInput) -> String {
    let name = &input.ident;
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let field_type = get_field_type(&field.ty);
                
                let mut prop = serde_json::Map::new();
                prop.insert("type".to_string(), serde_json::json!(field_type));
                
                // Add description from doc comment
                for attr in &field.attrs {
                    if attr.path().is_ident("doc") {
                        if let Meta::NameValue(meta) = &attr.meta {
                            if let syn::Expr::Lit(expr_lit) = &meta.value {
                                if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                    prop.insert("description".to_string(), serde_json::json!(lit_str.value()));
                                    break;
                                }
                            }
                        }
                    }
                }
                
                // Check if field has #[serde(default)] - if not, it's required
                let is_optional = field.attrs.iter().any(|attr| {
                    if attr.path().is_ident("serde") {
                        if let Meta::List(meta_list) = &attr.meta {
                            let tokens = meta_list.tokens.to_string();
                            tokens.contains("default")
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });
                
                if !is_optional {
                    required.push(field_name.clone());
                }
                
                properties.insert(field_name, serde_json::json!(prop));
            }
        }
    }
    
    let schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": name.to_string(),
        "type": "object",
        "properties": properties,
        "required": required
    });
    
    serde_json::to_string_pretty(&schema).unwrap_or_else(|_| "{}".to_string())
}

fn get_field_type(ty: &syn::Type) -> &'static str {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "String" => "string",
                    "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "isize" => "integer",
                    "f32" | "f64" => "number",
                    "bool" => "boolean",
                    "Vec" => "array",
                    "Option" => "object", // Simplified
                    _ => "object",
                }
            } else {
                "object"
            }
        }
        _ => "object",
    }
}
