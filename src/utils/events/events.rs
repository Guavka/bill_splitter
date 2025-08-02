pub trait Listener {
    fn notify(&self);
}

pub struct Event<'a> {
    listeners: Vec<&'a dyn Listener>,
    is_init: bool,
}

impl<'a> Event<'a> {
    pub fn new() -> Self {
        Event {
            listeners: Vec::new(),
            is_init: true,
        }
    }
    pub fn invoke(&self) {
        if !self.is_init {
            panic!("Event not init! Init with \"Event::new() method\"")
        }
        for listener in &self.listeners {
            listener.notify();
        }
    }
    pub fn subscribe(&mut self, other: &'a dyn Listener) {
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
    pub fn unsubscribe(&mut self, other: &'a dyn Listener) {
        if !self.is_init {
            panic!("Event not init! Init with \"Event::new() method\"")
        }

        // Удаление слушателя
        self.listeners
            .retain(|&listener| listener as *const _ != other as *const _);
    }
}
