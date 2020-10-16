#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

use evalexpr::eval;
use failure::Error;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use regex::{Captures, Regex};
use std::borrow::Cow;

mod dice;
use dice::Dice;

#[derive(Debug, Fail)]
#[fail(display = "{} (Context: {})", err, context)]
struct ErrorWithContext {
    context: String,
    err: Error,
}

impl ErrorWithContext {
    pub fn new(err: Error, context: String) -> ErrorWithContext {
        ErrorWithContext {
            err: err,
            context: context,
        }
    }
}

#[derive(Debug, Clone, Fail)]
enum DiceRollError {
    #[fail(display = "Adding {} and {} caused integer overflow", _0, _1)]
    IntegerOverFlow(u32, u32),
}

// TODO: this function is the hot path for very large numbers of rolls.
fn roll_dice(rng: &mut SmallRng, dice: &Dice) -> Result<u32, DiceRollError> {
    let mut result: u32 = 0;

    // TODO: experiment with a bigint implementation, benchmark against native
    // integers.
    for _ in 0..dice.rolls {
        // TODO: benchmark this against unchecked +=
        let roll = rng.gen_range(1, dice.sides);
        result = result
            .checked_add(roll)
            .ok_or(DiceRollError::IntegerOverFlow(result, roll))?;
    }

    return Ok(result);
}

// TODO: errors need to bubble up properly and not panic
pub fn solve_dice_expression(expression: String, random_seed: Option<u64>) -> Result<i64, Error> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+)d(\d+)").expect("Problem compiling regex");
    }

    // Initialize our RNG
    let mut rng = match random_seed {
        Some(inner) => SmallRng::seed_from_u64(inner),
        None => SmallRng::from_entropy(),
    };

    // In order to bubble up errors from Regex::replace, we use this Option
    let mut error = None;

    // For every match on the Dice expression regex, roll it in-place.
    let rolled_expression = PATTERN.replace_all(&expression, |caps: &Captures| {
        // FIXME: the unwrap here can cause a panic
        let diceroll_str = &caps.get(0).unwrap().as_str().to_string();
        match Dice::from_string(&diceroll_str) {
            Ok(dice) => match roll_dice(&mut rng, &dice) {
                Ok(roll_result) => return Cow::Owned(format!("{}", roll_result)),
                Err(e) => {
                    error = Some(ErrorWithContext::new(e.into(), diceroll_str.clone()));
                    Cow::Borrowed("")
                }
            },
            Err(e) => {
                error = Some(ErrorWithContext::new(e.into(), diceroll_str.clone()));
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
