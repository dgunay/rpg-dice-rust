extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // The dice expression (such as '1d6 + 5')
    #[structopt(required = true, min_values = 1)]
    expression: Vec<String>,

    // An optional random seed for repeatable results.
    #[structopt(short, long)]
    random_seed: Option<i32>,
}

fn main() {
    let args = Cli::from_args();

    
}
