[![Build Status](<https://travis-ci.org/MaaniBeigy/palantir.svg?branch=master>)](<https://travis-ci.org/MaaniBeigy/palantir>)
[![AppVeyor build status](https://ci.appveyor.com/api/projects/status/a5acm52l6ycfra2k?svg=true)](<https://ci.appveyor.com/project/MaaniBeigy/palantir>)
[![License: MIT/Apache-2.0](<https://img.shields.io/badge/license-MIT%2FApache--2.0-brightgreen.svg>)](#license)
![GitHub last commit](<https://img.shields.io/github/last-commit/MaaniBeigy/palantir.svg>)
![GitHub code size in bytes](<https://img.shields.io/github/languages/code-size/MaaniBeigy/palantir.svg?color=brightgreen>)
[![](<https://img.shields.io/badge/devel%20version-0.1.0-yellow.svg>)](<https://github.com/MaaniBeigy/palantir>)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![contributions welcome](<https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat>)](<https://github.com/AASAAM/palantir/issues>)

<img src="./palantir.svg" align="right" width="192" />

# palantir

**palantir is a HTTP REST API reverse proxy**. It performs load balance, caching, and health check, and also prevents DDOS and reports metrics concerning health status of backend servers.

**Important: palantir is still under development and is not ready**.

# Getting started

If you are using Linux or macOS, you need to install **Rust** using [rustup](<https://rustup.rs/>):

```shell
curl https://sh.rustup.rs -sSf | sh
```

For installation on Windows, read the instruction in [rust-lang book](<https://doc.rust-lang.org/book/ch01-01-installation.html#installing-rustup-on-windows>).

### License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

palantir is inspired by [actix-reverse-proxy](<https://github.com/felipenoris/actix-reverse-proxy>), [bloom](<https://github.com/valeriansaliou/bloom>), [rustnish](<https://github.com/klausi/rustnish>), and [weldr](<https://github.com/hjr3/weldr>).


#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Name Origin

**"But alone it could do nothing but see small images of things far off and days remote." <br/>*the Lord of the Rings*, *The Two Towers* by John R. R. Tolkien**

The proxy's name *palantír* is derived from *the Lord of the Rings*, which is an artefact "used for both communication and as a means of seeing events in other parts of the world or in the distant past    or in the future" [Palantír](<https://en.wikipedia.org/wiki/Palant%C3%ADr>).

This name has been chosen because:

1. Reverse proxies are communication tools similar to seeing-stones. They could do nothing alone, but can be used to converse.
  
1. They may show something from the past (*i.e.,* cached data).

1. They where designed to guard and unite humans' world, by obtaining information. This reverse proxy tries also to collect metrics and prevent DDOS in collaboration with other microservices.

1. *Palantíri* (plural of *palantír*), may mislead you since the health status of the message is not guaranteed *per se*. Much work is required for revealing the real health status of the upstream servers, which is going to be developed in **health module**.
