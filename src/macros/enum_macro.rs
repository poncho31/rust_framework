use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/// Permet de générer une méthode `to_vec` pour un enum qui retourne les noms des variantes
#[proc_macro_derive(EnumMacro)]
pub fn derive_enum_variant_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Récupère le nom de l'enum
    let enum_name = input.ident;

    // Vérifie que c'est bien un enum
    let variants = if let Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("#[derive(EnumMacro)] n'est supporté que pour les enums"); 
    };

    // Récupère les noms des variantes
    let variant_names: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // Génère la méthode `variant_names`
    let expanded = quote! {
        impl #enum_name {
            pub fn to_vec() -> Vec<&'static str> {
                vec![#(stringify!(#variant_names)),*]
            }
        }
    };

    TokenStream::from(expanded)
}
