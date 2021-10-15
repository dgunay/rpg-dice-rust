//! Represents a Dice.

use anyhow::Result;
use rand::{prelude::Rng, RngCore};
use regex::Regex;

use crate::error::{Error, RollError};

/// A dice roll expressed in RPG term e.g. 3d6 means "roll a 6-sided die 3 times".
pub struct DiceRoll {
    /// How many times the dice will be rolled.
    pub rolls: u32,

    /// The maximum number of sides the dice has. If it has 6 sides, the most it
    /// can roll for at any one time is 6.
    pub sides: u32,
}

impl DiceRoll {
    /// All dice must be rollable at least once.
    pub const MINIMUM_ROLLS: u32 = 1;

    /// All dice must have at least 2 sides.
    pub const MINIMUM_SIDES: u32 = 2;

    /// Create a `DiceRoll`.
    ///
    /// # Errors
    /// - `rolls` is less than 1
    /// - `sides` is less than 2
    pub fn new(rolls: u32, sides: u32) -> Result<Self> {
        if rolls < Self::MINIMUM_ROLLS {
            return Err(Error::InvalidRolls(rolls).into());
        }

        if sides < Self::MINIMUM_SIDES {
            return Err(Error::InvalidSides(sides).into());
        }

        Ok(Self { rolls, sides })
    }

    /// Create a `DiceRoll` from a string.
    /// ```
    /// use dicelib::dice::DiceRoll;
    /// let dice = DiceRoll::from_string(&"3d6".to_string()).unwrap();
    /// ```
    ///
    /// # Errors
    /// - See `parse_rolls_and_sides()`
    // TODO: use &str instead of &String
    pub fn from_string(string: &str) -> Result<Self> {
        let (rolls, sides) = Self::parse_rolls_and_sides(string)?;
        Self::new(rolls, sides)
    }

    /// Utility function to parse the rolls and sides of a dice roll string
    /// into a pair of u32s. If you want a `DiceRoll`, use `from_string()` instead.
    /// ```
    /// use dicelib::dice::DiceRoll;
    /// let (rolls, sides) = DiceRoll::parse_rolls_and_sides(&"1d4".to_string()).unwrap();
    /// ```
    ///
    /// # Errors
    /// - If rolls or sides cannot be matched (expression is malformed)
    /// - If the matched rolls and sides are not parseable as `u32`
    pub fn parse_rolls_and_sides(string: &str) -> Result<(u32, u32)> {
        // parse into rolls and sides, with regex validation
        lazy_static! {
            static ref PATTERN: Regex = Regex::new(r"^(\d+)d(\d+)$").unwrap();
        }

        // Parse the captures as u32s.
        let captures = PATTERN
            .captures(string)
            .ok_or_else(|| Error::InvalidExpression(string.to_string()))?;

        // The error handling here is more of a formality because if we got this
        // far, we probably matched two ints.
        let rolls = captures
            .get(1)
            .ok_or_else(|| Error::FailedToParseRolls(string.to_string()))?
            .as_str()
            .parse::<u32>()?;
        let sides = captures
            .get(2)
            .ok_or_else(|| Error::FailedToParseSides(string.to_string()))?
            .as_str()
            .parse::<u32>()?;

        Ok((rolls, sides))
    }

    // TODO: this function is the hot path for very large numbers of rolls.
    /// Performs the `DiceRoll` and returns the sum of all rolls.
    ///
    /// # Errors
    /// - `IntegerOverFlow` if the rolls and sides are very, very big numbers.
    pub fn roll(&self, rng: &mut impl RngCore) -> Result<u32> {
        let mut result: u32 = 0;

        // TODO: experiment with a bigint implementation, benchmark against native
        // integers.
        for _ in 0..self.rolls {
            // TODO: benchmark this against unchecked +=
            let roll = rng.gen_range(1, self.sides + 1);
            result = result
                .checked_add(roll)
                .ok_or(RollError::IntegerOverFlow(result, roll))?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod dice_unit_tests {
    use super::DiceRoll;
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn dice_from_string() {
        let mut rng = SmallRng::from_entropy();
        let d = DiceRoll::from_string(&"1d6".to_string());
        let r = d.unwrap().roll(&mut rng).unwrap();
        assert!((1..=6).contains(&r));
    }
}
