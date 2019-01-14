//! The examples from the section `Hints`.

#[cfg(test)]
mod tests {
    use dicetest::prelude::tests::*;

    // This test will fail!
    //
    // Output:
    // The test failed after 0 passes.
    //
    // # Config
    // - seed: 1002476580450337062
    // - start limit: 0
    // - end limit: 100
    // - passes: 1000
    //
    // # Counterexample
    // - run code: "wVL7vPmPphzT0HjGwTILuoksv/gH1iB0H5qfgApmKoIAAAAAAAAAAA=="
    // - limit: 0
    // - hints:
    //         - x = 5
    //         - took branch if with y = 2
    // - error: assertion failed: `(left == right)`
    //   left: `3`,
    //  right: `2`
    #[ignore]
    #[test]
    fn test_foo() {
        dicetest!(|fate| {
            let x = dice::u8(1..=5).roll(fate);
            hint_debug!(x);

            let y = dice::u8(1..=3).roll(fate);
            if y != x {
                hint!("took branch if with y = {}", y);

                assert_eq!(3, y);
            } else {
                hint!("took branch else");
            }
        })
    }
}
