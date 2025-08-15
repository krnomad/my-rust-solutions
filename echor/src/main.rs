use clap::{Arg, Command};

fn main() {
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
