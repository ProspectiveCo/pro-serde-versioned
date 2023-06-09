// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │  ██████╗ ██████╗  ██████╗   Copyright (C) The Prospective Company         │
// │  ██╔══██╗██╔══██╗██╔═══██╗  All Rights Reserved - April 2022              │
// │  ██████╔╝██████╔╝██║   ██║                                                │
// │  ██╔═══╝ ██╔══██╗██║   ██║  Proprietary and confidential. Unauthorized    │
// │  ██║     ██║  ██║╚██████╔╝  copying of this file, via any medium is       │
// │  ╚═╝     ╚═╝  ╚═╝ ╚═════╝   strictly prohibited.                          │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘
use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

#[derive(Debug)]
struct VersionVariant {
    version_number: usize,
    variant_ident: syn::Ident,
    variant_ty: syn::Type,
    latest: bool,
}

#[proc_macro_derive(VersionedSerialize)]
pub fn versioned_serialize(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let version_variants = get_version_variants(&ast);
    let variant_names: Vec<_> = version_variants
        .values()
        .map(|version_variant| &version_variant.variant_ident)
        .cloned()
        .collect();

    let variant_tys: Vec<_> = version_variants
        .values()
        .map(|version_variant| version_variant.variant_ty.clone())
        .collect();

    let variant_versions: Vec<_> = version_variants
        .values()
        .map(|version_variant| version_variant.version_number)
        .collect();

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        #(
            impl #impl_generics From<#variant_tys> for #name #ty_generics #where_clause {
                fn from(value: #variant_tys) -> #name {
                    #name::#variant_names(value)
                }
            }

            // // TODO This would be handy, but will create conflicts when
            // // multiple enums use `#variant_tys`, as we do in the tests.

            // impl VersionedSerialize for #variant_tys {
            //     fn versioned_serialize<F: SerializeFormat>(&self) -> Result<F, Box<dyn std::error::Error>> {
            //         #[derive(Serialize, Deserialize)]
            //         pub struct VersionedEnvelope<T> {
            //             pub version_number: usize,
            //             pub data: T,
            //         }

            //         let envelope = VersionedEnvelope {
            //             version_number: #variant_versions,
            //             data: F::serialize_format(&self)?
            //         };

            //         F::serialize_format(envelope)
            //     }
            // }
        )*

        impl #impl_generics ::pro_serde_versioned::VersionedSerialize for #name #ty_generics #where_clause {
            type VersionedEnvelope<A: Serialize> = ::pro_serde_versioned::VersionedEnvelope<A>;
            fn to_envelope<F: ::pro_serde_versioned::SerializeFormat>(&self) -> Result<Self::VersionedEnvelope<F>, F::Error> {
                match self {
                    #(
                        #name::#variant_names(value) => {
                            Ok(::pro_serde_versioned::VersionedEnvelope {
                                version_number: #variant_versions,
                                data: F::serialize_format(&value)?
                            })
                        }
                    )*
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(VersionedDeserialize)]
pub fn versioned_deserialize(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let version_variants = get_version_variants(&ast);
    let variant_names: Vec<_> = version_variants
        .values()
        .map(|version_variant| &version_variant.variant_ident)
        .cloned()
        .collect();

    let variant_versions: Vec<_> = version_variants
        .values()
        .map(|version_variant| version_variant.version_number)
        .collect();

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::pro_serde_versioned::VersionedDeserialize for #name #ty_generics #where_clause {
            type VersionedEnvelope<'a, F: Deserialize<'a>> = ::pro_serde_versioned::VersionedEnvelope<F>;
            fn from_envelope<'a, F: ::pro_serde_versioned::DeserializeFormat + Deserialize<'a>>(
                envelope: &::pro_serde_versioned::VersionedEnvelope<F>,
            ) -> Result<Self, F::Error> {
                match envelope.version_number {
                    #(
                        #variant_versions => Ok(#name::#variant_names(
                            <F as ::pro_serde_versioned::DeserializeFormat>::deserialize_format(
                                &envelope.data
                            )
                        ?)),
                    )*
                    _ => Err(serde::de::Error::custom("Unknown version number")),
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(VersionedUpgrade)]
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

    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics ::pro_serde_versioned::VersionedUpgrade for #name #ty_generics #where_clause {
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
