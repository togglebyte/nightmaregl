use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Field, Fields, Lit,
    Meta, MetaNameValue, Type,
};

use nightmare::vertexpointers::GlType;

// -----------------------------------------------------------------------------
//     - Convenience functions -
//     Get integers, strings etc.
// -----------------------------------------------------------------------------
fn normalize(attrs: &[Attribute]) -> bool {
    get_attr(attrs, "normalize").is_some()
}

fn parse_divisor(attrs: &[Attribute]) -> Option<u32> {
    parse_int(attrs, "divisor")
}

fn parse_location(attrs: &[Attribute]) -> Option<u32> {
    parse_int(attrs, "location")
}

fn get_attr<'a>(attrs: &'a [Attribute], ident: &str) -> Option<&'a Attribute> {
    attrs
        .iter()
        .filter(|a| match a.path.get_ident() {
            Some(i) => i == ident,
            None => false,
        })
        .next()
}

fn parse_lit(attrs: &[Attribute], ident_name: &str) -> Option<Lit> {
    let attr = get_attr(attrs, ident_name)?;

    let lit = match attr.parse_meta() {
        Ok(Meta::NameValue(MetaNameValue { lit, .. })) => lit,
        _ => panic!("Failed to parse literal value for \"{}\"", ident_name),
    };

    Some(lit)
}

// -----------------------------------------------------------------------------
//     - Parse args -
// -----------------------------------------------------------------------------
fn parse_int(attrs: &[Attribute], ident_name: &str) -> Option<u32> {
    let lit = parse_lit(attrs, ident_name)?;

    match lit {
        Lit::Int(int) => Some(
            int.base10_parse::<u32>()
                .expect(&format!("\"{}\" needs to be a value u32", ident_name)),
        ),
        _ => None,
    }
}

fn parse_str(attrs: &[Attribute], ident_name: &str) -> Option<String> {
    let lit = parse_lit(attrs, ident_name)?;

    match lit {
        Lit::Str(s) => Some(s.value()),
        _ => None,
    }
}

// -----------------------------------------------------------------------------
//     - Type to GlType -
// -----------------------------------------------------------------------------
fn type_to_gl(ty: Type) -> GlType {
    match ty {
        Type::Path(type_path) => {
            let ident = type_path.path.get_ident().expect("Failed to get the type");

            if ident == "f32" {
                GlType::Float
            } else if ident == "i32" {
                GlType::Int
            } else {
                panic!("{}: type has to be either f32 or i32", ident)
            }
        }
        _ => panic!("`{:?}` is not supported", stringify!(ty)),
    }
}

// -----------------------------------------------------------------------------
//     - Proc macro -
// -----------------------------------------------------------------------------
#[proc_macro_derive(VertexData, attributes(location, divisor, gl_type))]
pub fn vertex_data(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs can be VertexData"),
    };

    let fields = process_fields(&fields, name.clone());

    let modified = quote! {
        impl nightmare::vertexpointers::VertexPointersT for #name {
            fn vertex_pointer(vp: &mut nightmare::vertexpointers::VertexPointers) {
                #(#fields)*;
            }
        }
    };

    modified.into()
}

fn process_fields(
    fields: &Punctuated<Field, Comma>,
    name: proc_macro2::Ident,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fields.iter().map(move |field| {
        let field_ident = field.ident.as_ref().map(|ref f| f.to_string()).unwrap();

        let normalize = normalize(&field.attrs);

        let divisor = match parse_divisor(&field.attrs) {
            Some(d) => quote! { Some(nightmare::vertexpointers::Divisor(#d)) },
            None => quote! { None },
        };

        let location = parse_location(&field.attrs).expect(&format!("`{}` is missing the `location` attribute", field_ident));

        // Type of the field, e.g array, f32 etc.
        let ty = &field.ty;

        // Get underlying type if there is one

        let gl_type = match ty {
            Type::Array(arr) => match &arr.len {
                syn::Expr::Lit(syn::ExprLit {
                    lit: Lit::Int(_), ..
                }) => {
                   *arr.elem.clone()
                }
                _ => ty.clone()
            },
            _ => ty.clone()
        };

        let gl_type = parse_str(&field.attrs, "gl_type")
            .map(|t| match t.as_ref() {
                "f32" => GlType::Float,
                "i32" => GlType::Int,
                _ => panic!(
                    "`{}` has an invalid gl_type: `{}` is an invalid type. Use either f32 or i32",
                    field_ident, t
                ),
            })
            .unwrap_or_else(|| type_to_gl(gl_type));

        quote! {
            let total_param_count = (std::mem::size_of::<#ty>() as i32 + 3) / 4;

            for entry in (0..total_param_count).step_by(4) {
                let param_count = total_param_count.min(4);
                let location = #location + entry as u32 / 4;

                vp.add::<#name>(
                    nightmare::vertexpointers::Location(location),
                    nightmare::vertexpointers::ParamCount(param_count),
                    #gl_type,
                    #normalize,
                    #divisor,
                );
            }
        }
    })
}
