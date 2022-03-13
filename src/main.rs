use bat::PrettyPrinter;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("squish")
        .arg(
            Arg::new("language")
                .long("language")
                .short('l')
                .help("Language to use for pretty printing")
                .takes_value(true),
        )
        .get_matches();

    let mut printer = PrettyPrinter::new();
    printer.input_stdin();
    if let Some(lang) = matches.value_of("language") {
        printer.language(lang);
    }
    printer.print().unwrap();
}
