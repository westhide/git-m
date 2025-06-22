use std::fmt::Debug;

pub trait Execute: Debug {
    type Event;

    type Return;

    fn execute(&mut self, event: Self::Event) -> impl Future<Output = Self::Return> + Send;
}
