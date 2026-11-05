use calculator::calculate;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    x: f64,
    operator: char,
    y: f64,
}

fn main() {
    let args = Cli::parse();
    print!(
        "{} {} {} = {}",
        args.x,
        args.operator,
        args.y,
        calculate(args.x, args.y, args.operator)
    );
}
