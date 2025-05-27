# autospy

![Crates.io Version](https://img.shields.io/crates/v/autospy)
![docs.rs](https://img.shields.io/docsrs/autospy)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

A test spy object library.

## Overview

Spies are a type of [test double](https://en.wikipedia.org/wiki/Test_double) used for unit testing software. A test spy
is an object providing the same interface as the production code, but which allows setting of the output before a test
runs and verification of input parameters after the test run.

## Usage

Spy objects are often only used by unit tests, the example below demonstrates use in a unit test.

```rust
use autospy::autospy;

#[cfg_attr(test, autospy)]
trait MyTrait {
    fn foo(&self, x: u32) -> bool;
}

fn use_trait<T: MyTrait>(trait_object: T) -> bool {
    trait_object.foo(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait() {
        let spy = MyTraitSpy::default();
        spy.foo.returns.push_back(true);

        assert!(use_trait(spy.clone()));
        assert_eq!(vec![10], spy.foo.arguments.take_all())
    }
}
```

For additional examples and features see the [docs](https://docs.rs/autospy).

## Acknowledgements

Autospy is heavily influenced by the excellent [mockall](https://docs.rs/mockall/latest/mockall/) crate, which
through [automock](https://docs.rs/mockall/latest/mockall/attr.automock.html) provides much the same features. Autospy
aims to offer these same features through a macro generated spy object, rather than a mock object. The use of either is
largely personal preference; however, there are some benefits to using a spy object. Notably, a mock object will panic
if its expectations fail which can cause less legible error messages.
