extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use serde::{Serialize, Deserialize};
use syn::{Data, DeriveInput, Fields};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct VersionedEnvelope<'a> {
    version_number: usize,
    data: &'a[u8]
}

#[proc_macro_derive(UpgradableEnum, attributes(latest))]
pub fn upgradable_enum(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let mut latest_variant_ty = None;
    let version_variants = get_version_variants(&ast);

    for variant in version_variants.values() {
        if variant.latest {
            latest_variant_ty = Some(variant.variant_ty.clone());
            break;
        }
    }

    let latest_variant_ty = match latest_variant_ty {
        Some(latest_variant_ty) => latest_variant_ty,
        None => panic!("No latest variant found"),
    };

    let upgrade_match_arms = generate_upgrade_match_arms(&ast, version_variants);

    // gen.into()
    let gen = quote! {
        impl UpgradableEnum for #name {
            type Latest = #latest_variant_ty;
            fn upgrade_to_latest(self) -> Self::Latest {
                match self {
                    #(#upgrade_match_arms)*
                }
            }
        }
    };

    gen.into()
}

#[derive(Debug)]
struct VersionVariant {
    version_number: usize,
    variant_ident: syn::Ident,
    variant_ty: syn::Type,
    latest: bool,
}

fn generate_upgrade_match_arms(
    ast: &DeriveInput,
    version_variants: HashMap<usize, VersionVariant>,
) -> Vec<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let mut match_arms = Vec::new();

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

    match_arms
}

fn get_version_variants(ast: &DeriveInput) -> HashMap<usize, VersionVariant> {
    let mut version_variants: HashMap<usize, VersionVariant> = HashMap::new();

    let mut max = 0;

    if let Data::Enum(data_enum) = &ast.data {
        for variant in &data_enum.variants {
            let version_number = variant
                .ident
                .to_string()
                .replace("V", "")
                .parse::<usize>()
                .expect("Invalid version number");
            max = std::cmp::max(max, version_number);
            version_variants.insert(
                variant
                    .ident
                    .to_string()
                    .replace("V", "")
                    .parse::<usize>()
                    .expect("Invalid version number"),
                VersionVariant {
                    version_number,
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
                    latest: false,
                },
            );
        }
    }
    {
        let latest_version = version_variants.get_mut(&max).expect("No latest version");
        latest_version.latest = true;
    }

    version_variants
}
