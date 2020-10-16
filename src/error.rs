//! Errors that can occur in this crate.

use crate::dice::DiceRoll;
use thiserror::Error;

/// Things that can go wrong when rolling dice.
#[derive(Debug, Clone, Error)]
pub enum DiceRollError {
    /// Attempted to add two integers which would overflow (usually because the
    /// dice roll is too big)
    #[error("Adding integers {0} and {1} would overflow")]
    IntegerOverFlow(u32, u32),
}

/// Things that can go wrong when instantiating a Dice
#[derive(Debug, Clone, Error)]
pub enum DiceError {
    /// Attempted to create a DiceRoll with too few sides.
    #[error(
        "'{0}' is an invalid number of sides for a Dice (must be at least {})",
        DiceRoll::MINIMUM_SIDES
    )]
    InvalidSides(u32),

    /// Attempted to create a DiceRoll with too few rolls.
    #[error(
        "'{0}' is an invalid number of rolls for a Dice (must be at least {})",
        DiceRoll::MINIMUM_ROLLS
    )]
    InvalidRolls(u32),

    /// Sides were not captured from the provided expression.
    #[error("Sides were not captured from the provided expression '{0}'.")]
    FailedToParseSides(String),

    /// Rolls were not captured from the provided expression.
    #[error("Rolls were not captured from the provided expression '{0}'.")]
    FailedToParseRolls(String),

    /// Attempted to create a DiceRoll from an invalid string expression.
    #[error("'{0}' could not be parsed into a pair of rolls and sides.")]
    InvalidExpression(String),
}
