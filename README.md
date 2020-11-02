# Example cargo-embed project using Workspaces

[cargo-embed](https://github.com/probe-rs/cargo-embed) doesn't seem to support workspaces.  This simple
workspace project is to help debug issue [#119](https://github.com/probe-rs/cargo-embed/issues/119).

This project can be built using `cargo build` from the root directory.  However, attempting to use
`cargo embed` fails with:

```shell
$ cargo embed
error: package collision in the lockfile: packages common v0.1.0 (C:\path_to_workspace\common) and common v0.1.0 (C:\path_to_workspace\firmware\../common) 
are different, but only one can be written to lockfile unambiguously
       Error Failed to run cargo build: exit code = Some(101)
```