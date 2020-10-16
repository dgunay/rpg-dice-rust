#[macro_use]
extern crate lazy_static;

pub mod dice;
pub mod error;

use anyhow::Result;
use dice::Dice;
use evalexpr::eval;
use rand::{rngs::SmallRng, SeedableRng};
use regex::{Captures, Regex};
use std::borrow::Cow;

/// Solves a dice expression string by rolling each dice in-place and then 
/// evaluating the resulting arithmetic expression.
pub fn solve_dice_expression(expression: String, random_seed: Option<u64>) -> Result<i64> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)d(\d+)").expect("Problem compiling regex");
    }

    // Initialize our RNG
    let mut rng = match random_seed {
        Some(inner) => SmallRng::seed_from_u64(inner),
        None => SmallRng::from_entropy(),
    };

    // In order to bubble up errors from Regex::replace, we capture this Option
    // to smuggle it out.
    let mut error = None;

    // For every match on the Dice expression regex, roll it in-place.
    let rolled_expression = PATTERN.replace_all(&expression, |caps: &Captures| {
        // FIXME: the unwrap here can cause a panic
        let diceroll_str = &caps.get(0).unwrap().as_str().to_string();
        match Dice::from_string(&diceroll_str) {
            Ok(dice) => match dice.roll(&mut rng) {
                Ok(roll_result) => return Cow::Owned(format!("{}", roll_result)),
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

    match error {
        Some(e) => Err(e.into()),
        None => {
            // Calculate the result
            let result = eval(&rolled_expression)?.as_int()?;
            return Ok(result);
        }
    }
}
