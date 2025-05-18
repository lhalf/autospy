# autospy

![Crates.io Version](https://img.shields.io/crates/v/autospy)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

A test spy object library.

## Overview

Spies are a type of [test double](https://en.m.wikipedia.org/wiki/Test_double) used for unit testing software. A test spy is an object providing the same interface as the production code, but which allows setting of the output before a test runs and verification of input parameters after the test run.

## Acknowledgements

Autospy is heavily influenced by the excellent [mockall](https://docs.rs/mockall/latest/mockall/) crate, which through [automock](https://docs.rs/mockall/latest/mockall/attr.automock.html) provides much the same features. Autospy aims to offer these same features through a macro generated spy object, rather than a mock object. The use of either is largely personal preference; however, there are some benefits to using a spy object. Notably, a mock object will panic if its expectations fail which can cause less legible error messages.