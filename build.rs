use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use itertools::Itertools;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(format!("codegen.rs"));
    let mut file: BufWriter<File> = BufWriter::new(File::create(&path).unwrap());
    for n in [
        5, 10, 15, 20, 35, 50, 75, 100, 200, 500, 1000, 2000, 5000, 10000,
    ] {
        gen_bench(n, &mut file);
    }
}

fn generate_random_strings(n: usize) -> Vec<String> {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut strings = Vec::with_capacity(n);

    for _ in 0..n {
        let len = rng.gen_range(6..=10); // 生成6到10之间的随机长度
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

#[inline]
fn hash_one<T: std::hash::Hash + ?Sized>(t: &T) -> u64 {
    const HASHER: foldhash::fast::FixedState = foldhash::fast::FixedState::with_seed(41);
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = HASHER.build_hasher();
    t.hash(&mut hasher);
    hasher.finish()
}

fn gen_bench(n: usize, file: &mut BufWriter<File>) {
    _ = write!(file, "mod bench_{n}{{\n");
    let test_strings = generate_random_strings(n);
    // TEST_STRINGS
    _ = write!(
        file,
        "const TEST_STRINGS: [&str; {n}] = {:?};\n",
        test_strings
    );
    // match_hash
    _ = write!(
        file,
        "fn match_hash(s: &str) -> Option<usize> {{
#[inline]
fn hash_one<T: std::hash::Hash + ?Sized>(t: &T) -> u64 {{
    const HASHER: foldhash::fast::FixedState = foldhash::fast::FixedState::with_seed(41);
    use std::hash::{{BuildHasher, Hasher}};
    let mut hasher = HASHER.build_hasher();
    t.hash(&mut hasher);
    hasher.finish()
}}
#[deny(unreachable_patterns)]
match hash_one(s) {{
    {},
    _ => None,
}}
}}\n",
        test_strings
            .iter()
            .enumerate()
            .map(|(n, s)| format!("{} => Some({n})", hash_one(&s)))
            .join(",\n        ")
    );
    // match_str
    _ = write!(
        file,
        "fn match_str(s: &str) -> Option<usize> {{
match s {{
    {},
    _ => None,
}}
}}\n",
        test_strings
            .iter()
            .enumerate()
            .map(|(n, s)| format!("\"{s}\" => Some({n})"))
            .join(",\n        ")
    );
    // lookup_phf
    write!(
        file,
        "fn lookup_phf(s: &str) -> Option<usize> {{
const STRING_MAP: phf::Map<&str, usize> = phf::phf_map!{{
    {}
}};
STRING_MAP.get(s).copied()
}}\n",
        test_strings
            .iter()
            .enumerate()
            .map(|(n, s)| format!("\"{s}\" => {n}"))
            .join(",\n        ")
    )
    .unwrap();
    // lookup_lazy
    _ = write!(
    file,
"fn lookup_lazy(s: &str) -> Option<usize> {{
static STRING_MAP: std::sync::LazyLock<foldhash::HashMap<&str,usize>> = std::sync::LazyLock::new(||{{
    use foldhash::HashMapExt;
    let mut map = foldhash::HashMap::with_capacity({n});
    {}
    map
}});
STRING_MAP.get(s).copied()
}}\n",
test_strings
    .iter()
    .enumerate()
    .map(|(n, s)| format!("map.insert(\"{s}\", {n});"))
    .join("\n        ")
);
    _ = write!(
        file,
        "use criterion::{{black_box, criterion_group, Criterion}};
    
    fn bench_match_str(c: &mut Criterion) {{
    c.bench_function(\"match_str_{n}\", |b| {{
        b.iter(|| {{
            for s in TEST_STRINGS.iter() {{
                black_box(match_str(s));
            }}
        }})
    }});
    }}
    
    fn bench_match_hash(c: &mut Criterion) {{
    c.bench_function(\"match_hash_{n}\", |b| {{
        b.iter(|| {{
            for s in TEST_STRINGS.iter() {{
                black_box(match_hash(s));
            }}
        }})
    }});
    }}
    
    fn bench_lookup_phf(c: &mut Criterion) {{
    c.bench_function(\"lookup_phf_{n}\", |b| {{
        b.iter(|| {{
            for s in TEST_STRINGS.iter() {{
                black_box(lookup_phf(s));
            }}
        }})
    }});
    }}
    
    fn bench_lookup_lazy(c: &mut Criterion) {{
    c.bench_function(\"lookup_lazy_{n}\", |b| {{
        b.iter(|| {{
            for s in TEST_STRINGS.iter() {{
                black_box(lookup_lazy(s));
            }}
        }})
    }});
    }}
    
    criterion_group!(
    benches,
    bench_match_str,
    bench_match_hash,
    bench_lookup_phf,
    bench_lookup_lazy
    );
    
    #[test]
    fn valid() {{
    for s in TEST_STRINGS.iter() {{
        let res_match_str = match_str(s);
        let res_match_hash = match_hash(s);
        let res_lookup_phf = lookup_phf(s);
        let res_lookup_lazy = lookup_lazy(s);
        assert_eq!(res_match_str, res_match_hash);
        assert_eq!(res_match_hash, res_lookup_phf);
        assert_eq!(res_lookup_phf, res_lookup_lazy);
    }}
    }}"
    );
    _ = write!(file, "}}\n");
}
