use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

pub fn write_vector_to_file<T: Serialize>(filename: &str, data: &[T]) -> io::Result<()> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn read_vector_from_file<T: DeserializeOwned>(filename: &str) -> io::Result<Vec<T>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn write_hashmap_to_file<K, V>(filename: &str, data: &HashMap<K, V>) -> io::Result<()>
where
    K: Serialize + std::hash::Hash + Eq,
    V: Serialize,
{
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn read_hashmap_from_file<K, V>(filename: &str) -> io::Result<HashMap<K, V>>
where
    K: DeserializeOwned + std::hash::Hash + Eq,
    V: DeserializeOwned,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}