% id: 15
% title: Performance testing Serverless Rust

## The Rust programming language

Rust is the most loved programming language for four years in a row, according to the yearly survey of [Stack Overflow](https://survey.stackoverflow.co/2023/). But what is all the hype about? There are a couple of things that make the language ergonomic, performant and [safe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html). About two and a half years ago I started with Rust by reading the [Rust book](https://doc.rust-lang.org/book/) and doing some of the exercises and I can highly recommend it as a starting point. Coming from dynamic languages such as Python and Javascript, it was refreshing to statically type my code and to compile my code before running it. The Rust compiler, `rustc`, is often praised for the error messages it emits, which often contain an actual explanation and suggestion to fix an error. I noticed that the compiler is providing guard rails and prevented several ambiguous runtime errors.

<img src="../assets/images/rustacean-flat-happy.svg" alt="rust logo" width="150"/>

## Null

Tony Hoare introduced the `Null` reference in [ALGOL](https://en.wikipedia.org/wiki/ALGOL) because it seemed convenient, but later regretted this and called it the [billion dollar mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/?ref=hackernoon.com).

## AWS Lambda Rust Runtime
