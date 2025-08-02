pub trait Props {}
pub trait Events {}

pub trait Menu {
    fn show(&self);
}

pub struct Component<P: Props, E: Events> {
    pub props: Option<P>,
    pub events: Option<E>,
}

impl<P: Props, E: Events> Component<P, E> {
    pub fn new(&self, props: Option<P>, events: Option<E>) -> Self {
        Self { props, events }
    }
}

pub struct EmptyEvents;
impl Events for EmptyEvents {}

pub struct EmptyProps;
impl Props for EmptyProps {}
