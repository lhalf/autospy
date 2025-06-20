# autospy

###### *🎵 autospy record, autospy replace 🎵*

[![Crates.io Version](https://img.shields.io/crates/v/autospy)](https://crates.io/crates/autospy)
[![docs.rs](https://img.shields.io/docsrs/autospy)](https://docs.rs/autospy/latest/autospy/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/autospy/on_commit.yml)](https://github.com/lhalf/autospy/actions/workflows/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

A test spy object library.

## Overview

Spies are a type of [test double](https://en.wikipedia.org/wiki/Test_double) used for unit testing software. A test spy
is an object providing the same interface as the production code, but which allows setting of the output before a test
runs and verification of input parameters after the test run.

[`#[autospy]`](https://docs.rs/autospy/latest/autospy/) generates a test spy object for traits.

## Usage

Spy objects are often only used by unit tests, the example below demonstrates use in a unit test assuming included in
`[dev-dependencies]`.

```rust
#[cfg(test)]
use autospy::autospy;

#[cfg_attr(test, autospy)]
trait MyTrait {
    fn foo(&self, x: u32) -> bool;
}

fn use_trait(trait_object: impl MyTrait) -> bool {
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
