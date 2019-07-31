use crate::types::MalType;

pub fn pr_str(ast: MalType) -> String {
    match ast {
        MalType::Symbol(name) => name,
        MalType::Num(n) => n.to_string(),
        MalType::List(xs) => {
           format!("({})", xs.into_iter().map(|dt| pr_str(dt)).collect::<Vec<String>>().join(" "))
        }
        MalType::String(s) => s,
    }
}