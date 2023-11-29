use clap::Parser;
use std::borrow::Cow;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Debug, clap::Parser)]
#[clap(name = "bits")]
struct Options {
    /// The input to use. Will be ignored if --stdin flag is used.
    #[clap(group = "input_source", allow_hyphen_values = true)]
    input: Option<String>,
    /// If set, read input from stdin instead of positional argument.
    #[clap(long, group = "input_source")]
    stdin: bool,
}

fn main() {
    let options = Options::parse();

    let input = match read_input(&options) {
        None => return,
        Some(input) => input,
    };

    if let Ok(value) = f32::from_str(&input) {
        let bits: u32 = unsafe { std::mem::transmute(value) };

        println!("f32: {:032b}", bits);
        println!("     SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM");
        println!("     S: Sign (1 bit)");
        println!("     E: Exponent (8 bits)");
        println!("     M: Fraction/Mantissa (23 bits)");
    }

    if let Ok(value) = f64::from_str(&input) {
        let bits: u64 = unsafe { std::mem::transmute(value) };
        println!("f64: {:064b}", bits);
        println!("     SEEEEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM");
        println!("     S: Sign (1 bit)");
        println!("     E: Exponent (11 bits)");
        println!("     M: Fraction/Mantissa (52 bits)");
    }
}

fn read_input(options: &Options) -> Option<Cow<str>> {
    if options.stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        Some(Cow::Owned(buffer.trim().to_string()))
    } else if let Some(input) = &options.input {
        Some(Cow::Borrowed(&input))
    } else {
        None
    }
}
