extern crate structopt;
use structopt::StructOpt;
use rpg_dice_rust::lib::solve_dice_expression;

#[derive(StructOpt)]
struct Cli {
    // The dice expression (such as '1d6 + 5')
    #[structopt(required = true, min_values = 1)]
    expression: Vec<String>,

    // An optional random seed for repeatable results.
    #[structopt(short, long)]
    random_seed: Option<u64>,
}

fn main() {
    let args = Cli::from_args();

    let expression = args.expression;
    
    // Mash the Strings together into one
    let combined_expression = expression.join("");

    println!("{}", solve_dice_expression(combined_expression, args.random_seed));
}
