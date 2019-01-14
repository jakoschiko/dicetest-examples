//! The examples from the section `Stats`.

#[cfg(test)]
mod tests {
    use dicetest::prelude::tests::*;

    // Run this test with `DICETEST_LOG_CONDITION=always DICETEST_STATS_ENABLED=true`.
    //
    // Output:
    // The test withstood 1000 passes.
    //
    // # Config
    // - seed: 10582207530707092664
    // - start limit: 0
    // - end limit: 100
    // - passes: 1000
    //
    // # Stats
    // - branch:
    //         - 26% (265): if with y = 1
    //         - 25% (252): if with y = 3
    //         - 24% (243): else
    //         - 24% (240): if with y = 2
    // - x:
    //         - 36% (368): 1
    //         - 23% (237): 5
    //         - 20% (207): 2
    //         - 9% (99): 4
    //         - 8% (89): 3
    #[test]
    fn test_foo() {
        dicetest!(|fate| {
            let x = dice::u8(1..=5).roll(fate);
            stat_debug!(x);

            let y = dice::u8(1..=3).roll(fate);
            if y != x {
                stat!("branch", "if with y = {}", y)
            } else {
                stat!("branch", "else");
            }
        })
    }
}
