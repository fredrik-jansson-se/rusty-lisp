#[macro_use]
extern crate nom;
extern crate rustyline;

// http://norvig.com/lispy.html
// https://en.wikibooks.org/wiki/Write_Yourself_a_Scheme_in_48_Hours/Parsing

use nom::{space, digit, alpha};
use nom::types::CompleteStr;

// use std::str;
// use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[derive(Debug, PartialEq)]
enum LispVal {
    // Atom(String),
    Number(i64),
    // FNumber(f64),
    // List(Vec<LispVal>),
    String(String),
    // Bool(bool)
}

// named!(string<&str, LispVal>,
//        do_parse!(
//            s: escaped_transform!
//

// named!(boolean<&str, LispVal>,
//        do_parse!(
//            tag!("#") >>
//            v: alt(tag!("t") | tag!("f")) >>
//            ( LispVal::Bool(v == "t"))
//            )
//        );
fn p_u64(s: CompleteStr) -> Result<i64, std::num::ParseIntError> {
    s.parse()
}

named!(number_parser<CompleteStr, LispVal>,
       do_parse!(
           s: opt!(char!('-')) >>
           n: map_res!(digit, p_u64) >>
           (LispVal::Number(if s.is_some() {-n} else {n}))
           )
       );

named!(string_parser<CompleteStr, LispVal>,
       do_parse!(
           char!('"') >>
           s: take_until_and_consume!(r#"""#) >>
           (LispVal::String(s.to_string()))
           ));

named!(name_parser<nom::types::CompleteStr, nom::types::CompleteStr>,
       do_parse!(
           tag!("Hello") >>
           space >>
           // name: map_res!(
           //     nom::alpha,
           //     str::from_utf8
           //     ) >>
           name: alpha >> 
           (name)
           )
       );


fn main() {
    // println!("{:?}", number_parser(CompleteStr("-123")));
    // println!("{:?}", number_parser(CompleteStr("456")));
    // println!("{:?}", name_parser(nom::types::CompleteStr("Hello Fredrik")));

    println!("{:?}", string_parser(CompleteStr("\"foo\"")));
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_ref());
                // println!("Parse: {:?}", name_parser(line.as_bytes()));
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
    println!("Done");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_string() {
        let s = r#"foo"#;
        let rv = Ok((CompleteStr(""), LispVal::String("foo".to_string())));
        assert_eq!(rv, string_parser(CompleteStr("\"foo\"")));
    }

    #[test]
    fn parse_number() {
        assert_eq!(Ok((CompleteStr(""), LispVal::Number(123))), number_parser(CompleteStr("123")));
        assert_eq!(Ok((CompleteStr(""), LispVal::Number(-123))), number_parser(CompleteStr("-123")));
    }

}
