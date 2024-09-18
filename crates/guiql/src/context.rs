pub trait Context {}

pub trait HasContext {
    fn context(&self) -> impl Context;
}
