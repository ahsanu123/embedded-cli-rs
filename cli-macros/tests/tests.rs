#[cfg(test)]
mod test_macro {
    #[test]
    pub fn pass() {
        macrotest::expand("tests/expand/*.rs");
    }
}
