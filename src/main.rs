use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error, Read, Write},
    path::PathBuf,
};

enum Element {
    Text(&'static str),
    Addr(usize),
    Value(Base, usize),
    Ascii(usize),
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Base {
    Bin,
    Oct,
    Dec,
    Hex,
}
impl ToString for Base {
    fn to_string(&self) -> String {
        match self {
            Base::Bin => "bin".to_string(),
            Base::Oct => "oct".to_string(),
            Base::Dec => "dec".to_string(),
            Base::Hex => "hex".to_string(),
        }
    }
}
impl Base {
    fn write(self, byte: u8, w: &mut impl Write) -> Result<(), Error> {
        match self {
            Base::Bin => write!(w, "{:08b}", byte),
            Base::Oct => write!(w, "{:03o}", byte),
            Base::Dec => write!(w, "{:3}", byte),
            Base::Hex => write!(w, "{:02x}", byte),
        }
    }
    fn pad(self, w: &mut impl Write) -> Result<(), Error> {
        match self {
            Base::Bin => write!(w, "        "),
            Base::Oct => write!(w, "   "),
            Base::Dec => write!(w, "   "),
            Base::Hex => write!(w, "  "),
        }
    }
}

fn parse_range(s: &str) -> Result<(Option<usize>, Option<usize>), &'static str> {
    let err = "expecting a value matching the regex '[0-9]*\\.\\.[0-9]*'";
    let mut iter = s.split("..");
    if let (Some(start), Some(stop), None) = (iter.next(), iter.next(), iter.next()) {
        Ok((
            if start.is_empty() {
                None
            } else {
                Some(start.parse().map_err(|_| err)?)
            },
            if stop.is_empty() {
                None
            } else {
                Some(stop.parse().map_err(|_| err)?)
            },
        ))
    } else {
        Err(err)
    }
}

#[derive(clap::Parser)]
struct Args {
    /// Path to a binary file [default: stdin]
    input: Option<PathBuf>,
    /// Path to write the text output [default: stdout]
    output: Option<PathBuf>,
    #[arg(short, long)]
    /// Show the address of the first byte of each line
    address: bool,
    #[arg(short, long)]
    /// Show the corresponding ascii character
    text: bool,
    /// Numerical base for the bytes value
    #[arg(short, long, default_value_t = Base::Bin)]
    radix: Base,
    /// What range of the input to show, as N..N where N is an optional integer
    #[arg(short, long, value_parser = parse_range, default_value = "..")]
    select: (Option<usize>, Option<usize>),
    /// How many bytes per line
    #[arg(short, long, default_value_t = 8)]
    line_width: usize,
    /// Start a new line when a given byte value is found
    #[arg(short, long)]
    break_on: Vec<u8>,
    /// If the output should you ansi escapes sequences
    #[arg(short, long)]
    colored: bool,
}

impl Args {
    fn make_format(&self) -> Vec<Element> {
        let mut format = Vec::new();
        if self.address {
            if self.colored {
                format.push(Element::Text("\x1b[1;95m"));
                format.push(Element::Addr(0));
                format.push(Element::Text("\x1b[0m"));
            } else {
                format.push(Element::Addr(0));
            }
            format.push(Element::Text("  "));
        }
        let mut sep = false;
        for i in 0..self.line_width {
            if sep {
                format.push(Element::Text(" "));
            }
            sep = true;

            format.push(Element::Value(self.radix, i));
        }
        if self.text {
            format.push(Element::Text("  "));
            if self.colored {
                format.push(Element::Text("\x1b[94m"));
            }
            for i in 0..self.line_width {
                format.push(Element::Ascii(i));
            }
            if self.colored {
                format.push(Element::Text("\x1b[0m"));
            }
        }
        format.push(Element::Text("\n"));
        format
    }
    fn get_input(&self) -> Result<Box<dyn Read>, Error> {
        Ok(if let Some(ref input) = self.input {
            Box::new(File::open(input)?)
        } else {
            Box::new(std::io::stdin())
        })
    }
    fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        Ok(if let Some(ref output) = self.output {
            Box::new(File::create(output)?)
        } else {
            Box::new(std::io::stdout())
        })
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let format = args.make_format();

    let input = args.get_input()?;
    let output = args.get_output()?;

    let input = BufReader::new(input).bytes().enumerate();
    let mut output = BufWriter::new(output);

    let (start, stop) = args.select;
    // limit up to the end of the selection range
    let mut input: Box<dyn Iterator<Item = _>> = if let Some(stop) = stop {
        Box::new(input.take(stop))
    } else {
        Box::new(input)
    };
    // skip up to the begining of the selection range
    if let Some(start) = start {
        for _ in 0..start {
            input.next();
        }
    }

    let mut line = Vec::new();
    while {
        // load a line of bytes
        line.clear();
        for (addr, byte) in (&mut input).take(args.line_width) {
            let byte = byte?;
            line.push((addr, byte));
            if args.break_on.contains(&byte) {
                break;
            }
        }
        !line.is_empty()
    } {
        for element in &format {
            match *element {
                Element::Text(text) => write!(output, "{}", text)?,
                Element::Addr(of) => {
                    if let Some((addr, _)) = line.get(of) {
                        write!(output, "{:06x}", addr)?;
                    } else {
                        write!(output, "      ")?;
                    }
                }
                Element::Value(base, of) => {
                    if let Some((_, byte)) = line.get(of) {
                        base.write(*byte, &mut output)?;
                    } else {
                        base.pad(&mut output)?;
                    }
                }
                Element::Ascii(of) => {
                    if let Some((_, byte @ 32..=126)) = line.get(of) {
                        write!(output, "{}", *byte as char)?;
                    } else {
                        write!(output, " ")?;
                    }
                }
            }
        }
    }
    Ok(())
}
