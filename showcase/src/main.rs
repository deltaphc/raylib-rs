use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, line_ending, space1};
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn parse_space(input: &str) -> IResult<&str, &str> {
    space1(input)
}

fn parse_line_ending(input: &str) -> IResult<&str, &str> {
    line_ending(input)
}

fn parse_quote(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_until("\""), char('"'))(input)
}

fn parse_include(input: &str) -> IResult<&str, &str> {
    preceded(tag("#include "), parse_quote)(input)
}

fn main() {
    let source = include_str!("../original/core/core_basic_window.c");

    let print_line = |i: usize, str: &str| println!("{}:{}", i, str);

    let mut lines = source.lines().enumerate();

    'top: loop {
        let line = lines.next();
        if line.is_none() {
            return;
        }
        let (line_number, line) = line.unwrap();

        if line.is_empty() {
            println!("");
            continue;
        }

        if let Ok((_, file)) = parse_include(line) {
            match file {
                "raylib.h" => print_line(line_number, "use raylib::prelude::*;"),
                _ => unimplemented!(),
            }
            continue;
        }

        if line.starts_with("//") {
            print_line(line_number, line);
            continue;
        }
        if line.starts_with("/*") {
            print_line(line_number, line);
            if line.ends_with("*/") {
                continue;
            }
            while let Some((_, next_line)) = lines.next() {
                println!("{}", next_line);
                if next_line.ends_with("*/") {
                    continue 'top;
                }
            }
        }
        print_line(line_number, line);
    }
}
