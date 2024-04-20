//! The RPG Dice Rust crate. A combination command line dice roller and library
//! for evaluating dice roll expressions.

// Enables a lot of annoying warnings to keep us honest.
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
// Warnings we sincerely don't care about.
#![allow(clippy::module_name_repetitions)]
// Won't compile if there are missing docs.
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

pub mod dice;
pub mod error;

use anyhow::Result;
use dice::DiceRoll;
use evalexpr::eval;
use rand::{rngs::SmallRng, SeedableRng};
use regex::{Captures, Regex};
use std::borrow::Cow;

/// A struct representing the result of a dice roll.
pub struct DiceResult {
    /// The final result of the dice roll.
    pub result: i64,

    /// The expression after rolling all the dice.
    pub rolled_expression: String,
}

/// Solves a dice expression string by rolling each dice in-place and then
/// evaluating the resulting arithmetic expression.
///
/// ```
/// use dicelib::solve_dice_expression;
/// let result = solve_dice_expression(&"2d5 + 4".to_string(), None).unwrap();
/// assert!(result >= 6 && result <= 14);
/// ```
///
/// # Errors
/// - Integer overflow from huge dice rolls.
///
/// # Panics
/// - Will panic if invalid `DiceRoll` expression given
pub fn solve_dice_expression(expression: &str, random_seed: Option<u64>) -> Result<DiceResult> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)d(\d+)").expect("Problem compiling regex");
    }

    // Initialize our RNG
    let mut rng = random_seed.map_or_else(SmallRng::from_entropy, |inner| {
        SmallRng::seed_from_u64(inner)
    });

    // In order to bubble up errors from Regex::replace, we capture this Option
    // to smuggle it out.
    let mut error = None;

    // For every match on the DiceRoll expression regex, roll it in-place.
    let rolled_expression = PATTERN.replace_all(expression, |caps: &Captures| {
        // FIXME: the unwrap here can cause a panic
        let diceroll_str = &caps.get(0).unwrap().as_str().to_string();
        match DiceRoll::from_string(diceroll_str) {
            Ok(dice) => match dice.roll(&mut rng) {
                Ok(roll_result) => Cow::Owned(roll_result.to_string()),
                Err(e) => {
                    error = Some(e.context(diceroll_str.clone()));
                    Cow::Borrowed("")
                }
            },
            Err(e) => {
                error = Some(e.context(diceroll_str.clone()));
                Cow::Borrowed("")
            }
        }
    });

    if let Some(e) = error {
        Err(e)
    } else {
        // Calculate the result
        let result = eval(&rolled_expression)?.as_int()?;
        Ok(DiceResult {
            result,
            rolled_expression: rolled_expression.to_string(),
        })
    }
}
