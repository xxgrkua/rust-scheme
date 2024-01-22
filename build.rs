use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

const ASCII_LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_INITIALS: &'static str = "!$%&*/:<=>?^_~";
const DIGITS: &'static str = "0123456789";
const HEX_DIGITS: &'static str = "0123456789abcdef";
const OCTAL_DIGITS: &'static str = "01234567";
const BINARY_DIGITS: &'static str = "01";
const VERTICAL_LINE: &'static str = "|";
const WHITESPACE: &'static str = " \t\n\r";
const DELIMITERS: &'static str = "()\";";
const RESERVED: &'static str = "[]{}|";
const EXACTNESS: [&'static str; 2] = ["#i", "#e"];
const RADIX: [&'static str; 4] = ["#b", "#o", "#d", "#x"];
const BOOLEAN: [&'static str; 4] = ["#t", "#f", "#true", "#false"];
const PECULIAR_IDENTIFIERS: [&'static str; 3] = ["...", "+", "-"];
const SPECIAL_SUBSEQUENTS: &'static str = "+-@.";
const NEWLINES: &'static str = "\n\r";

fn gen_set_from_iterator<'a, I>(iter: I) -> phf_codegen::Set<&'a str>
where
    I: Iterator<Item = &'a str>,
{
    let mut set = phf_codegen::Set::new();
    for string in iter {
        set.entry(string);
    }
    set
}

fn gen_set_from_slice<'a>(slice: &[&'a str]) -> phf_codegen::Set<&'a str> {
    let mut set = phf_codegen::Set::new();
    for &string in slice {
        for i in 0..string.len() {
            set.entry(&string[i..i + 1]);
        }
    }
    set
}

fn gen_token_set_code(out_dir: &Path) -> PathBuf {
    let path = out_dir.join("tokens.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());
    let whitespace = gen_set_from_slice(&[WHITESPACE]);
    let delimiter = gen_set_from_slice(&[DELIMITERS, WHITESPACE]);
    let initial = gen_set_from_slice(&[ASCII_LETTERS, SPECIAL_INITIALS]);
    let subsequent =
        gen_set_from_slice(&[ASCII_LETTERS, SPECIAL_INITIALS, DIGITS, SPECIAL_SUBSEQUENTS]);
    let peculiar_identifier = gen_set_from_iterator(PECULIAR_IDENTIFIERS.into_iter());
    let newline = gen_set_from_slice(&[NEWLINES]);
    let digit = gen_set_from_slice(&[DIGITS]);
    write!(
        file,
        "const WHITESPACE: phf::Set<&'static str> = {};\n",
        whitespace.build()
    )
    .unwrap();
    write!(
        file,
        "const INITIAL: phf::Set<&'static str> = {};\n",
        initial.build()
    )
    .unwrap();
    write!(
        file,
        "const DELIMITER: phf::Set<&'static str> = {};\n",
        delimiter.build()
    )
    .unwrap();
    write!(
        file,
        "const PECULIAR_IDENTIFIER: phf::Set<&'static str> = {};\n",
        peculiar_identifier.build()
    )
    .unwrap();
    write!(
        file,
        "const SUBSEQUENT: phf::Set<&'static str> = {};\n",
        subsequent.build()
    )
    .unwrap();
    write!(
        file,
        "const NEWLINE: phf::Set<&'static str> = {};\n",
        newline.build()
    )
    .unwrap();
    write!(
        file,
        "const DIGIT: phf::Set<&'static str> = {};\n",
        digit.build()
    )
    .unwrap();

    path
}

fn main() {
    let out_dir_str = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);
    gen_token_set_code(out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}
