use std::ops::Deref;

pub struct Glop {
    pub matches: Vec<Match>,
}

pub struct Match {
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
}

pub type Identifier = Vec<String>;

pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    Message(String),
}

pub enum CmpOpcode {
    Equal,
    NotEqual,
}

pub enum Action {
    SetVar(Identifier, String),
    UnsetVar(Identifier),
    Acknowledge(String),
    Exec(String),
    Script(String),
}

use std::fmt;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Action::SetVar(ref k, ref v) => write!(f, "set {} {};", FmtIdentifier(k), v),
            &Action::UnsetVar(ref k) => write!(f, "unset {};", FmtIdentifier(k)),
            &Action::Acknowledge(ref k) => write!(f, "acknowledge {};", k),
            &Action::Exec(ref v) => write!(f, r#"exec {};"#, v),
            &Action::Script(ref v) => write!(f, r#"script {}!#"#, v),
        }
    }
}

struct FmtActions<'a>(&'a Vec<Action>);

impl<'a> Deref for FmtActions<'a> {
    type Target = Vec<Action>;

    fn deref(&self) -> &Vec<Action> {
        self.0
    }
}

impl<'a> fmt::Display for FmtActions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.iter() {
            try!(writeln!(f, "    {}", i));
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
            &Condition::Cmp(ref l, ref op, ref r) => write!(f, "{} {} {}", FmtIdentifier(l), op, r),
            &Condition::IsSet(ref k) => write!(f, "is_set {}", FmtIdentifier(k)),
            &Condition::Message(ref k) => write!(f, "message {}", k),
        }
    }
}

struct FmtConditions<'a>(&'a Vec<Condition>);

impl<'a> Deref for FmtConditions<'a> {
    type Target = Vec<Condition>;

    fn deref(&self) -> &Vec<Condition> {
        self.0
    }
}

impl<'a> fmt::Display for FmtConditions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.len();
        for i in 0..n {
            if i > 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}", self[i]));
        }
        Ok(())
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "match ({}) {{\n{}}}",
                 FmtConditions(&self.conditions),
                 FmtActions(&self.actions))
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

struct FmtIdentifier<'a>(&'a Identifier);

impl<'a> fmt::Display for FmtIdentifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join("."))
    }
}
