//! The examples from the section `Dice`.

/// Shows a `DieOnce` example.
pub fn die_once() {
    println!("DieOnce example");

    use dicetest::prelude::dice::*;

    let xx = "xx".to_string();
    let yy = "yy".to_string();

    // This generator implements `DieOnce`.
    // It chooses one of the `String`s without cloning them.
    let xx_or_yy_die = dice::one_of_2_once(xx, yy);

    println!("{:?}", xx_or_yy_die.sample_once());
    // Output: "yy"
}

/// Shows some `Die` examples.
pub fn die() {
    println!("Die example");

    use dicetest::prelude::dice::*;

    let xx = "xx".to_string();
    let yy = "yy".to_string();

    // This generator implements `Die`.
    // It chooses one of the `String`s by cloning them.
    let xx_or_yy_die = dice::one_of_2(xx, yy);

    // This generator uses `xx_or_yy_die` three times.
    let three_xx_or_yy_die = dice::array_3(xx_or_yy_die);

    for _ in 0..4 {
        println!("{:?}", three_xx_or_yy_die.sample());
    }
    // Output:
    // ["yy", "yy", "xx"]
    // ["xx", "xx", "yy"]
    // ["xx", "yy", "yy"]
    // ["yy", "xx", "xx"]
}

/// Shows how to implement and compose generators.
#[allow(unused_variables)]
pub fn implement_and_compose() {
    use dicetest::prelude::dice::*;

    // A classic die. Generates a number between 1 and 6.
    let classic_die = dice::u8(1..=6);

    // A loaded die. Generates the number 6 more frequently.
    let loaded_die = dice::weighted_one_of_6::<u8>((1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 6));

    // Generates the result of the function.
    let die_from_fn = dice::from_fn(|_fate| 42);

    // Generates always the same `String` by cloning it.
    let foo_die = dice::just("foo".to_string());

    struct NonZeroU8(u8);
    // Generates instances of `NonZeroU8`.
    let non_zero_die = dice::u8(1..).map(NonZeroU8);

    // Generates permutations of `(0..=n)` for an arbitrary `n`.
    let permutations = dice::size(0..).flat_map(|n| {
        let vec = (0..=n).collect::<Vec<_>>();
        dice::shuffled_vec(vec)
    });
}

/// Shows how to create and use a `Fate` for pseudorandom value generation.
pub fn fate() {
    println!("Fate example");

    use dicetest::prelude::dice::*;

    // The generator is allowed to mutate this `Prng`.
    let mut prng = Prng::from_seed(42.into());
    // But the generator cannot mutate this `Limit`.
    let limit = 5.into();

    let fate = &mut Fate::new(&mut prng, limit);

    // Generates an arbitrary number of arbitrary bytes.
    let bytes_die = dice::vec(dice::u8(..), ..);

    // Nevertheless, the user of the generator can set an upper limit
    // for the number of bytes. How the upper limit is interpreted
    // varies from generator to generator. In this case, up to 5 bytes
    // are generated.
    let pseudorandom_bytes = bytes_die.roll(fate);

    println!("{:?}", pseudorandom_bytes);
    // Output: [2, 255, 176, 0, 92]
}
