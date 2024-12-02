use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("codegen.rs"));
    let mut file: BufWriter<File> = BufWriter::new(File::create(&path).unwrap());
    let (list, bench): (Vec<_>, Vec<_>) = [
        5, 7, 10, 12, 14, 16, 20, 28, 35, 50, 75, 100, 200, 500, 1000, 2000, 5000, 10000,
    ]
    .into_iter()
    .map(gen_bench)
    .unzip();
    _ = write!(
        file,
        "{}",
        quote! {
            const BENCHS: &[(usize, fn())] = &[
                #(#list),*
            ];
            #(#bench)*
        }
    );
}

fn gen_bench(n: usize) -> (TokenStream, TokenStream) {
    let test_strings = generate_random_strings(n);
    let mod_name = Ident::new(&format!("bench_{n}"), Span::call_site());
    let match_str_arms: Vec<_> = test_strings
        .iter()
        .enumerate()
        .map(|(n, s)| quote! {#s => Some(#n)})
        .collect();
    let match_hash_arms: Vec<_> = test_strings
        .iter()
        .enumerate()
        .map(|(n, s)| quote! {hashmatch::hash_arm!(#s) => Some(#n)})
        .collect();
    let lookup_phf_arms: Vec<_> = test_strings
        .iter()
        .enumerate()
        .map(|(n, s)| quote! {#s => #n})
        .collect();
    let lookup_lazy_arms: Vec<_> = test_strings
        .iter()
        .enumerate()
        .map(|(n, s)| quote! {map.insert(#s, #n);})
        .collect();
    (
        quote! {
            (#n, #mod_name::bench)
        },
        quote! {
            mod #mod_name{
                const TEST_STRINGS: [&str; #n] = [
                    #(#test_strings),*
                ];
                #[inline]
                fn match_str(s: &str) -> Option<usize> {
                    #[deny(unreachable_patterns)]
                    match s {
                        #(#match_str_arms),*,
                        _ => None,
                    }
                }
                #[inline]
                fn match_hash(s: &str) -> Option<usize> {
                    #[deny(unreachable_patterns)]
                    match hashmatch::hash_str(s) {
                        #(#match_hash_arms),*,
                        _ => None,
                    }
                }
                #[inline]
                fn lookup_phf(s: &str) -> Option<usize> {
                    const STRING_MAP: phf::Map<&str, usize> = phf::phf_map! {
                        #(#lookup_phf_arms),*
                    };
                    STRING_MAP.get(s).copied()
                }
                #[inline]
                fn lookup_lazy(s: &str) -> Option<usize> {
                    static STRING_MAP: std::sync::LazyLock<foldhash::HashMap<&str, usize>> =
                        std::sync::LazyLock::new(|| {
                            use foldhash::HashMapExt;
                            let mut map = foldhash::HashMap::with_capacity(#n);
                            #(#lookup_lazy_arms)*
                            map
                        });
                    STRING_MAP.get(s).copied()
                }
                #[inline]
                fn bench_match_str(c: &mut criterion::Criterion) {
                    c.bench_function(concat!("match_str_", #n), |b| {
                        b.iter(|| {
                            for s in TEST_STRINGS.iter() {
                                criterion::black_box(match_str(s));
                            }
                        })
                    });
                }
                #[inline]
                fn bench_match_hash(c: &mut criterion::Criterion) {
                    c.bench_function(concat!("match_hash_", #n), |b| {
                        b.iter(|| {
                            for s in TEST_STRINGS.iter() {
                                criterion::black_box(match_hash(s));
                            }
                        })
                    });
                }
                #[inline]
                fn bench_lookup_phf(c: &mut criterion::Criterion) {
                    c.bench_function(concat!("lookup_phf_", #n), |b| {
                        b.iter(|| {
                            for s in TEST_STRINGS.iter() {
                                criterion::black_box(lookup_phf(s));
                            }
                        })
                    });
                }
                #[inline]
                fn bench_lookup_lazy(c: &mut criterion::Criterion) {
                    c.bench_function(concat!("lookup_lazy_", #n), |b| {
                        b.iter(|| {
                            for s in TEST_STRINGS.iter() {
                                criterion::black_box(lookup_lazy(s));
                            }
                        })
                    });
                }
                criterion::criterion_group!(
                    bench,
                    bench_match_str,
                    bench_match_hash,
                    bench_lookup_phf,
                    bench_lookup_lazy
                );
            }
        },
    )
}

fn generate_random_strings(n: usize) -> Vec<String> {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut strings = Vec::with_capacity(n);
    for _ in 0..n {
        let len = rng.gen_range(6..=10);
        let rand_string: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphabetic())
            .map(char::from)
            .take(len)
            .collect();
        strings.push(rand_string.to_lowercase());
    }
    strings
}
