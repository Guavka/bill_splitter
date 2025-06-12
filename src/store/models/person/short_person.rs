use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct ShortPerson {
    pub id: String,
    pub name: String,
    pub surname: String,
}

impl fmt::Display for ShortPerson {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.surname)
    }
}
