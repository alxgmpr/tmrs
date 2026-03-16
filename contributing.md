# Contribution Guidelines

Everything is welcome, just be professional. Use a good description if you're going to open a PR.

CI runs formatting, clippy, and tests on ubuntu/macos/windows — make sure these pass locally before pushing:

```shell
$ cargo fmt --all
$ cargo clippy --all-targets --all-features
$ cargo test
```
