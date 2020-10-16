use thiserror::Error;
use crate::dice::Dice;

/// Things that can go wrong when rolling dice.
#[derive(Debug, Clone, Error)]
pub enum DiceRollError {
    #[error("Adding integers {0} and {1} would overflow")]
    IntegerOverFlow(u32, u32),
}

/// Things that can go wrong when instantiating a Dice
#[derive(Debug, Clone, Error)]
pub enum DiceError {
    // TODO: can these be not hardcoded (maybe overkill considering how invariant this is)
    #[error("'{0}' is an invalid number of sides for a Dice (must be at least {})", Dice::MINIMUM_SIDES)]
    InvalidSides(u32),
    #[error("'{0}' is an invalid number of rolls for a Dice (must be at least {})", Dice::MINIMUM_ROLLS)]
    InvalidRolls(u32),
}
