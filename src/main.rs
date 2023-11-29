use clap::Parser;
use std::borrow::Cow;
use std::io::{self, repeat, Read, Write};
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    if let Ok(value) = f32::from_str(&input) {
        let bits: u32 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("f32: {:032b}", bits),
            [
                Foreground::None(5),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 23),
            ],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 23),
            ],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "E: Exponent (8 bits)",
            [Foreground::Color(Color::Green, 0)],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "M: Fraction / Mantissa (23 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )
        .ok();
    }

    if let Ok(value) = f64::from_str(&input) {
        let bits: u64 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("f64: {:064b}", bits),
            [
                Foreground::None(5),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 11),
                Foreground::Color(Color::Blue, 52),
            ],
        )
        .ok();

        print_colored(
            &mut stdout,
            5,
            "SEEEEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 11),
                Foreground::Color(Color::Blue, 52),
            ],
        )
        .ok();

        print_colored(
            &mut stdout,
            5,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "E: Exponent (11 bits)",
            [Foreground::Color(Color::Green, 0)],
        )
        .ok();
        print_colored(
            &mut stdout,
            5,
            "M: Fraction / Mantissa (52 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )
        .ok();
    }
}

fn print_colored<S: AsRef<str>, C: IntoIterator<Item = Foreground>>(
    stdout: &mut StandardStream,
    padding: usize,
    input: S,
    colors: C,
) -> io::Result<()> {
    write!(stdout, "{}", " ".repeat(padding))?;

    let input = input.as_ref();
    let mut start = 0;
    for color in colors {
        let (color, repetitions) = match color {
            Foreground::Color(color, repetitions) => (Some(color), repetitions),
            Foreground::None(repetitions) => (None, repetitions),
        };

        let repetitions = if repetitions == 0 {
            input.len() - start
        } else {
            repetitions
        };
        let end = start + repetitions;
        let slice = &input[start..end];
        start = end;
        stdout.set_color(ColorSpec::new().set_fg(color))?;
        write!(stdout, "{}", slice)?
    }

    stdout.reset()?;

    if start < input.len() {
        let slice = &input[start..];
        write!(stdout, "{}", slice)?
    }

    writeln!(stdout, "")?;
    Ok(())
}

enum Foreground {
    Color(Color, usize),
    None(usize),
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