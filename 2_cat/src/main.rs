use clap::Parser;
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(required(true))]
    text: Vec<String>,

    #[arg(short('n'), long("no-newline"), help("do not print new line"))]
    omit_newline: bool,
}

fn main() {
    let _args = Args::parse();
}
