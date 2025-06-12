use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

pub fn merge_map<K, V>(map_base: &mut HashMap<K, V>, map_second: HashMap<K, V>)
where
    K: Eq,
    K: Hash,
{
    map_base.clear();

    // Вставляем новые данные
    for (key, value) in map_second {
        map_base.insert(key, value);
    }
}

/// Функция для записи данных из `Vec<T>` в формате JSON в файл.
///
/// # Аргументы
///
/// * `filename` - Имя файла, в который будут записаны данные.
/// * `data` - Вектор с данными, которые нужно записать.
///
/// # Возвращаемое значение
///
/// Возвращает `io::Result<()>`, которое указывает на успешность операции.
pub fn write_vector_to_file<T: Serialize>(filename: &str, data: &[T]) -> io::Result<()> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

use std::collections::HashMap;
use std::hash::Hash;

/// Записывает HashMap в файл в формате JSON
///
/// # Параметры
/// - `filename`: путь к файлу для записи
/// - `data`: ссылка на HashMap для сериализации и записи
///
/// # Возвращаемое значение
/// Результат операции записи (Ok() или io::Error)
pub fn write_hashmap_to_file<K, V>(filename: &str, data: &HashMap<K, V>) -> io::Result<()>
where
    K: Serialize + std::hash::Hash + Eq,
    V: Serialize,
{
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

/// Читает HashMap из файла в формате JSON
///
/// # Параметры
/// - `filename`: путь к файлу для чтения
///
/// # Возвращаемое значение
/// Результат операции чтения, содержащий HashMap или io::Error
pub fn read_hashmap_from_file<K, V>(filename: &str) -> io::Result<HashMap<K, V>>
where
    K: DeserializeOwned + std::hash::Hash + Eq,
    V: DeserializeOwned,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(data)
}

/// Функция для чтения данных из `Vec<T>` из файла формата JSON.
///
/// # Аргументы
///
/// * `filename` - Имя файла, из которого будут прочитаны данные.
///
/// # Возвращаемое значение
///
/// Возвращает `io::Result<Vec<T>>`, содержащий вектор данных указанного типа из файла.
pub fn read_vector_from_file<T: DeserializeOwned>(filename: &str) -> io::Result<Vec<T>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
