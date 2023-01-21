extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Token, LitStr, Ident};
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use quote::format_ident;

#[proc_macro]
pub fn github_event(input: TokenStream) -> TokenStream {
    let tokens = input.clone();
    let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
    let args = parser.parse(tokens).unwrap();

    let event_name = args.first().unwrap().value();
    let event_indent = format_ident!("{}", event_name);
    let activity_ident = format_ident!("{}ActivityType", event_name);
    let activity_types: Vec<Ident> = args.iter().skip(1)
        .map(|tok| tok.value())
        .map(|tok| format_ident!("{tok}"))
        .collect();
    
    // eprintln!("Activity Name: {activity_ident:?}");
    // eprintln!("Activity Types: {activity_types:?}");
    
    // TODO: Remove the types field if there are no parseable types.
    
    let event_type = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "snake_case")]
        pub struct #event_indent {
            pub types: Option<Vec<#activity_ident>>,
            pub github_sha: Option<String>,
            pub github_ref: Option<String>,
        }
    };
   
    // 
    // #(#activity_types),*
    let activity_enum = quote!{
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "snake_case")]
        pub enum #activity_ident {
            #(#activity_types),*
        }
    };
    // let token_out = activity_enum.to_string();
    // eprintln!("Token Result: {token_out:?}");

    quote! {
        #event_type

        #activity_enum
    }.into()
    /*
    // This is the name of the GitHub event.
    let input_str = input.to_string();
    let ast = syn::parse_fn_input(&input_str).unwrap();
    
    let foo: Ident = parse(input).unwrap();
    println!("{foo:?}");
    let event_name: LitStr = parse(input).unwrap();
    let event_name_val = event_name.value();
    // Collect the list of event types, if any.
    // let activity_types: Vec<LitStr> = parse(input).unwrap();
    let event_name_activity = format!("{event_name_val}ActivityType");
    

    // Return the generated tokens.
    quote! {
        #event_type
    }.into()    

    */

    /*
    let event_type = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "snake_case")]
        pub struct #event_name {
            pub types: Option<Vec<#event_name_activity>>,
            pub github_sha: Option<String>,
            pub github_ref: Option<String>,
        }        
    };

    let activity_types = quote! {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "snake_case")]
    };

    // Return the generated tokens.
    quote! {
        #event_type
        #activity_types
    }.into()    
    */
}
