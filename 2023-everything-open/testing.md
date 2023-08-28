---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

# **Advanced Testing in Rust**

- Tobin Harding (me@tobin.cc)
- Play along:

    github.com/tcharding/talks/tree/master/advanced-testing-in-rust

---

## Overview

- Unit tests, integration tests, directory structure.
- Rustdoc examples.
- Mutation testing with cargo-mutants.
- Mutation testing with Mutagen.
- Property-based testing, or why I haven't.
- Fuzzing with Hongfuzz.
- Code verification with Kani.

---

## Basic Stuff

- Unit tests
- Integration tests

You are faced with a module that you didn't write, it has some functionality that you more or less understand, and it has some tests.

**Question:** How do you convince yourself that it is good quality and not full of bugs?

---

## Test coverage

Good test coverage?

One metric is lines of code covered but just because you've executed a line of code with a test is that line tested?

Another interesting way to test code is to mutate the code and see if the tests break.

---

## cargo mutants

Basic mutation can be achieved with `cargo mutants`

- Replaces functions boby with `Default::default()`
- Requires no changes to the source tree

ref: https://mutants.rs/

How to catch all the mutations?

---

## Rustdoc

Add rustdoc examples, benefits:

- Improves documentation.
- Tests the public API.
- Demonstrates usage (incl. import paths).
- Highlights API breaking changes.

---

Now we have:

- Awesome docs
- Good test coverage of the public API

But have we tested the logic sufficiently?

---

## Mutation testing with `mutagen`

- Install with: `cargo +nightly install --git https://github.com/llogiq/mutagen`
- Run with: `RUSTFLAGS='--cfg=mutate' cargo +nightly mutagen`

ref: https://github.com/llogiq/mutagen

---

Now we have:

- Confidence that our unit test coverage is solid
- Confidence that we unit tested corner cases
- Confidence our API surface is covered by tests
- Confidence we (as the dev doing the testing) understand the module

WIN! But is our code _always_ correct?

---

## Hongfuzz

Throw _any_ data at a function.

---

## Code verification

There are at least two ways to gain confidence that our code behaves correctly with _all_ arguments:

- property based testing (prop testing)
- code verification

Prop testing is a kind of fuzzing that passes values into the function being tested, as such it adds confidence that the function is correct but it does not test _all_ input values (imagine a function taking three u64 args, to test all combinations would take ages).

---

## Enter kani

Mathematically prove that a function is correct for any input.

---

## Summing up

- Rustdoc examples.
- Mutation testing with cargo-mutants.
- Mutation testing with Mutagen.
- Fuzzing with Hongfuzz.
- Code verification with Kani.
