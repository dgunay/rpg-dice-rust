#[macro_use]
extern crate lazy_static;

pub mod lib {
  use rand::{rngs::SmallRng, Rng, SeedableRng};
  use regex::Regex;
  use std::str::FromStr;
  use evalexpr::eval;

  pub struct DiceExpression {}
  impl DiceExpression {
    pub fn new(tokens: Vec<String>) -> DiceExpression {
      return DiceExpression {};
    }
  }

  pub enum MathToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    ParenOpen,
    ParenClose,
  }

  impl FromStr for MathToken {
    type Err = ();

    fn from_str(s: &str) -> Result<MathToken, ()> {
      match s {
        "+" => Ok(MathToken::Plus),
        "-" => Ok(MathToken::Minus),
        "*" => Ok(MathToken::Multiply),
        "/" => Ok(MathToken::Divide),
        "(" => Ok(MathToken::ParenOpen),
        ")" => Ok(MathToken::ParenClose),
        _ => Err(()),
      }
    }
  }

  // TODO: try generating docs
  pub struct DiceToken {
    rolls: u32,
    sides: u32,
  }
  impl DiceToken {
    pub fn new(rolls: u32, sides: u32) -> DiceToken {
      return DiceToken {
        rolls: rolls,
        sides: sides,
      };
    }

    pub fn from_string(string: String) -> DiceToken {
      // parse into rolls and sides, with regex validation
      lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)d(\d+)$").unwrap();
      }

      let captures = PATTERN.captures(&string).unwrap();

      // Parse the captures at u32s.
      let rolls = captures.get(0).unwrap().as_str().parse::<u32>().unwrap();
      let sides = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();

      return DiceToken::new(rolls, sides);
    }

    pub fn roll(&self, random_seed: Option<u64>) -> u32 {
      // TODO: may be better to move this to a static area instead of making it
      // for every roll
      let mut rng = match random_seed {
        Some(inner) => SmallRng::seed_from_u64(inner),
        None => SmallRng::from_entropy(),
      };

      let mut result = 0;
      for _ in 0..self.rolls {
        result += rng.gen_range(1, self.sides);
      }
      return result;
    }
  }

  pub fn solve_dice_expression(expression: String, random_seed: Option<u64>) -> i64 {
    // Parse the expression into tokens

    // Roll all of the dice tokens in-place

    // Calculate the result
    let result = match eval(&expression).unwrap() {
      evalexpr::Value::Int(inner) => inner,
      _ => panic!("Not an int, something went wrong")
    };

    return result;
  }

  #[cfg(test)]
  mod tests {
    use crate::lib::solve_dice_expression;

    #[test]
    fn solve_dice_expression_can_do_basic_math() {
      assert_eq!(4, solve_dice_expression(String::from("2 + 2"), None));
    }
  }
}
