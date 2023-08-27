use clap::Parser;
use cli_db::args::CliArgs;

fn main() {
    let cli = CliArgs::parse();

    println!("{:?}", cli);
}
