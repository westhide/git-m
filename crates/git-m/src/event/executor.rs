pub trait Executor {
    type Event;

    fn execute(&self, event: Self::Event);
}
