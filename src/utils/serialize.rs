use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

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