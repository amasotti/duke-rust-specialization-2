use clap::Parser;
use fruit_cli::create_salad;

#[derive(Parser)]
#[clap(
    name = "fruit-cli",
    version = "1.0.0",
    author = "Antonio Masotti",
    about = "Create a fruit salad"
)]
struct Opts {
    #[clap(short, long, default_value = "3")]
    n_fruits: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let salad = create_salad(opts.n_fruits);

    println!("Fruit Salad:");
    for fruit in salad {
        println!("\t- {}", fruit);
    }
}
