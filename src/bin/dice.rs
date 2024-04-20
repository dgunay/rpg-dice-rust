extern crate structopt;

use dicelib::solve_dice_expression;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "dice",
    about = "A command line dice roller.",
    author = "Devin Gunay <devingunay@gmail.com>"
)]
struct Cli {
    /// The dice expression (such as '1d6 + 5')
    #[structopt(required = true, min_values = 1)]
    expression: Vec<String>,

    /// An optional random seed for repeatable results.
    #[structopt(short, long)]
    random_seed: Option<u64>,

    /// If set, the expression after rolling the dice will be printed.
    #[structopt(short, long)]
    verbose: bool,
}

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let args = Cli::from_args();

    let expression = args.expression;

    // Mash the Strings together into one
    let combined_expression = expression.join("");

    let result = solve_dice_expression(&combined_expression, args.random_seed);
    match result {
        Ok(out) => {
            if args.verbose {
                println!("Before: {}", combined_expression);
                println!("After: {}", out.rolled_expression);
            }

            println!("{}", out.result);
        }
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
