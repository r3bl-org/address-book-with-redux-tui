# address-book-with-redux-tui
<a id="markdown-address-book-with-redux-tui" name="address-book-with-redux-tui"></a>


This is a CLI app that implements an immutable data structure that represents an address book. It
also has a rich text user interface. Mutations to this address book must happen via a Redux store,
and actions that are dispatched to it. Along w/ a reducer function.

This app is meant to be a pedagogical example of how to use the `r3bl_rs_utils` crate quite
comprehensively.

<hr/>

**Table of contents**

<!-- TOC -->

- [TODO](#todo)
- [Issues w/ loading shared libraries libssl.so.1.1 when running cargo run on Ubuntu 22.04](#issues-w-loading-shared-libraries-libsslso11-when-running-cargo-run-on-ubuntu-2204)
  - [Problem w/ OpenSSL v1.1](#problem-w-openssl-v11)
  - [Solution - install them manually](#solution---install-them-manually)

<!-- /TOC -->
<!-- /TOC -->

<hr/>

## TODO
<a id="markdown-todo" name="todo"></a>


- Change the simple terminal UI to use r3bl_rs_utils `tui` module.
- You can find all the TODOs [here](./TODO.todo), and work completed [here](./DONE.todo).

## Issues w/ loading shared libraries `libssl.so.1.1` when running `cargo run` on Ubuntu 22.04
<a id="markdown-issues-w%2F-loading-shared-libraries-libssl.so.1.1-when-running-cargo-run-on-ubuntu-22.04" name="issues-w%2F-loading-shared-libraries-libssl.so.1.1-when-running-cargo-run-on-ubuntu-22.04"></a>


With Ubuntu / PopOS! 22.04 libssl v3 is installed by default. And the APT sources list for v1.1 is
removed. There are lots of libraries written in Rust that have dependencies on locally installed
`libssl.so.1.1` file.

### Problem w/ OpenSSL v1.1
<a id="markdown-problem-w%2F-openssl-v1.1" name="problem-w%2F-openssl-v1.1"></a>


> Where is the `libssl.so.1.1` file? You can search for this file using this command (make sure you
> have `fd` installed via brew):
>
> ```bash
> fd -t f libssl.so.1.1 /usr/
> ```

Here is a list of errors that we get:

1. In our case, running `cargo run` results in an error message like:
   `error while loading shared libraries libssl.so.1.1`.
2. Passing `--features vendored-openssl` to `cargo build` doesn't do anything (the `libssl.so.1.1`
   requirement must be in one of the underlying dependencies).

> For some crates, eg: `cargo-outdated`, you can force them to build openssl 1.1 from source rather
> than using the system installed version. Here's the command to do this
> (`--features vendored-openssl`):
>
> ```bash
> cargo install --locked cargo-outdated --force --features vendored-openssl
> ```

### Solution - install them manually
<a id="markdown-solution---install-them-manually" name="solution---install-them-manually"></a>


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
