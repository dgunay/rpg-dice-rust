use failure::Error;
use regex::Regex;
use std::fmt;

pub struct Dice {
    pub rolls: u32,
    pub sides: u32,
}

#[derive(Debug, Clone, Fail)]
pub enum DiceError {
    InvalidSides(u32),
    InvalidRolls(u32),
}

impl fmt::Display for DiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiceError::InvalidSides(sides) => write!(
                f,
                "Sides must be at least {} (was {})",
                Dice::MINIMUM_SIDES,
                sides
            ),
            DiceError::InvalidRolls(rolls) => write!(
                f,
                "Rolls must be at least {} (was {})",
                Dice::MINIMUM_ROLLS,
                rolls
            ),
        }
    }
}

impl Dice {
    pub const MINIMUM_ROLLS: u32 = 1;
    pub const MINIMUM_SIDES: u32 = 2;

    pub fn new(rolls: u32, sides: u32) -> Result<Dice, Error> {
        if rolls < Dice::MINIMUM_ROLLS {
            return Err(DiceError::InvalidRolls(rolls).into());
        }

        if sides < Dice::MINIMUM_SIDES {
            return Err(DiceError::InvalidSides(sides).into());
        }

        return Ok(Dice {
            rolls: rolls,
            sides: sides,
        });
    }

    pub fn from_string(string: &String) -> Result<Dice, Error> {
        let (rolls, sides) = Dice::parse_rolls_and_sides(string)?;
        return Dice::new(rolls, sides);
    }

    pub fn parse_rolls_and_sides(string: &String) -> Result<(u32, u32), Error> {
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
            .parse::<u32>()?;
        let sides = captures
            .get(2)
            .expect(format!("Failed to match number of sides for {}", string).as_str())
            .as_str()
            .parse::<u32>()?;

        return Ok((rolls, sides));
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
