use clap::Parser;
use std::{
    collections::HashSet,
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};
#[derive(Parser, Debug)]
#[command(author = "xyve", version = "0.1.0", about = "mangles a given wordlist", long_about = None)]
struct ManglerArgs {
    #[arg(short, long, required = true)]
    /// path of the wordlist
    file: PathBuf,
    #[arg(short, long, required = true)]
    /// path of the output mangled list
    output: PathBuf,
    #[arg(short, long, default_value_t = false)]
    /// duplicate words per line
    double: bool,
    #[arg(short, long, default_value_t = false)]
    /// capitalize words
    capital: bool,
    #[arg(short, long, default_value_t = false)]
    /// swap case words
    swap: bool,
    #[arg(short, long, default_value_t = false)]
    /// add ed to the end of each word
    ed: bool,
    #[arg(short, long, default_value_t = false)]
    /// add ing to the end of each word
    ing: bool,
    #[arg(short, long, default_value_t = false)]
    /// disables uppercasing words
    upper: bool,
    #[arg(short, long, default_value_t = false)]
    /// disables lowercasing words
    lower: bool,
    #[arg(short, long, default_value_t = false)]
    /// disables reversing words
    reverse: bool,
    #[arg(long, default_value_t = false)]
    /// adds an assortment of punctuation to the end of words
    punctuation: bool,
    #[arg(short, long, default_value_t = false)]
    /// prefixes and postfixes the word with 1990..=2023
    years: bool,
    #[arg(long, default_value_t = false)]
    /// postfixes the word with 1..=123
    na: bool,
    #[arg(long, default_value_t = false)]
    /// prefixes the word with 1..=123
    nb: bool,
    #[arg(long, default_value_t = false)]
    /// postfixes the word with 01..=09
    pna: bool,
    #[arg(long, default_value_t = false)]
    /// prefixes the word with 01..=09
    pnb: bool,
    #[arg(short = 'C', long, default_value_t = false)]
    /// prefixes and postfixes the word with common words like pw, pwd, admin, and sys
    common: bool,
}
fn mangler(args: &ManglerArgs) -> Result<(), Box<dyn Error>> {
    let input = File::open(&args.file)?;
    let mut output = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(&args.output)?,
    );

    let lines: Result<Vec<String>, _> = BufReader::new(input).lines().collect();
    let mut result: Vec<String> = Vec::new();

    for l in lines? {
        if args.double {
            result.push(format!("{}{}", l, l));
        }
        if args.reverse {
            result.push(l.chars().rev().collect::<String>());
        }
        if args.capital {
            if let Some(ch) = l.chars().next() {
                result.push(ch.to_uppercase().chain(l.chars().skip(1)).collect());
            } else {
                result.push(l.clone());
            }
        }
        if args.lower {
            result.push(l.to_lowercase());
        }
        if args.upper {
            result.push(l.to_uppercase());
        }
        if args.swap {
            result.push(
                l.chars()
                    .map(|ch| {
                        if ch.is_uppercase() {
                            ch.to_lowercase().to_string()
                        } else if ch.is_lowercase() {
                            ch.to_uppercase().to_string()
                        } else {
                            ch.to_string()
                        }
                    })
                    .collect(),
            );
        }
        if args.ed {
            result.push(format!("{}ed", l));
        }
        if args.ing {
            result.push(format!("{}ing", l));
        }
        if args.common {
            for word in ["pw", "pwd", "admin", "sys"] {
                result.push(format!("{}{}", word, l));
                result.push(format!("{}{}", l, word));
            }
        }
        if args.punctuation {
            for punc in "!@$%^&*()".chars() {
                result.push(format!("{}{}", l, punc));
            }
        }
        if args.years {
            for year in 1990..=2023 {
                result.push(format!("{}{}", year, l));
                result.push(format!("{}{}", l, year));
            }
        }
        if args.pnb || args.pnb {
            for i in 1..=9 {
                if args.pnb {
                    result.push(format!("0{}{}", i, l));
                }
                if args.pna {
                    result.push(format!("{}0{}", l, i));
                }
            }
        }
        if args.nb || args.nb {
            for i in 1..=123 {
                if args.nb {
                    result.push(format!("{}{}", i, l));
                }
                if args.na {
                    result.push(format!("{}{}", l, i));
                }
            }
        }
        let mut found = HashSet::new();
        result.retain(|item| found.insert(item.clone()));

        for line in result.iter() {
            output.write(line.as_ref())?;
            output.write(b"\n")?;
        }

        result.clear();
    }
    Ok(())
}
fn main() {
    let args = ManglerArgs::parse();
    if let Err(err) = mangler(&args) {
        eprintln!("mangler: {}", err);
    }
}
