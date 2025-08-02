pub trait ParamListener<T> {
    fn notify(&self, value: T);
}

pub struct ParamEvent<'a, T> {
    listeners: Vec<&'a dyn ParamListener<T>>,
    is_init: bool,
}

impl<'a, T: Clone> ParamEvent<'a, T> {
    pub fn new() -> Self {
        ParamEvent {
            listeners: Vec::new(),
            is_init: true,
        }
    }
    pub fn invoke(&self, value: T) {
        if !self.is_init {
            panic!("Event not init! Init with \"Event::new() method\"")
        }
        for listener in &self.listeners {
            listener.notify(value.clone());
        }
    }
    pub fn subscribe(&mut self, other: &'a dyn ParamListener<T>) {
        if !self.is_init {
            panic!("Event not init! Init with \"Event::new() method\"")
        }

        // Проверка на наличие слушателя
        if self
            .listeners
            .iter()
            .any(|&listener| listener as *const _ == other as *const _)
        {
            return;
        }

        self.listeners.push(other);
    }
    pub fn unsubscribe(&mut self, other: &'a dyn ParamListener<T>) {
        if !self.is_init {
            panic!("Event not init! Init with \"Event::new() method\"")
        }

        // Удаление слушателя
        self.listeners
            .retain(|&listener| listener as *const _ != other as *const _);
    }
}
