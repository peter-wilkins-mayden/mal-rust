pub enum MalType {
    List(Vec<MalType>),
    Num(f64),
    Symbol(String),
    String(String),
}

pub enum MalErr {
    EOF,
    UNEXPECTED(String),
}