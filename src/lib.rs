#[macro_use]
extern crate lazy_static;
use evalexpr::eval;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use regex::{Captures, Regex};
use std::error::Error;
use std::borrow::Cow;

mod dice;
use dice::Dice;

fn roll_dice(rng: &mut SmallRng, dice: &Dice) -> u32 {
  let mut result = 0;

  for _ in 0..dice.rolls {
    result += rng.gen_range(1, dice.sides);
  }

  return result;
}

// TODO: errors need to bubble up properly and not panic
pub fn solve_dice_expression(
  expression: String,
  random_seed: Option<u64>,
) -> Result<i64, Box<dyn Error>> {
  lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"(\d+)d(\d+)").expect("Problem compiling regex");
  }

  // Initialize our RNG
  let mut rng = match random_seed {
    Some(inner) => SmallRng::seed_from_u64(inner),
    None => SmallRng::from_entropy(),
  };

  // In order to bubble up errors from Regex::replace, we use this variable.
  let mut error = None;

  // For every match on the Dice expression regex, roll it in-place.
  let rolled_expression = PATTERN.replace_all(&expression, |caps: &Captures| {
    let diceroll_str = &caps.get(0).unwrap().as_str().to_string();
    match Dice::from_string(&diceroll_str) {
      Ok(dice) => return Cow::Owned(format!("{}", roll_dice(&mut rng, &dice))),
      Err(e) => {
        error = Some(e);
        return Cow::Borrowed("");
      }
    }
  });

  match error {
    Some(e) => Err(e),
    None => {
      // Calculate the result
      let result = eval(&rolled_expression)?.as_int()?;
      return Ok(result);
    }
  }
}
