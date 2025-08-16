use clap::{Arg, Command};
use clap::Parser;

fn _deprecated_main() {
    let _mactches = Command::new("echor")
                                .version("0.1.0")
                                .author("jong")
                                .about("Rust version of 'echo'")
                                .arg(Arg::new("omit_newline")
                                        .short('n')
                                        .action(clap::ArgAction::SetTrue)
                                        .help("Do not print newline")
                                ).arg(Arg::new("text")
                                        .value_name("TEXT")
                                        .required(true)
                                        .help("Input text")
                                        .num_args(1..),
                                ).get_matches();

    let text: Vec<String> = _mactches.get_many::<String>("text").unwrap().cloned().collect();
    let omit_newline = _mactches.get_flag("omit_newline");

    // let ending;
    // if omit_newline {
    //     ending = ""; // it does not work
    // } else {
    //     ending = "\n";
    // }
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{ending}", text.join(" "));
}

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
    let ending = if _args.omit_newline { "" } else { "\n" };

    print!("{}{ending}", _args.text.join(" "));
}