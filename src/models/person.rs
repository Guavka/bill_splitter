#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Person {
    name: String,
    surname: String,
}

impl Person {
    pub fn new(name: String, surname: String) -> Person {
        Person { name, surname }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_surname(&self) -> String {
        self.surname.clone()
    }
}
