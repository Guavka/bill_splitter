use crate::store::models::person::Person;
use std::collections::HashMap;

pub fn get_mock_persons() -> HashMap<String, Person> {
    let mut persons = HashMap::new();
    persons.insert("test1".to_string(), Person::new("test1", "testtest").unwrap());
    persons.insert("test2".to_string(), Person::new("test2", "testtest").unwrap());
    persons.insert("test3".to_string(), Person::new("test3", "testtest").unwrap());
    persons.insert("test4".to_string(), Person::new("test4", "testtest").unwrap());
    persons.insert("test5".to_string(), Person::new("test5", "testtest").unwrap());
    persons
}
