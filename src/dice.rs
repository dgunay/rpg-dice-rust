use regex::Regex;
use std::error::Error;
use std::fmt;

pub struct Dice {
  pub rolls: u32,
  pub sides: u32,
}

#[derive(Debug, Clone)]
pub enum DiceError {
  InvalidSides(u32),
  InvalidRolls(u32),
}

impl fmt::Display for DiceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DiceError::InvalidSides(sides) => write!(f, "Sides must be greater than 0 (was {})", sides),
      DiceError::InvalidRolls(rolls) => write!(f, "Rolls must be greater than 0 (was {})", rolls),
    }
  }
}

impl std::error::Error for DiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    // Generic error, underlying cause isn't tracked.
    None
  }
}

impl Dice {
  pub fn new(rolls: u32, sides: u32) -> Result<Dice, Box<dyn Error>> {
    if rolls < 1 {
      return Err(Box::new(DiceError::InvalidRolls(rolls)));
    }

    if sides < 1 {
      return Err(Box::new(DiceError::InvalidSides(sides)));
    }

    return Ok(Dice {
      rolls: rolls,
      sides: sides,
    });
  }

  pub fn from_string(string: &String) -> Result<Dice, Box<dyn Error>> {
    let (rolls, sides) = Dice::parse_rolls_and_sides(string);
    return Dice::new(rolls, sides);
  }

  pub fn parse_rolls_and_sides(string: &String) -> (u32, u32) {
    // parse into rolls and sides, with regex validation
    lazy_static! {
      static ref PATTERN: Regex = Regex::new(r"^(\d+)d(\d+)$").unwrap();
    }

    // Parse the captures as u32s.
    let captures = PATTERN.captures(string).unwrap();

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

    return (rolls, sides);
  }
}

#[cfg(test)]
mod dice_unit_tests {
  use super::Dice;
  #[test]
  fn dice_from_string() {
    let _d = Dice::from_string(&"1d6".to_string());
    assert!(true);
  }
}
