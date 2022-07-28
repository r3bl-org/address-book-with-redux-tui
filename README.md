# address-book-with-redux-tui

This is a CLI app that implements an immutable data structure that represents an address book. It
also has a rich text user interface. Mutations to this address book must happen via a Redux store,
and actions that are dispatched to it. Along w/ a reducer function.

This app is meant to be a pedagogical example of how to use the `r3bl_rs_utils` crate quite
comprehensively.

## TODO

- Change the simple terminal UI to use r3bl_rs_utils `tui` module

## Issues w/ loading shared libraries `libssl.so.1.1` when running `cargo run` on Ubuntu 22.04

With Ubuntu / PopOS! 22.04 libssl v3 is installed by default. And the APT sources list for v1.1 is
removed. There are lots of libraries written in Rust that have dependencies on locally installed
`libssl.so.1.1` file.

> You search for this file using this command:
>
> ```bash
> fd -t f libssl.so.1.1 /usr/
> ```

> There is a case when installing `cargo outdated` where you can force it to build openssl 1.1 from
> source rather than using the system installed version. Here's the command to do this
> (`--features vendored-openssl`):
>
> ```bash
> cargo install --locked cargo-outdated --force --features vendored-openssl
> ```

Here is a list of errors that we get:

1. In our case, running `cargo run` results in an error message like:
   `error while loading shared libraries libssl.so.1.1`.
2. Passing `--features vendored-openssl` to `cargo build` doesn't do anything (the `libssl.so.1.1`
   requirement must be in one of the underlying dependencies).

Here's how to solve the issue.

Download old versions of both libssl and libssl-dev from archive repository. Here's a
[so thread](https://unix.stackexchange.com/a/688289/302646) which points to the
[archive repository](https://snapshot.debian.org/package/openssl/1.1.1n-0%2Bdeb11u3/#libssl1.1_1.1.1n-0:2b:deb11u3).

1. Download & install the `libssl???.deb` file first:

   ```bash
   cd ~/Downloads/
   wget https://snapshot.debian.org/archive/debian-security/20220626T182543Z/pool/updates/main/o/openssl/libssl1.1_1.1.1n-0%2Bdeb11u3_amd64.deb
   sudo dpkg -i libssl1.1_1.1.1n-0+deb11u3_amd64.deb
   ```

2. Download & install the `libssl-dev???.deb` file last:

   ```bash
   cd ~/Downloads/
   wget https://snapshot.debian.org/archive/debian-security/20220626T182543Z/pool/updates/main/o/openssl/libssl-dev_1.1.1n-0%2Bdeb11u3_amd64.deb
   sudo dpkg -i libssl-dev_1.1.1n-0+deb11u3_amd64.deb
   ```

That's it. If you run the following commands you will see that `libssl.so.1.1` has been installed to
`/usr/lib/???` & `cargo run` works.

```bash
fd -t f libssl.so.1.1 /usr/
/usr/lib/x86_64-linux-gnu/libssl.so.1.1
```
