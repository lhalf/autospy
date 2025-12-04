#![deny(clippy::pedantic)]

//! expanded macro does not trigger any pedantic clippy lints
#[autospy::autospy]
#[allow(dead_code)]
trait MyTrait {
    fn function(&self);
}
