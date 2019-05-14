extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

// use crate::proc_macro::TokenStream;
use quote::quote;
// use syn;
use syn::DeriveInput;

use proc_macro::TokenStream;
use syn::{braced, parse_macro_input, token, Field, Ident, LitStr, Result, Token};

use syn::parse::{Parse, ParseStream, Result as ParseResult};

struct Foo {
    name: syn::Ident,
    path: String,
}

impl Parse for Foo {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let name = input.parse::<Ident>()?;
        let path = input.parse::<LitStr>()?.value();

        Ok(Foo { name, path })
    }
}

#[proc_macro]
pub fn make_get_router(tokens: TokenStream) -> TokenStream {
    use crate::proc_macro::TokenStream;
    use syn::parse::Parser;
    use syn::punctuated::Punctuated;
    use syn::Item;

    let parser = Punctuated::<Foo, Token![,]>::parse_terminated;
    let args = parser.parse(tokens).unwrap();

    let def_start = "pub fn get_router() -> router::Router { use router::Router; let mut router = Router::new();";
    let def_end = " router }";

    let mut def_acc = "".to_string();

    for a in args {
        let n = a.name.to_string();
        let p = a.path;
        def_acc.push_str(&format!("#[path=\"{}.rs\"] mod {};", &n, &n));
        def_acc.push_str(&format!(
            "router.get(\"{}\", {}::PageState::handler, \"{}\");",
            &p, &n, &p
        ));
        def_acc.push_str(&format!(
            "router.post(\"{}\", {}::PageState::handler, \"{}\");",
            &p, &n, &p
        ));
    }
    let full_def = format!("{}{}{}", def_start, def_acc, def_end);
    full_def.parse().unwrap()
}

#[proc_macro_derive(RspTraits, attributes(html, table, field))]
pub fn rsp_traits_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use crate::proc_macro::TokenStream;
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
       impl RspStateName for #name {
          fn get_template_name() -> String {
             // stringify!(#name).to_string()
             module_path!().split("::").last().unwrap().to_string()
          }
       }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(RspHandlers, attributes(html, html_fill, html_hook, table, field))]
pub fn rsp_handlers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use crate::proc_macro::TokenStream;

    let ast = parse_macro_input!(input as DeriveInput);

    impl_handlers(&ast)
}

fn get_field_attr<'a>(name: &str, v: &'a Vec<syn::Attribute>) -> Option<&'a syn::Attribute> {
    if v.len() > 0 {
        for attr in v {
            let type_name = &attr
                .path
                .segments
                .pairs()
                .last()
                .unwrap()
                .into_value()
                .ident;
            if type_name == name {
                return Some(&attr);
            }
        }
        return None;
    } else {
        return None;
    }
}

fn impl_handlers(ast: &syn::DeriveInput) -> TokenStream {
    let def_start = "\n\nmacro_rules! derived_html_inputs { ($gd: ident, $key: ident, $state: ident, $default_state: ident, $modified: ident) => {";
    let def_end = "}; }";
    let mut def_acc = "".to_string();

    if let syn::Data::Struct(datastruct) = &ast.data {
        if let syn::Fields::Named(fieldsnamed) = &datastruct.fields {
            let mut out_debug: Vec<String> = vec![];

            let nfields = fieldsnamed.named.len();
            out_debug.push(format!("total fields: {}", &nfields));

            for field in fieldsnamed.named.iter() {
                let hook_attr = get_field_attr("html_hook", &field.attrs);
                let hook_text = if let Some(attr) = hook_attr {
                    format!("{};", &attr.tts)
                } else {
                    format!("")
                };
                def_acc.push_str(&hook_text);

                let field_name = if let Some(ident) = &field.ident {
                    format!("{}", &ident)
                } else {
                    panic!("Could not determine field name");
                };
                let html_field_name = format!("html_{}", &field_name);
                let field_type = if let syn::Type::Path(tp) = &field.ty {
                    // use quote::ToTokens;
                    if tp.path.segments.len() > 1 {
                        panic!("Only simple (path len = 1) types are supported)");
                    }
                    let type_name = &tp.path.segments.pairs().last().unwrap().into_value().ident;
                    format!("{}", &type_name)
                } else {
                    panic!("Only simple types (path, path len = 1) are supported");
                };
                if field_name.starts_with("cb") {
                    def_acc.push_str(&format!(
                        "html_check!($gd, {}, $state, $default_state, $modified);\n",
                        &field_name
                    ));
                } else if field_name.starts_with("txt") {
                    def_acc.push_str(&format!(
                        "html_text!($gd, {}, $state, $default_state, $modified);\n",
                        &field_name
                    ));
                } else if field_name.starts_with("btn") {
                    let setup_attr = get_field_attr("html_fill", &field.attrs);
                    if let Some(attr) = setup_attr {
                        out_debug.push(format!(" fill: {}", &attr.tts));
                        let fill_tokens = format!("{}", &attr.tts);
                        def_acc.push_str(&format!(
                            "html_button!($gd, {}, {});\n",
                            &field_name, &fill_tokens
                        ));
                    } else {
                        def_acc.push_str(&format!(
                            "html_button!($gd, {}, {});\n",
                            &field_name, &field_name
                        ));
                    }
                } else if field_name.starts_with("dd") {
                    let setup_attr = get_field_attr("html_fill", &field.attrs);
                    if let Some(attr) = setup_attr {
                        out_debug.push(format!(" fill: {}", &attr.tts));
                        let fill_tokens = format!("{}", &attr.tts);
                        def_acc.push_str(&format!(
                            "html_select!($gd, {}, {}, $state, $default_state, $modified);\n",
                            &field_name, &fill_tokens
                        ));
                    } else {
                        panic!(
                            "field {} is dropdown, requires 'html_fill' attribute",
                            &field_name
                        );
                    }
                }

                out_debug.push(format!(" {} : {}", &field_name, &field_type));
            }

            println!("{:#?}", &out_debug);
            let full_def = format!("{}\n{}\n{}", def_start, def_acc, def_end);
            println!("full def: {}", &full_def);
            full_def.parse().unwrap()
        } else {
            panic!("Can not derive on unnamed fields");
        }
    } else {
        panic!("Can not derive RspHandlers on non-struct");
    }
}

#[proc_macro_derive(RspXHandlers, attributes(html, table, field))]
pub fn rsp_xhandlers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
