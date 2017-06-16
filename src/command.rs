type Error = Box<::std::error::Error>;

// mvp:
// line number to jump to and print that line
// a to append
// i to insert
// d to delete
// c to change

#[derive(Debug,PartialEq)]
pub enum Cmd {
    Cmda,
    Cmdc,
    Cmdd,
    Cmdi,
}

#[derive(Debug,PartialEq)]
pub enum Command {
    Cmd(Cmd),
    No(i64),
}

use nom::{IResult, digit};

named!(line_no<&str, i64>, map_res!(ws!(digit), |s: &str| s.parse::<i64>()));

named!(parse_cmd<&str, Cmd>, alt!(
        tag!("a") => { |_| Cmd::Cmda }
        | tag!("c") => { |_| Cmd::Cmdc }
        | tag!("d") => { |_| Cmd::Cmdd }
        | tag!("i") => { |_| Cmd::Cmdi }));

named!(parse_command<&str, Command>, alt!(
        parse_cmd => { Command::Cmd }
        | line_no => { Command::No }));

// Entry point to the parser
pub fn handle_command(s: &str) -> Result<Command, Error> {
    let res = parse_command(s);
    if let IResult::Done(_, parsed) = res {
        Ok(parsed)
    } else {
        Err(From::from(format!("{:?}", res)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd() {
        assert_eq!(parse_cmd("a"), IResult::Done(&""[..], Cmd::A));
        assert_matches!(parse_cmd("b"), IResult::Error(_));
    }
}

/*
pub enum AtomicAddr {
    Absolute(i64),
    Current,
    Last,
    NextRE(String),
    PrevRE(String),
    Tick(char),
}

pub struct Addr(AtomicAddr, i64);

pub enum AddrRange {
    From(Addr),
    FromTo(Addr, Addr),
}

pub struct Command<'a> {
    from_addr: Option<&'a str>,
    to_addr: Option<&'a str>,
    command: Option<&'a str>,
}
*/

