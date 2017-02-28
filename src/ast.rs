use std::collections::HashSet;
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Glop {
    pub matches: Vec<Match>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Match {
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub acting_roles: HashSet<String>,
}

pub fn acting_roles(conditions: &Vec<Condition>) -> HashSet<String> {
    conditions.iter()
        .map(|c| {
            if let &Condition::Message { topic: _, peer_role: _, ref acting_role } = c {
                if let &Some(ref role) = acting_role {
                    return Some(role.to_string());
                }
            }
            None
        })
        .filter(|maybe_role| if let &Some(_) = maybe_role {
            true
        } else {
            false
        })
        .map(|maybe_role| maybe_role.unwrap().to_string())
        .collect()
}

pub type Identifier = Vec<String>;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum Condition {
    Cmp(Identifier, CmpOpcode, String),
    IsSet(Identifier),
    IsUnset(Identifier),
    Message {
        topic: String,
        peer_role: Option<String>,
        acting_role: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum CmpOpcode {
    Equal,
    NotEqual,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum Action {
    SetVar(Identifier, String),
    UnsetVar(Identifier),
    PopMsg(String),
    Script(String),
}

use std::fmt;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Action::SetVar(ref k, ref v) => write!(f, "var set {} {};", FmtIdentifier(k), v),
            &Action::UnsetVar(ref k) => write!(f, "var unset {};", FmtIdentifier(k)),
            &Action::PopMsg(ref topic) => write!(f, "msg pop {};", topic),
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
            &Condition::IsUnset(ref k) => write!(f, "is_unset {}", FmtIdentifier(k)),
            &Condition::Message { ref topic, ref peer_role, ref acting_role } => {
                write!(f, "message {}", topic)?;
                if let &Some(ref peer) = peer_role {
                    write!(f, " from {}", peer)?;
                }
                if let &Some(ref acting) = acting_role {
                    write!(f, " as {}", acting)?;
                }
                Ok(())
            }
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
                 "when ({}) {{\n{}}}",
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
