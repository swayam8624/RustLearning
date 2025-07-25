//!
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

pub struct StrSplit {}

impl StrSplit {
    pub fn new(haystack: &str, delimeter: &str) -> Self {
        // This function is a placeholder for the actual implementation
        // It should return an instance of StrSplit that can be used to split the haystack by the delimeter
    }
}

impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {}
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letter = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
    {}
}
