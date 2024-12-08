use std::env;
use std::path::PathBuf;

use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

#[proc_macro]
pub fn all(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    all_impl(ts.into()).unwrap_or_else(syn::Error::into_compile_error).into()
}

mod kw {
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
    syn::custom_keyword!(jq);
}

struct Input {
    days: Vec<Day>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut days = Vec::new();
        while !input.is_empty() {
            days.push(input.parse::<Day>()?);
        }
        Ok(Self { days })
    }
}

struct Day {
    day_token:     kw::day,
    day_number:    syn::LitInt,
    _braces_token: syn::token::Brace,
    parts:         Vec<Part>,
}

impl Day {
    fn mod_ident(&self) -> syn::Ident {
        syn::Ident::new(&format!("d{}", &self.day_number), self.day_token.span)
    }
}

impl Parse for Day {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let day_token = input.parse()?;
        let day_number = input.parse()?;

        let inner;
        let braces_token = syn::braced!(inner in input);

        let mut parts = Vec::new();
        while !inner.is_empty() {
            parts.push(inner.parse::<Part>()?);
        }
        Ok(Self { day_token, day_number, _braces_token: braces_token, parts })
    }
}

struct Part {
    part_token:    kw::part,
    part_number:   syn::LitInt,
    _braces_token: syn::token::Brace,
    solutions:     Punctuated<Solution, syn::Token![,]>,
}

impl Parse for Part {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let part_token = input.parse()?;
        let part_number = input.parse()?;

        let inner;
        let braces_token = syn::braced!(inner in input);

        let solutions = Punctuated::parse_terminated(&inner)?;

        Ok(Self { part_token, part_number, _braces_token: braces_token, solutions })
    }
}

struct Solution {
    name:   syn::LitStr,
    arrow:  syn::Token![=>],
    target: SolutionTarget,
}

impl Parse for Solution {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { name: input.parse()?, arrow: input.parse()?, target: input.parse()? })
    }
}

enum SolutionTarget {
    Jq {
        _jq_token:       kw::jq,
        _brackets_token: syn::token::Bracket,
        filter_ident:    syn::LitStr,
    },
    Rust {
        fn_ident: syn::Ident,
    },
}

impl SolutionTarget {
    fn fn_expr(&self, day: &Day, all_module_path: TokenStream) -> syn::Result<TokenStream> {
        Ok(match self {
            Self::Jq { filter_ident, .. } => {
                let day_number = &day.day_number;
                let file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join(format!("src/all/d{}.jq", day_number.base10_parse::<u32>()?));
                let file_path =
                    file_path.to_str().expect("build path must not contain non-unicode characters");
                quote! {{
                    let mut program =
                        jq_rs::compile(concat!(include_str!(#file_path), "\n", #filter_ident))
                            .map_err(|err| anyhow::anyhow!("compile d{}.jq: {err}", #day_number))?;
                    move |data: JsonString| -> String {
                        program.run(data.0.as_str()).expect("jq program error")
                    }
                }}
            }
            Self::Rust { fn_ident } => {
                let day_ident = day.mod_ident();
                quote!(#all_module_path #day_ident::#fn_ident)
            }
        })
    }
}

impl Parse for SolutionTarget {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lh = input.lookahead1();
        if lh.peek(kw::jq) {
            let jq_token = input.parse()?;
            let inner;
            let brackets_token = syn::bracketed!(inner in input);
            let filter_ident = inner.parse()?;
            Ok(Self::Jq { _jq_token: jq_token, _brackets_token: brackets_token, filter_ident })
        } else if lh.peek(syn::Ident) {
            Ok(Self::Rust { fn_ident: input.parse()? })
        } else {
            Err(lh.error())
        }
    }
}

fn all_impl(ts: TokenStream) -> syn::Result<TokenStream> {
    let Input { days } = syn::parse2(ts)?;

    let mods = days.iter().map(|day| {
        let day_ident = day.mod_ident();
        quote_spanned! { day.day_token.span =>
            pub mod #day_ident;
        }
    });

    let day_run_arms = days
        .iter()
        .map(|day| {
            let day_int = &day.day_number;

            let part_arms = day
                .parts
                .iter()
                .map(|part| {
                    let part_int = &part.part_number;
                    let solution_arms = part
                        .solutions
                        .iter()
                        .map(|solution| {
                            let solution_name = &solution.name;
                            let solution_fn_expr = solution.target.fn_expr(day, quote!())?;
                            Ok(quote_spanned! { solution.arrow.span() =>
                                #solution_name => call(#solution_fn_expr, &input),
                            })
                        })
                        .collect::<syn::Result<Vec<_>>>()?;

                    let available_solution_names = part
                        .solutions
                        .iter()
                        .map(|solution| format!("\"{}\"", solution.name.value()))
                        .join(", ");

                    Ok(quote_spanned! { part.part_token.span =>
                        #part_int => {
                            let input = load_input(args.mode, #day_int)?;
                            let output = match variant {
                                #(#solution_arms)*
                                variant => anyhow::bail!(
                                    "Unknown solution name {variant}. Available solutions: {}",
                                    #available_solution_names,
                                )
                            };
                            println!("Output: {output}");
                            Ok(())
                        },
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;

            Ok(quote_spanned! { day.day_token.span =>
                #day_int => match args.part {
                    #(#part_arms)*
                    part => anyhow::bail!("No solutions for part {part}"),
                },
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let bench_groups = days
        .iter()
        .flat_map(|day| {
            let day_number = &day.day_number;

            day.parts.iter().map(move |part| {
                let rust_group_name =
                    format!("Day {} Part {} Rust", day.day_number, part.part_number);
                let jq_group_name = format!("Day {} Part {} JQ", day.day_number, part.part_number);

                let functions = part
                    .solutions
                    .iter()
                    .map(|soln| {
                        let soln_name = &soln.name;
                        let solution_fn_expr = soln.target.fn_expr(day, quote!(aoc2024::all::))?;
                        Ok((
                            matches!(soln.target, SolutionTarget::Jq { .. }),
                            quote_spanned! { soln.arrow.span() =>
                                {
                                    let mut f = #solution_fn_expr;
                                    group.bench_function(#soln_name, move |b| {
                                        call_benched(b, #day_number, &mut f);
                                    });
                                }
                            },
                        ))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;

                let rust_functions =
                    functions.iter().filter_map(|&(is_jq, ref ts)| (!is_jq).then_some(ts));
                let jq_functions =
                    functions.iter().filter_map(|&(is_jq, ref ts)| is_jq.then_some(ts));

                Ok(quote! {
                    {
                        let mut group = $criterion_manager.benchmark_group(#rust_group_name);
                        #(#rust_functions)*
                        group.finish();
                    }
                    {
                        let mut group = $criterion_manager.benchmark_group(#jq_group_name);
                        #(#jq_functions)*
                        group.finish();
                    }
                })
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let iai_fns = days
        .iter()
        .map(|day| {
            let day_number = day.day_number.clone().base10_parse::<u32>()?;

            day.parts
                .iter()
                .flat_map(move |part| {
                    part.solutions
                        .iter()
                        .filter_map(move |soln| match &soln.target {
                            SolutionTarget::Jq { .. } => None,
                            target @ SolutionTarget::Rust { fn_ident } => {
                                Some((soln.name.clone(), target, fn_ident.clone()))
                            }
                        })
                        .map(move |(soln_name, soln_target, fn_ident)| {
                            let fn_ident = syn::Ident::new(
                                &format!("day_{day_number}_part_{}_{fn_ident}", part.part_number),
                                soln_name.span(),
                            );
                            let solution_fn_expr =
                                soln_target.fn_expr(day, quote!(aoc2024::all::))?;
                            let file_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                                .join(format!("input/d{day_number}.private.input.txt"));
                            let file_path = file_path
                                .to_str()
                                .expect("build path must not contain non-unicode characters");
                            let fn_def = quote_spanned! { soln_name.span() =>
                                fn #fn_ident() {
                                    let input = include_str!(#file_path);
                                    let parsed = iai::black_box(aoc2024::Parse::parse(input));
                                    iai::black_box(#solution_fn_expr(iai::black_box(parsed)));
                                }
                            };
                            Ok((fn_ident, fn_def))
                        })
                })
                .collect::<syn::Result<Vec<_>>>()
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let iai_fn_idents =
        iai_fns.iter().flatten().map(|(iai_fn_ident, _)| iai_fn_ident).collect::<Vec<_>>();
    let iai_fn_defs =
        iai_fns.iter().flatten().map(|(_, iai_fn_def)| iai_fn_def).collect::<Vec<_>>();

    let output = quote! {
        #(#mods)*

        pub fn run(args: Args) -> anyhow::Result<()> {
            let variant = args.variant.as_str();
            match args.day {
                #(#day_run_arms)*
                day => anyhow::bail!("No solutions for day {day}"),
            }
        }

        #[macro_export]
        macro_rules! bench {
            ($criterion_manager:expr) => {
                #(#bench_groups)*
            }
        }

        #[macro_export]
        macro_rules! iai {
            () => {
                #(#iai_fn_defs)*
                iai::main! {
                    #(#iai_fn_idents,)*
                }
            }
        }
    };
    Ok(output)
}
