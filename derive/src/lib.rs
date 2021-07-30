use std::mem::size_of;

use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Expr, Field, Fields, Ident, Lit,
    Meta, MetaNameValue, Type,
};

use nightmare::renderer::vertexpointers::{new_vertex_pointers, GlType, VertexPointers};
use nightmare::Context;

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

fn parse_uniform(attrs: &[Attribute], field_name: &str) -> Option<String> {
    match get_attr(attrs, "uniform").is_some() {
        true => Some(parse_str(attrs, "uniform").unwrap_or(field_name.to_owned())),
        false => None
    }
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
            eprintln!("{:#?}", type_path.path.get_ident());
            eprintln!("--------------");

            let ident = type_path.path.get_ident().unwrap_or_else(|| {
                eprintln!("the very thing that won't work: {:#?}", type_path.path);
                eprintln!("--------------");
                panic!();
            });
            if ident == "f32" {
                GlType::Float
            } else if ident == "i32" {
                GlType::Int
            } else {
                panic!("type has to be either f32 or i32")
            }
        }
        _ => panic!("Invalid type"),
    }
}

// -----------------------------------------------------------------------------
//     - Proc macro -
// -----------------------------------------------------------------------------
#[proc_macro_derive(VertexData, attributes(location, divisor, uniform))]
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
        impl #name {
            pub fn vertex_pointer(vp: &mut nightmare::renderer::VertexPointers) {
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
            Some(d) => quote!{ Some(nightmare::renderer::vertexpointers::Divisor(#d)) },
            None => quote!{ None }
        };

        let location = parse_location(&field.attrs);
        let uniform = parse_uniform(&field.attrs, &field_ident);

        // Either the location (for pointers) or a uniform has
        // to be set, otherwise this is invalid vertex data.
        if location.is_none() && uniform.is_none() {
            panic!("Either a `location` or a `uniform` has to be set");
        }

        // Type of the field, e.g array, f32 etc.
        let ty = &field.ty;

        // Get underlying type if there is one

        let (param_count, gl_type) = match ty {
            Type::Array(arr) => {
                match &arr.len {
                    syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(lit), .. }) => {
                        let ty = *arr.elem.clone();
                        (lit.base10_parse::<i32>().expect("Typical type problems"), ty)
                    }
                    _ => (1, ty.clone()),
                }
            }
            _ => (1, ty.clone()),
        };

        let gl_type = type_to_gl(gl_type);

        // let gl_type = GlType::Float;

        match (location, uniform) {
            (Some(location), None) => quote!{
               vp.add::<#name>(
                   nightmare::renderer::vertexpointers::Location(#location),
                   nightmare::renderer::vertexpointers::ParamCount(#param_count),
                   #gl_type,
                   #normalize,
                   #divisor,
               );
            },
            (None, Some(uniform)) => quote!{},
            (None, None) => panic!("Must have either uniform or location"),
            (Some(_), Some(_)) => panic!("Can't have both uniform and location"),
        }

        //        // let field_ident = {
        //        //     let ident = field.ident.clone().unwrap();
        //        //     field
        //        //         .attrs
        //        //         .iter()
        //        //         .filter(|attr| attr.path.get_ident().unwrap() == "prop_name")
        //        //         .next()
        //        //         .map(|attr| Ident::new("lark", ident.span()))
        //        //         .unwrap_or(ident)
        //        // };

        //        let ty = &field.ty;
        //        // let size = std::mem::size_of::<ty>();
        //        let attr = field.attrs.iter().next().map(|a| &a.tokens);

        //        let location = parse_int(&field.attrs, "location").unwrap_or(0);//.expect("location missing");

        //        let (param_count, gl_type) = match &field.ty {
        //            Type::Array(arr) => {
        //                match &arr.len {
        //                    syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(lit), .. }) => {
        //                        let ty = *arr.elem.clone();
        //                        (lit.base10_parse::<i32>().expect("Typical type problems"), ty)
        //                    }
        //                    _ => (1, ty.clone()),
        //                }
        //            }
        //            _ => (1, ty.clone()),
        //        };

        //        let gl_type = type_to_gl(gl_type);

        //        // TODO:
        //        // * Parse location as an optional, if it has a location then it's a property
        //        // * Parse uniform as optional, if it has a uniform name then it's a uniform
        //        // * If both location and uniform is set or `None` then panic
        //        // * Vertex pointers needs two functions: add_prop and add_uniform (remove add)
        //        // * Optional type field (for Matrix4<T> etc.). 
        //             `T` can be extrapolated from arrays.
        //        //
        //        // Questions:
        //        // * Where do we cache the uniform locations?
        //        // * How can I solve the underlying type of the matrix? (maybe an optional type field)
        //        // * How does this end up on the renderer? `Renderer::<(&[Transforms], [Vertex; 4])>::new())`

        // quote! {
        // //            vp.add::<#ty>(
        // //                nightmare::renderer::vertexpointers::Location(#location),
        // //                nightmare::renderer::vertexpointers::ParamCount(#param_count),
        // //                #gl_type,
        // //                #normalize
        // //            );
        //        }
    })
}
