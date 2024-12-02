//! Using hash-match for efficiency when #arm larger than 40.
//! ```
//! use hashmatch::{hash_arm, hash_str};
//! // to avoid hash conflict
//! #[deny(unreachable_patterns)]
//! let res = match hash_str("ABC") {
//!     hash_arm!("ABC") => 1,
//!     hash_arm!("AAA") | hash_arm!("BBB") => 2,
//!     _ => 3,
//! };
//! assert_eq!(res, 1);
//! ```

pub use hashmatch_macro::hash_arm;

const HASHER: foldhash::fast::FixedState = foldhash::fast::FixedState::with_seed(41);
#[inline]
pub fn hash_str<S: AsRef<str>>(t: S) -> u64 {
    use std::hash::{BuildHasher, Hash, Hasher};
    let mut hasher = HASHER.build_hasher();
    t.as_ref().hash(&mut hasher);
    hasher.finish()
}

#[test]
fn demo() {
    // to avoid hash conflict
    #[deny(unreachable_patterns)]
    let res = match hash_str("ABC") {
        hash_arm!("ABC") => 1,
        hash_arm!("AAA") | hash_arm!("BBB") => 2,
        _ => 3,
    };
    assert_eq!(res, 1);
}
