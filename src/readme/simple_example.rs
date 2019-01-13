//! A simple example that shows how to Dicetest.

/// A sort function that contains a bug.
pub fn bubble_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();

    for _ in 0..len {
        for j in 1..len - 1 {
            let jpp = j + 1;
            if slice[j] > slice[jpp] {
                slice.swap(j, jpp);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dicetest::prelude::tests::*;

    // This test will fail!
    //
    // Output:
    // The test failed after 36 passes.
    //
    // # Config
    // - seed: 795359663177100823
    // - start limit: 0
    // - end limit: 100
    // - passes: 1000
    //
    // # Counterexample
    // - run code: "ABIDje/+CYVkmmCVTwKJ2go6VrzZWMjO2Bqc9m3b3h0DAAAAAAAAAA=="
    // - limit: 3
    // - hints:
    //     - unsorted: [255, 252, 131]
    //     -   sorted: [255, 131, 252]
    // - error: assertion failed: is_sorted
    #[ignore]
    #[test]
    fn result_of_bubble_sort_is_sorted() {
        dicetest!(|fate| {
            let mut v = dice::vec(dice::u8(..), ..).roll(fate);
            hint!("unsorted: {:?}", v);

            bubble_sort(&mut v);
            hint!("  sorted: {:?}", v);

            let is_sorted = v.windows(2).all(|w| w[0] <= w[1]);
            assert!(is_sorted);
        })
    }
}
