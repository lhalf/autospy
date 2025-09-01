# autospy

###### *🎵 autospy record, autospy replace 🎵*

[![Crates.io Version](https://img.shields.io/crates/v/autospy)](https://crates.io/crates/autospy)
[![docs.rs](https://img.shields.io/docsrs/autospy)](https://docs.rs/autospy/latest/autospy/)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/autospy/on_commit.yml)](https://github.com/lhalf/autospy/actions/workflows/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

A test spy object library.

## Overview

A test spy is a type of [test double](https://en.wikipedia.org/wiki/Test_double) used in unit testing. It provides the same
interface as the production code, but allows you to set outputs before use in a test
and to verify input parameters after the spy has been used.

[`#[autospy]`](https://docs.rs/autospy/latest/autospy/) generates a test spy object for traits.

## Usage

The example below demonstrates use in a unit test assuming `autospy` is included in `[dev-dependencies]`.

```rust
#[cfg_attr(test, autospy::autospy)]
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
        let spy = MyTraitSpy::default(); // build spy
        
        spy.foo.returns.push_back(true); // set the return values

        assert!(use_trait(spy.clone())); // use the spy
        assert_eq!(vec![10], spy.foo.arguments.take_all()) // verify the arguments passed
    }
}
```

For additional examples and features see the [docs](https://docs.rs/autospy).

## Acknowledgements

Autospy is heavily influenced by the excellent [mockall](https://docs.rs/mockall/latest/mockall/) crate, which,
through [automock](https://docs.rs/mockall/latest/mockall/attr.automock.html), provides many similar features. 

Autospy aims to offer these features through a macro-generated spy object, rather than a mock object. The use of either is
largely personal preference; however, there are some advantages to using a spy object:

| Test object | Test failures                                              | Test structure                                            | Complexity                                                |
|-------------|------------------------------------------------------------|-----------------------------------------------------------|-----------------------------------------------------------|
| Mock        | Panics if expectations fail; error messages can be unclear | Less standard pattern, expectations are baked into object | More crate-specific syntax and usage patterns             |
| Spy         | Asserts like any regular test                              | Assert after use, more standard test pattern              | Simple: set what's returned, then inspect what was called |
