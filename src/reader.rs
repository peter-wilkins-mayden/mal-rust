use regex::{Regex, Captures};
use crate::types::{MalType, MalErr};

struct RDR {
    tokens: Vec<String>,
    position: usize,
}

impl RDR {
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.position).map(|v| &v[..])
    }
    fn next(&mut self) -> Option<&str> {
        let i = self.position;
        self.position += 1;
        self.tokens.get(i).map(|v| &v[..])
    }

    fn read_form(&mut self) -> Result<MalType, MalErr> {
        match self.peek() {
            Some("'") => {
                self.next();
                Ok(MalType::List(vec![MalType::Symbol("quote".to_string()), self.read_form()?]))
            }
            Some("`") => {
                self.next();
                Ok(MalType::List(vec![MalType::Symbol("quasiquote".to_string()), self.read_form()?]))
            }
            Some("~") => {
                self.next();
                Ok(MalType::List(vec![MalType::Symbol("unquote".to_string()), self.read_form()?]))
            }
            Some("~@") => {
                self.next();
                Ok(MalType::List(vec![MalType::Symbol("splice-unquote".to_string()), self.read_form()?]))
            }
            Some("(") => self.read_seq("(", ")"),
            Some(")") => Err(MalErr::UNEXPECTED(")".to_string())),
            Some("[") => self.read_seq("[", "]"),
            Some("]") => Err(MalErr::UNEXPECTED("]".to_string())),
            Some(_) => self.read_atom(),
            None => Err(MalErr::EOF),
        }
    }

    fn read_seq(&mut self, start: &str, end: &str) -> Result<MalType, MalErr> {
        self.next(); // skip the opening bracket

        let mut res = Vec::new();
        loop {
            match self.peek() {
                Some(")") => {
                    self.next(); // skip closing bracket
                    break;
                }
                None => { return Err(MalErr::EOF); }
                _ => res.push(self.read_form()?)
            }
        };
        Ok(MalType::List(res))
    }
    fn read_atom(&mut self) -> Result<MalType, MalErr> {
        lazy_static! {
            static ref INT_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
            static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
          }
        let t = self.next().unwrap();

        if INT_RE.is_match(&t) {
            Ok(MalType::Num(t.parse().unwrap()))
        } else if STR_RE.is_match(&t) {
            Ok(MalType::String(t.to_string()))
        } else if t.starts_with("\"") {
            Err(MalErr::EOF)
        }
        else {
            Ok(MalType::Symbol(t.to_string()))
        }
    }
}

fn tokenize(s: &str) -> Vec<String> {
    lazy_static!(static ref RE: Regex =
    Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###).unwrap();
);
    let mut res = vec![];
    for cap in RE.captures_iter(s) {
        if cap[1].starts_with(";") { continue; }
        res.push(String::from(&cap[1]));
    }
    res
}

pub fn read_str(input: &str) -> Result<MalType, MalErr> {
    let mut rdr = RDR { tokens: tokenize(input), position: 0 };
    //println!("{:?}", rdr.tokens);
    rdr.read_form()
}

fn unescape_str(s: &str) -> String {
    lazy_static! {
    static ref RE: Regex = Regex::new(r#"\\(.)"#).unwrap();
  }
    RE.replace_all(&s, |caps: &Captures| {
        format!("{}", if &caps[1] == "n" { "\n" } else { &caps[1] })
    }).to_string()
}