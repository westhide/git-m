use std::fmt::Debug;

pub trait Executor: Debug + Clone {
    type Event;

    type Return;

    fn execute(&self, event: Self::Event) -> Self::Return;
}
