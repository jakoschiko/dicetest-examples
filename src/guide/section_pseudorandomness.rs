//! The examples from the section `Pseudorandomness`.

/// Shows how to create a `Seed`.
#[allow(unused_variables)]
pub fn seed() {
    println!("Seed example");

    use dicetest::prand::Seed;

    println!("{:?}", Seed(42));
    // Output: Seed(42)

    println!("{:?}", Seed::random());
    // Output: Seed(8019292413750407764)
}

/// Shows how to create and use a `Prng`.
pub fn prng() {
    println!("Prng example");

    use dicetest::prand::*;

    let mut prng = Prng::from_seed(Seed(42));

    for _ in 0..4 {
        println!("{:?}", prng.next_number());
    }
    // Output:
    // 16628028624323922065
    // 3476588890713931039
    // 59688652182557721
    // 8649295813736445329
}
