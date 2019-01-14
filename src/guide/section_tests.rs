//! The examples from the section `Tests`.

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use dicetest::prelude::tests::*;

    #[test]
    fn test_foo() {
        // Runs your test with default configuration.
        dicetest!(|fate| {
            // Your test.
        });
    }

    #[test]
    fn test_bar() {
        // Runs your test with custom configuration.
        dicetest!(Config::default().with_passes(10000), |fate| {
            // Your test.
        });
    }
}
