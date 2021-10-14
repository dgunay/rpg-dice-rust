extern crate dicelib;

use dicelib::solve_dice_expression;

const TEST_SEED: u64 = 42;

#[test]
fn solve_dice_expression_can_do_basic_math() {
    assert_eq!(
        4,
        solve_dice_expression(&String::from("2 + 2"), None).unwrap()
    );
}

#[test]
fn seeded_rolls_are_deterministic() {
    let seed = Some(TEST_SEED);
    let rolls = ["2d6", "1d20", "2d8", "9d4", "1d12"];
    for s in &rolls {
        let a = solve_dice_expression(&s.to_string(), seed);
        let b = solve_dice_expression(&s.to_string(), seed);

        assert_eq!(a.unwrap(), b.unwrap());
    }
}

#[test]
fn multiple_rolls() {
    let a = solve_dice_expression(&"1d6 + 1d6".to_string(), None).unwrap();
    assert!((2..=12).contains(&a))
}

#[test]
fn fuzz_artifacts_dont_cause_crashes_anymore() {
    let inputs = vec![
        "6d0%",
        "994d01$",
        "2222225d22225d||?",
        "!-0d11111111111d//00,",
    ];
    for s in &inputs {
        assert!(solve_dice_expression(&s.to_string(), None).is_err());
    }
}
