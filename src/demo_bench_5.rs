const TEST_STRINGS: [&str; 5] = ["xrxeclxu", "vukddz", "qwhkdyjog", "dpesutax", "tqgzzfcblp"];
fn match_str(s: &str) -> Option<usize> {
    match s {
        "xrxeclxu" => Some(0),
        "vukddz" => Some(1),
        "qwhkdyjog" => Some(2),
        "dpesutax" => Some(3),
        "tqgzzfcblp" => Some(4),
        _ => None,
    }
}
fn match_hash(s: &str) -> Option<usize> {
    #[inline]
    fn hash_one<T: std::hash::Hash + ?Sized>(t: &T) -> u64 {
        const HASHER: foldhash::fast::FixedState = foldhash::fast::FixedState::with_seed(41);
        use std::hash::{BuildHasher, Hasher};
        let mut hasher = HASHER.build_hasher();
        t.hash(&mut hasher);
        hasher.finish()
    }
    #[deny(unreachable_patterns)]
    match hash_one(s) {
        10609036174714360756 => Some(0),
        16318186236859975462 => Some(1),
        15192521078157042894 => Some(2),
        7132384987385148670 => Some(3),
        8589146856602784662 => Some(4),
        _ => None,
    }
}
fn lookup_phf(s: &str) -> Option<usize> {
    const STRING_MAP: phf::Map<&str, usize> = phf::phf_map! {
        "xrxeclxu" => 0,
        "vukddz" => 1,
        "qwhkdyjog" => 2,
        "dpesutax" => 3,
        "tqgzzfcblp" => 4
    };
    STRING_MAP.get(s).copied()
}
fn lookup_lazy(s: &str) -> Option<usize> {
    static STRING_MAP: std::sync::LazyLock<foldhash::HashMap<&str, usize>> =
        std::sync::LazyLock::new(|| {
            use foldhash::HashMapExt;
            let mut map = foldhash::HashMap::with_capacity(5);
            map.insert("xrxeclxu", 0);
            map.insert("vukddz", 1);
            map.insert("qwhkdyjog", 2);
            map.insert("dpesutax", 3);
            map.insert("tqgzzfcblp", 4);
            map
        });
    STRING_MAP.get(s).copied()
}
