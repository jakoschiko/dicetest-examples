use dicetest_examples::*;

// Runs all examples that print to stdout.
fn main() {
    guide::section_pseudorandomness::seed();
    guide::section_pseudorandomness::prng();
    guide::section_dice::die_once();
    guide::section_dice::die();
    guide::section_dice::fate();
}
