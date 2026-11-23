//! To test the vscode extension

/// A greeting, to show that the crate is built and tested; replace it with the crate's own code.
pub fn greeting() -> &'static str {
    "Hello from bruce-test-app-core"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greeting(), "Hello from bruce-test-app-core");
    }
}
