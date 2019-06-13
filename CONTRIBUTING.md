# Contributing to palantir development

The goal of this guide is to facilitate contributing to `palantir` as
quickly as possible. The guide is divided into two main pieces:

1. Filing a bug report or feature request in an issue.
1. Suggesting a change via a pull request.

Please note that `palantir` is released with a
[Contributor Code of Conduct](.github/CODE_OF_CONDUCT.md). By contributing to
this project, you agree to abide by its terms.

## Issues

When filing an issue, the most important thing is to include a **minimal
reproducible example** so that we can quickly verify the problem, and then figure
out how to fix it. There are three things you need to include to make your
example reproducible: **required crates**, **meta data**, and **code**.

1. **Crates** should be imorted at the top of the script, so it's easy to
    see which ones the example needs.
  
1. The easiest way to include **meta data** is to use `config files` to
    generate the Rust code to recreate it.
  
1. Spend a little bit of time ensuring that your **code** is easy for others to
    read:
  
    * make sure you've used spaces and your variable names are concise, but
      informative
  
    * use comments to indicate where your problem lies
  
    * do your best to remove everything that is not related to the problem.  
     The shorter your code is, the easier it is to understand.
  
    * do your best to follow [Rust Style Guide]
      <https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md>.
    * Use comments as much as needed to clarify the addition of a dependency,
      function, module, etc.
    * Divide the parts or blocks of code by topic lines like:
     `// -------------------  we declare variables here ----------------------`
    * To avoid spaghetti code make mod.rs in modules and submodules. This might
      be a brief representation of project's map:
      palantir
      └── main
          ├── mod proxy
          │   ├── serve
          │   ├── tunnel
          │   ├── headers
          │   └── defaults
          └── mod server
              ├── listen
              └── handle

## Pull requests

To contribute a change to `palantir`, you follow these steps:

1. Create a branch in git and make your changes.
1. Push branch to github and issue pull request (PR).
1. Discuss the pull request.
1. Iterate until either we accept the PR or decide that it's not
   a good fit for `palantir`.

Each of these steps are described in more detail below. This might feel
overwhelming the first time you get set up, but it gets easier with practice.
If you get stuck at any point, please reach out for help from package maintainer's email: [manibeygi@gmail.com)](manibeygi@gmail.com).

`palantir` is built using the latest version of `stable` Rust, using [the 2018 edition](https://doc.rust-lang.org/edition-guide/rust-2018/).

Using `rustup` you can get started this way:

```bash
rustup component add clippy
rustup component add rustfmt
rustup component add rustfmt-preview
```

In order to have your PR merged running the following must finish without error:

```bash
cargo test --all && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

Pull requests will be evaluated against a seven point checklist:

1. __Motivation__. Your pull request should clearly and concisely motivate the
    need for change.

1. __Only related changes__. Before you submit your pull request, please
    check to make sure that you haven't accidentally included any unrelated
    changes. These make it harder to see exactly what's changed, and to
    evaluate any unexpected side effects.

    Each PR corresponds to a git branch, so if you expect to submit
    multiple changes make sure to create multiple branches. If you have
    multiple changes that depend on each other, start with the first one
    and don't submit any others until the first one has been processed.

1. __Use of Rust's coding style guide__.
    <https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md>. Maintaining
    a consistent style across the whole code base makes it much easier to
    jump into the code. If you're modifying existing `palantir` code that
    doesn't follow the style guide, a separate pull request to fix the
    style would be greatly appreciated.

1. If you're adding new parameters or a new function, you'll also need
    to document them.

1. If fixing a bug or adding a new feature to a function,
    please add a unit test and benchmark.

This seems like a lot of work but don't worry if your pull request isn't perfect.
It's a learning process and members of the `palantir` team will be on hand to help you
out. A pull request ("PR") is a process, and unless you've submitted a few in the
past it's unlikely that your pull request will be accepted as is. All PRs require
review and approval from at least one member of the `palantir` development team
before merge.

Finally, remember that `palantir` is an in-development crate. We honorably welcome pull requests and contributions.
