extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields};

#[proc_macro_derive(UpgradableEnum, attributes(latest))]
pub fn upgradable_enum(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let mut latest_variant_ty = None;

    if let Data::Enum(data_enum) = &ast.data {
        for variant in &data_enum.variants {
            if has_latest_attribute(&variant.attrs) {
                let first = variant
                    .fields
                    .iter()
                    .nth(0)
                    .expect("No fields on latest variant");
                latest_variant_ty = Some(&first.ty);
                break;
            }
        }
    }

    let latest_variant_ty = match latest_variant_ty {
        Some(latest_variant_ty) => latest_variant_ty,
        None => panic!("No latest variant found"),
    };

    let upgrade_match_arms = generate_upgrade_match_arms(&ast);

    // gen.into()
    let gen = quote! {
        impl #name {
            pub fn upgrade_to_latest(self) -> #latest_variant_ty {
                match self {
                    #(#upgrade_match_arms)*
                }
            }
        }
    };

    gen.into()
}

fn has_latest_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("latest"))
}

#[derive(Debug)]
struct VersionVariant {
    version_number: usize,
    variant_ident: syn::Ident,
    variant_ty: syn::Type,
    latest: bool,
}

fn generate_upgrade_match_arms(ast: &DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let mut match_arms = Vec::new();
    let mut version_variants: HashMap<usize, VersionVariant> = HashMap::new();

    if let Data::Enum(data_enum) = &ast.data {
        for variant in &data_enum.variants {
            version_variants.insert(
                variant
                    .ident
                    .to_string()
                    .replace("V", "")
                    .parse::<usize>()
                    .expect("Invalid version number"),
                VersionVariant {
                    version_number: variant
                        .ident
                        .to_string()
                        .replace("V", "")
                        .parse::<usize>()
                        .expect("Invalid version number"),
                    variant_ident: variant.ident.clone(),
                    variant_ty: {
                        if variant.fields.len() != 1 {
                            panic!("Only single-field variants are supported");
                        }
                        if let Fields::Unnamed(fields_unnamed) = &variant.fields {
                            fields_unnamed.unnamed[0].ty.clone()
                        } else {
                            panic!("Only unnamed fields are supported");
                        }
                    },
                    latest: has_latest_attribute(&variant.attrs),
                },
            );
        }
    }

    for version_variant in version_variants.values() {
        let version_number = version_variant.version_number;
        let variant_ident = &version_variant.variant_ident;

        if !version_variant.latest {
            let next_variant = version_variants
                .get(&(version_number + 1))
                .expect("No variant for next version");

            let next_variant_ident = &next_variant.variant_ident;
            let next_variant_ty = &next_variant.variant_ty;
            match_arms.push(quote! {
                #name::#variant_ident(value) => {
                    let upgraded: #next_variant_ty = value.upgrade();
                    #name::#next_variant_ident(upgraded).upgrade_to_latest()
                },
            });
        } else {
            match_arms.push(quote! {
                #name::#variant_ident(value) => value,
            });
        }
    }

    // if let Data::Enum(data_enum) = &ast.data {
    //     for variant in &data_enum.variants {
    //         let variant_ident = &variant.ident;
    //         let fields = if let Fields::Unnamed(fields_unnamed) = &variant.fields {
    //             &fields_unnamed.unnamed
    //         } else {
    //             panic!("Only unnamed fields are supported");
    //         };

    //         if fields.len() != 1 {
    //             panic!("Only single-field variants are supported");
    //         }

    //         let version_number = variant_ident
    //             .to_string()
    //             .replace("V", "")
    //             .parse::<usize>()
    //             .expect("Invalid version number");

    //         let next_variant = format!("V{}", version_number + 1);

    //         if !has_latest_attribute(&variant.attrs) {
    //             let field_ty = &fields[0].ty;
    //             match_arms.push(quote! {
    //                 #name::#variant_ident(value) => {
    //                     let upgraded: #field_ty = value.upgrade();
    //                     #name::#next_variant(upgraded).upgrade_to_latest()
    //                 },
    //             });
    //         } else {
    //             let variant_ident = &variant.ident;
    //             match_arms.push(quote! {
    //                 #name::#variant_ident(value) => value,
    //             });
    //         }
    //     }
    // }

    match_arms
}
