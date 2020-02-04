#[macro_use]
extern crate lazy_static;

pub mod lib {
  use evalexpr::eval;
  use rand::{rngs::SmallRng, Rng, SeedableRng};
  use regex::{Captures, Regex};
  use std::error::Error;

  // TODO: try generating docs
  pub struct Dice {
    rolls: u32,
    sides: u32,
  }

  impl Dice {
    pub fn new(
      rolls: u32, sides: u32
    ) -> Dice {

      return Dice {
        rolls: rolls,
        sides: sides,
      };
    }

    pub fn from_string(
      string: &String
    ) -> Result<Dice, Box<dyn Error>> {
      // parse into rolls and sides, with regex validation
      lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)d(\d+)$").unwrap();
      }

      let captures = PATTERN.captures(string).unwrap();

      // Parse the captures as u32s.
      let rolls = captures
        .get(1)
        .expect(format!("Failed to match number of rolls for {}", string).as_str())
        .as_str()
        .parse::<u32>()
        .unwrap();
      let sides = captures
        .get(2)
        .expect(format!("Failed to match number of sides for {}", string).as_str())
        .as_str()
        .parse::<u32>()
        .unwrap();

      return Ok(Dice::new(rolls, sides));
    }    
  }

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
      None        => SmallRng::from_entropy(),
    };

    // For every match on the Dice expression regex, roll it in-place.
    let rolled_expression = PATTERN.replace(&expression, |caps: &Captures| {
      let dice = Dice::from_string(&caps.get(0).unwrap().as_str().to_string()).unwrap();
      return format!("{}", roll_dice(&mut rng, &dice));
    });

    // Calculate the result
    let result = eval(&rolled_expression)?.as_int()?;

    return Ok(result);
  }

  #[cfg(test)]
  mod tests {
    use crate::lib::*;

    const TEST_SEED: u64 = 42;

    #[test]
    fn solve_dice_expression_can_do_basic_math() {
      assert_eq!(
        4,
        solve_dice_expression(String::from("2 + 2"), None).unwrap()
      );
    }

    #[test]
    fn seeded_rolls_are_deterministic() {
      let seed = Some(TEST_SEED);
      let rolls = ["2d6", "1d20", "2d8", "9d4", "1d12"];
      for s in &rolls {
        let a = solve_dice_expression(s.to_string(), seed);
        let b = solve_dice_expression(s.to_string(), seed);

        assert_eq!(a.unwrap(), b.unwrap());
      }
    }

    #[test]
    fn fuzz_artifacts_dont_cause_crashes_anymore() {
      let inputs = ["6d0%"];
      for s in &inputs {
        solve_dice_expression(s.to_string(), None).unwrap();
      }

      assert!(true);
    }
  }
}
