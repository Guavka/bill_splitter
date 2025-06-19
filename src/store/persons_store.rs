use crate::store::models::mock::mock_persons::get_mock_persons;
use crate::store::models::person::person::Person;
use crate::store::models::person::short_person::ShortPerson;
use crate::utils::serialize::{merge_map, read_hashmap_from_file, write_hashmap_to_file};
use crate::IS_DEBUG;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

/// Статическая переменная с ленивой инициализацией для хранения единственного экземпляра `PersonsStore`.
static SINGLETON: Lazy<Mutex<PersonsStore>> = Lazy::new(|| {
    let mut persons = PersonsStore {
        state: HashMap::new(),
    };
    persons.load_persons(); // Загружаем пользователей при инициализации
    Mutex::new(persons) // Оборачиваем в мьютекс для безопасного доступа из нескольких потоков
});

/// Структура для хранения информации о пользователях.
pub struct PersonsStore {
    state: HashMap<String, Person>, // Хранит пользователей по их уникальным идентификаторам
}

impl PersonsStore {
    /// Получает список всех пользователей в виде вектора `ShortPerson`.
    ///
    /// # Returns
    /// Вектор `Vec<ShortPerson>` с краткой информацией о каждом пользователе.
    pub fn get_persons_short(&self) -> Vec<ShortPerson> {
        self.state
            .values()
            .map(|person| ShortPerson {
                id: person.get_id(),
                name: person.get_name(),
                surname: person.get_surname(),
            })
            .collect()
    }

    pub fn is_exits(&self, name: String, surname: String) -> bool {
        self.state
            .values()
            .any(|value| value.get_surname() == surname && value.get_name() == name)
    }

    /// Получает пользователя по его уникальному идентификатору.
    ///
    /// # Arguments
    /// * `id` - Уникальный идентификатор пользователя.
    ///
    /// # Returns
    /// `Option<&Person>` - `Some(&Person)` если пользователь найден, иначе `None`.
    pub fn get_person(&mut self, id: &String) -> Option<&mut Person> {
        self.state.get_mut(id)
    }

    /// Удаляет пользователя по его уникальному идентификатору.
    ///
    /// # Arguments
    /// * `id` - Уникальный идентификатор пользователя, которого нужно удалить.
    pub fn remove_person(&mut self, id: String) {
        let _ = self.state.remove(&id); // Удаляем пользователя из состояния
        self.save_persons(); // Сохраняем изменения в файл
    }

    /// Добавляет нового пользователя в хранилище.
    ///
    /// # Arguments
    /// * `person` - Объект `Person`, который нужно добавить.
    pub fn add_person(&mut self, person: Person) {
        let _ = self.state.insert(person.get_id(), person); // Добавляем пользователя в состояние
        self.save_persons(); // Сохраняем изменения в файл
    }

    /// Сохраняет текущее состояние пользователей в файл.
    pub fn save_persons(&self) {
        write_hashmap_to_file("persons", &self.state).unwrap(); // Записываем состояние в файл
    }

    /// Загружает пользователей из файла в состояние.
    fn load_persons(&mut self) {
        if IS_DEBUG {
            self.state = get_mock_persons().unwrap(); // Загружаем тестовые данные в режиме отладки
            self.save_persons(); // Сохраняем тестовые данные в файл
        } else {
            match read_hashmap_from_file("persons") {
                Ok(values) => merge_map(&mut self.state, values), // Объединяем загруженные данные с текущими
                _ => self.save_persons(), // Если не удалось загрузить, сохраняем текущее состояние
            }
        }
    }
}

/// Получает доступ к единственному экземпляру `PersonsStore` с блокировкой.
pub fn use_person_store() -> MutexGuard<'static, PersonsStore> {
    SINGLETON.try_lock().unwrap()
}
