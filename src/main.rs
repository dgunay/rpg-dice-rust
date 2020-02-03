extern crate structopt;
use rpg_dice_rust::lib::solve_dice_expression;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The dice expression (such as '1d6 + 5')
    #[structopt(required = true, min_values = 1)]
    expression: Vec<String>,

    // An optional random seed for repeatable results.
    #[structopt(short, long)]
    random_seed: Option<u64>,
}

// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let args = Cli::from_args();

    let expression = args.expression;

    // Mash the Strings together into one
    let combined_expression = expression.join("");

    let result = solve_dice_expression(combined_expression, args.random_seed);
    match result {
        Ok(out) => println!("{}", out),
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
