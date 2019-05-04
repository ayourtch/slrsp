extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

// use crate::proc_macro::TokenStream;
use quote::quote;
// use syn;

#[proc_macro_derive(RspHandlers, attributes(html, table, field))]
pub fn rsp_handlers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let mut ast = syn::parse(input).unwrap();
    let mut output = proc_macro::TokenStream::new();

    // Build the trait implementation
    let gen: proc_macro2::TokenStream = impl_hello_macro(&mut ast).into();

    // let ref m = vec![1,2,3,4];
    let ref m = vec![1, 2, 3, 4];
    let mx = vec!["test1", "test2", "test3", "test4"];
    let mx_ident: Vec<syn::Expr> = mx
        .into_iter()
        .map(|x| syn::parse_str(&format!("id_{}", x)).unwrap())
        .collect();
    // let m1 = mx;
    // let x: syn::Type = syn::parse_str("std::collections::HashMap<String, Value>").unwrap();
    let x: syn::Expr = syn::parse_str("i32").unwrap();

    // let x_i32: syn::Type = syn::parse_str("i32").unwrap();

    let mut output2: proc_macro::TokenStream = quote! {
       fn Test() {
          #(let #mx_ident  = #m;)*
       }
    }
    .into();

    let ref struct_field_names = vec!["ddTestValue", "txtAnotherValue"];
    let ref struct_field_types = vec!["i32", "String"];
    let sf_names: Vec<syn::Expr> = struct_field_names
        .into_iter()
        .map(|x| syn::parse_str(&format!("{}", x)).unwrap())
        .collect();
    let sf_types: Vec<syn::Expr> = struct_field_types
        .into_iter()
        .map(|x| syn::parse_str(&format!("{}", x)).unwrap())
        .collect();
    let hhh: syn::Expr = syn::parse_str("html_input(test, test)").unwrap();

    output.extend(output2);
    // panic!("{:#?}", output);
    output.into()
}
fn impl_hello_macro(ast: &mut syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let mut fields_acc: Vec<(String, String)> = vec![];

    match &ast.data {
        syn::Data::Struct(ref a_struct) => {
            match &a_struct.fields {
                syn::Fields::Named(ref a_fields) => {
                    // a_fields.named.pairs().for_each(|x| panic!("{:?}", &x.into_value().ident));
                    parse_fields(a_fields);
                }
                _ => {
                    panic!("rspten: struct should contain only named fields");
                }
            }
        }
        _ => {
            panic!("not a struct");
        }
    };
    let gen = quote! {
        // impl HelloMacro for #name {
            fn handler() {
                // println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        // }
    };
    gen.into()
}

fn parse_fields(f: &syn::FieldsNamed) {
    use quote::ToTokens;

    let mut dd: Vec<String> = vec![];
    f.named.pairs().for_each(|x| {
        // let () = x.into_value();
        let fld = x.into_value();
        dd.push(format!("{:?}", &fld.ident));
        for attr in &fld.attrs {
            // dd.push(format!("    attr {:?}", &attr.tts));
            let mm = attr.interpret_meta();
            if let Some(mm_val) = mm {
                dd.push(format!("    attr {:?}", &mm_val.name()));
            }
        }
        match &fld.ty {
            syn::Type::Verbatim(vtype) => {
                dd.push(format!("{:?}", &vtype.tts));
            }
            syn::Type::Path(vtype) => {
                dd.push("-path-".to_string());
                // dd.push(format!("path {:?}", &vtype.into_token_stream()));
                dd.push(format!("path {:?}", &vtype.into_token_stream()));
                dd.push(format!("len: {}", vtype.path.segments.len()));
                vtype.path.segments.pairs().for_each(|tp| {
                    dd.push(format!("{}", &tp.into_value().ident));
                });
            }
            _ => {
                dd.push("--".to_string());
            }
        }
    });
    println!("{:#?}", dd);
    // panic!("{:#?}", dd);
}
