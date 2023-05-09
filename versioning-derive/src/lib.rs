extern crate proc_macro;

use std::collections::{HashMap, HashSet};

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Ident};

#[derive(Debug, Hash, PartialEq, Eq)]
enum VersionedWrapperFormat {
    Json,
    MsgPack,
}

#[derive(Debug)]
struct VersionVariant {
    version_number: usize,
    variant_ident: syn::Ident,
    variant_ty: syn::Type,
    latest: bool,
}

fn get_format_attr(attrs: &[Attribute]) -> HashSet<VersionedWrapperFormat> {
    let mut formats = HashSet::new();
    attrs.iter().for_each(|attr| {
        if attr.meta.path().is_ident("versioned_wrapper") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("formats") {
                    // println!("VALUE: {:?}", meta.path);
                    meta.parse_nested_meta(|meta| {
                        // let value: Ident = meta.value()?.parse()?;
                        if meta.path.is_ident("json") {
                            formats.insert(VersionedWrapperFormat::Json);
                            Ok(())
                        } else if meta.path.is_ident("msgpack") {
                            formats.insert(VersionedWrapperFormat::MsgPack);
                            Ok(())
                        } else {
                            return Err(meta
                                .error("Expected `format = \"json\"` or `format = \"msgpack\"`"));
                        }
                    })
                    // Ok(())
                } else {
                    return Err(
                        meta.error("Expected `format = \"json\"` or `format = \"msgpack\"`")
                    );
                }
            })
            .expect("Invalid format");
        }
    });
    formats
}

#[proc_macro_derive(VersionedWrapper, attributes(versioned_wrapper))]
pub fn versioned_serde(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let format = get_format_attr(&ast.attrs);
    if format.len() < 1 {
        panic!("Expected `#[versioned_wrapper(format = \"json\")]` or `#[versioned_wrapper(format = \"msgpack\")]`");
    }

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

    let instances: Vec<_> = format
        .iter()
        .map(|format| match format {
            VersionedWrapperFormat::Json => {
                generate_json_impl(name, &variant_names, &variant_versions)
            }
            VersionedWrapperFormat::MsgPack => {
                generate_msgpack_impl(name, &variant_names, &variant_versions)
            }
        })
        .collect();

    quote!(
        #(#instances)*
    )
    .into()
}

fn generate_msgpack_impl(
    name: &Ident,
    variant_names: &[Ident],
    variant_versions: &[usize],
) -> proc_macro2::TokenStream {
    quote! {
        impl<'a> VersionedWrapperDe<'a, MsgPackBytes<'a>> for #name {
            fn from_versioned_envelope(
                envelope: VersionedEnvelope<MsgPackBytes<'a>>,
            ) -> Result<Self, Box<dyn std::error::Error>> {
                match envelope.version_number.0 {
                    #(
                        #variant_versions => Ok(#name::#variant_names(rmp_serde::from_slice(&envelope.data.0)?)),
                    )*
                    _ => Err("Unknown version".into()),
                }
            }
        }

        impl<'a> VersionedWrapperSer<'a, MsgPackBytes<'a>> for #name {
            fn to_versioned_envelope(
                &self,
            ) -> Result<VersionedEnvelope<MsgPackBytes<'a>>, Box<dyn std::error::Error>> {
                match self {
                    #(
                        #name::#variant_names(value) => {
                            let mut struct_ser = rmp_serde::Serializer::new(Vec::new());
                            value.serialize(&mut struct_ser)?;
                            Ok(VersionedEnvelope {
                                version_number: #variant_versions.into(),
                                data: MsgPackBytes(Cow::Owned(struct_ser.into_inner().to_owned())),
                            })
                        }
                    )*
                }
            }
        }
    }
}

fn generate_json_impl(
    name: &Ident,
    variant_names: &[Ident],
    variant_versions: &[usize],
) -> proc_macro2::TokenStream {
    quote! {
        impl VersionedWrapperSer<'_, serde_json::Value> for #name {
            fn to_versioned_envelope(
                &self,
            ) -> Result<VersionedEnvelope<serde_json::Value>, Box<dyn std::error::Error>> {
                match &self {
                    #(
                        #name::#variant_names(value) => Ok(VersionedEnvelope {
                            version_number: #variant_versions.into(),
                            data: serde_json::to_value(value)?,
                        }),
                    )*
                }
            }
        }

        impl VersionedWrapperDe<'_, serde_json::Value> for #name {
            fn from_versioned_envelope(
                envelope: VersionedEnvelope<serde_json::Value>,
            ) -> Result<Self, Box<dyn std::error::Error>> {
                match envelope.version_number.0 {
                    #(
                        #variant_versions => Ok(#name::#variant_names(serde_json::from_slice(
                            &envelope.data.to_string().as_bytes(),
                        )?)),
                    )*
                    _ => Err("Unknown version".into()),
                }
            }
        }
    }
}

#[proc_macro_derive(UpgradableEnum)]
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
