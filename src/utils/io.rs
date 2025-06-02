use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::process::Command;
use std::str::FromStr;

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

/// Функция для очистки консоли.
///
/// Эта функция очищает консольный экран в зависимости от операционной системы.
pub fn clear_console() {
    if cfg!(target_os = "windows") {
        // Для Windows
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Не удалось очистить консоль");
    } else {
        // Для Unix-подобных систем (Linux, macOS)
        Command::new("clear")
            .status()
            .expect("Не удалось очистить консоль");
    }
}

/// Функция для получения значения из консоли.
///
/// Эта функция использует обобщенные типы, чтобы можно было получать разные типы данных,
/// такие как `u8` или `f64`, при условии, что тип реализует трейт `std::str::FromStr`.
///
/// # Аргументы
///
/// * `msg` - Сообщение, которое будет выведено пользователю перед вводом значения.
/// * `error_msg` - Сообщение, которое будет выведено в случае ошибки при вводе.
/// * `check_func` - Необязательная функция для дополнительной проверки введенного значения.
///
/// # Возвращаемое значение
///
/// Возвращает значение типа `T`, которое было введено пользователем и успешно обработано.
pub fn get_number_console<T>(
    msg: &str,
    error_msg: &str,
    check_func: Option<Box<dyn Fn(T) -> bool>>,
) -> T
where
    T: FromStr + Copy,
{
    loop {
        println!("{}", msg);
        let mut input = String::new();

        // Чтение строки из консоли
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", error_msg);
            continue;
        }

        // Попытка парсинга введенной строки в тип T
        match input.trim().parse::<T>() {
            Ok(value) => {
                // Если предоставлена функция проверки, вызываем её
                if let Some(ref func) = check_func {
                    if !func(value) {
                        // Вызываем функцию
                        continue;
                    }
                }
                return value; // Возвращаем успешно полученное значение
            }
            Err(_) => println!("{}", error_msg), // Обработка ошибки парсинга
        }
    }
}

pub fn get_string_console(
    msg: &str,
    error_msg: &str,
    check_func: Option<Box<dyn Fn(&String) -> bool>>,
) -> String {
    loop {
        println!("{}", msg);
        let mut input = String::new();

        // Чтение строки из консоли
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", error_msg);
            continue;
        }

        let input = input.trim().to_string();
        // Если предоставлена функция проверки, вызываем её
        if let Some(ref func) = check_func {
            if !func(&input) {
                continue;
            }
        }
        return input; // Возвращаем успешно полученное значение
    }
}
