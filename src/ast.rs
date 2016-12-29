pub struct Glop {
    pub matches: Vec<Box<Match>>,
}

pub struct Match {
    pub conditions: Vec<Box<Condition>>,
    pub actions: Vec<Box<Action>>,
}

pub enum Condition {
    Cmp(String, CmpOpcode, String),
    IsSet(String),
    Message(String),
}

pub enum CmpOpcode {
    Equal,
    NotEqual,
}

pub enum Action {
    SetVar(String, String),
    UnsetVar(String),
    Acknowledge(String),
    Exec(String),
    Script(String),
}

use std::fmt;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Action::SetVar(ref k, ref v) => write!(f, "set {} {}", k, v),
            &Action::UnsetVar(ref k) => write!(f, "unset {}", k),
            &Action::Acknowledge(ref k) => write!(f, "acknowledge {}", k),
            &Action::Exec(ref v) => write!(f, r#"exec "{}""#, v),
            &Action::Script(ref v) => write!(f, r#"script "{}""#, v),
        }
    }
}

struct Actions<'a>(&'a Vec<Box<Action>>);

impl<'a> fmt::Display for Actions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.0 {
            try!(writeln!(f, "  {};", i));
        }
        Ok(())
    }
}

impl fmt::Display for CmpOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CmpOpcode::Equal => write!(f, "=="),
            &CmpOpcode::NotEqual => write!(f, "!="),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Condition::Cmp(ref l, ref op, ref r) => write!(f, "{} {} {}", l, op, r),
            &Condition::IsSet(ref k) => write!(f, "is_set {}", k),
            &Condition::Message(ref k) => write!(f, "message {}", k),
        }
    }
}

struct Conditions<'a>(&'a Vec<Box<Condition>>);

impl<'a> fmt::Display for Conditions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = *&self.0.len();
        for i in 0..n {
            if i > 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}", &self.0[i]));
        }
        Ok(())
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "match ({}) {{\n{}}}",
                 Conditions(&self.conditions),
                 Actions(&self.actions))
    }
}

impl fmt::Display for Glop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for m in &self.matches {
            try!(writeln!(f, "{}", m));
        }
        Ok(())
    }
}
