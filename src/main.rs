use clap::Parser;
#[cfg(feature = "half")]
use half::{bf16, f16};
use std::borrow::Cow;
use std::collections::HashSet;
use std::io::{self, Read, Write};
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
    /// Set color choice. One of: always, always-ansi, auto, or never.
    #[clap(long, value_parser(color_style))]
    color: Option<ColorChoice>,
    /// Set the allowed type.
    #[clap(long("type"), value_parser(allowed_type), use_value_delimiter(true))]
    types: Option<Vec<AllowedType>>,
}

fn main() {
    let options = Options::parse();

    let input = match read_input(options.stdin, &options.input) {
        None => return,
        Some(input) => input,
    };

    let color_choice = options.color.unwrap_or(ColorChoice::Auto);
    let mut stdout = StandardStream::stdout(color_choice);

    let types = options.types.map_or(HashSet::new(), HashSet::from_iter);
    let type_enabled = |t: AllowedType| types.is_empty() || types.contains(&t);

    #[cfg(feature = "half")]
    if type_enabled(AllowedType::Ieee754HalfPrecision) {
        try_handle_f16(&input, &mut stdout).ok();
    }

    #[cfg(feature = "half")]
    if type_enabled(AllowedType::BrainFloatingPoint) {
        try_handle_bf16(&input, &mut stdout).ok();
    }

    if type_enabled(AllowedType::Ieee754SinglePrecision) {
        try_handle_f32(&input, &mut stdout).ok();
    }

    if type_enabled(AllowedType::Ieee754DoublePrecision) {
        try_handle_f64(&input, &mut stdout).ok();
    }
}

#[cfg(feature = "half")]
fn try_handle_f16<I: AsRef<str>>(input: I, mut stdout: &mut StandardStream) -> io::Result<bool> {
    if let Ok(value) = f16::from_str(input.as_ref()) {
        let bits: u16 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("f16:  {:016b}", bits),
            [
                Foreground::None(6),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 5),
                Foreground::Color(Color::Blue, 10),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "SEEEEEMMMMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 5),
                Foreground::Color(Color::Blue, 10),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "E: Exponent (5 bits)",
            [Foreground::Color(Color::Green, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "M: Fraction / Mantissa (10 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(feature = "half")]
fn try_handle_bf16<I: AsRef<str>>(input: I, mut stdout: &mut StandardStream) -> io::Result<bool> {
    if let Ok(value) = bf16::from_str(input.as_ref()) {
        let bits: u16 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("bf16: {:016b}", bits),
            [
                Foreground::None(6),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 7),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "SEEEEEEEEMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 7),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "E: Exponent (8 bits)",
            [Foreground::Color(Color::Green, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "M: Fraction / Mantissa (7 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn try_handle_f32<I: AsRef<str>>(input: I, mut stdout: &mut StandardStream) -> io::Result<bool> {
    if let Ok(value) = f32::from_str(input.as_ref()) {
        let bits: u32 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("f32:  {:032b}", bits),
            [
                Foreground::None(6),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 23),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "SEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 8),
                Foreground::Color(Color::Blue, 23),
            ],
        )?;
        print_colored(
            &mut stdout,
            6,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "E: Exponent (8 bits)",
            [Foreground::Color(Color::Green, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "M: Fraction / Mantissa (23 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn try_handle_f64<I: AsRef<str>>(input: I, mut stdout: &mut StandardStream) -> io::Result<bool> {
    if let Ok(value) = f64::from_str(input.as_ref()) {
        let bits: u64 = unsafe { std::mem::transmute(value) };

        print_colored(
            &mut stdout,
            0,
            format!("f64:  {:064b}", bits),
            [
                Foreground::None(6),
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 11),
                Foreground::Color(Color::Blue, 52),
            ],
        )?;

        print_colored(
            &mut stdout,
            6,
            "SEEEEEEEEEEEMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            [
                Foreground::Color(Color::Red, 1),
                Foreground::Color(Color::Green, 11),
                Foreground::Color(Color::Blue, 52),
            ],
        )?;

        print_colored(
            &mut stdout,
            6,
            "S: Sign (1 bit)",
            [Foreground::Color(Color::Red, 0)],
        )
        .ok();
        print_colored(
            &mut stdout,
            6,
            "E: Exponent (11 bits)",
            [Foreground::Color(Color::Green, 0)],
        )?;
        print_colored(
            &mut stdout,
            6,
            "M: Fraction / Mantissa (52 bits)",
            [Foreground::Color(Color::Blue, 0)],
        )?;
        Ok(true)
    } else {
        Ok(false)
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

fn read_input(stdin: bool, input: &Option<String>) -> Option<Cow<str>> {
    if stdin {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        Some(Cow::Owned(buffer.trim().to_string()))
    } else if let Some(input) = &input {
        Some(Cow::Borrowed(&input))
    } else {
        None
    }
}

fn color_style(s: &str) -> Result<ColorChoice, String> {
    match s {
        "always" => Ok(ColorChoice::Always),
        "always-ansi" => Ok(ColorChoice::AlwaysAnsi),
        "auto" => Ok(ColorChoice::Auto),
        "never" => Ok(ColorChoice::Never),
        _ => Err(String::from(
            "Either never, auto, always or always-ansi must be specified",
        )),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AllowedType {
    /// Enable [`f16`].
    #[cfg(feature = "half")]
    Ieee754HalfPrecision,
    /// Enable [`bf16`].
    #[cfg(feature = "half")]
    BrainFloatingPoint,
    /// Enable [`f32`].
    Ieee754SinglePrecision,
    /// Enable [`f64`].
    Ieee754DoublePrecision,
}

fn allowed_type(s: &str) -> Result<AllowedType, String> {
    match s {
        #[cfg(feature = "half")]
        "f16" => Ok(AllowedType::Ieee754HalfPrecision),
        #[cfg(feature = "half")]
        "bf16" => Ok(AllowedType::BrainFloatingPoint),
        "f32" => Ok(AllowedType::Ieee754SinglePrecision),
        "f64" => Ok(AllowedType::Ieee754DoublePrecision),
        _ => Err(String::from(
            "Either f16, bf16, f32 or f64 must be specified",
        )),
    }
}
