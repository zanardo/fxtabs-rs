# fxtabs

Rust crate to collect open tabs on all Mozilla Firefox windows (from the same
_profile_).

Tabs are collected from `recovery.jsonlz4` file, where Firefox uses as a
persistent backup of open tabs, back and forward button pages, cookies, forms,
and other session data.

This file is written almost in real time (there will be only some seconds delay)
whenever there is a browsing/tabs action.

## Usage

Add the dependency on `Cargo.toml`:

```toml
fxtabs = "<version>"
```

Collect open tabs:

```rust
use fxtabs::open_tabs;

const FILE: &str = "/path/to/sessionstore-backups/recovery.jsonlz4";

fn main() {
    let tabs = open_tabs(FILE).unwrap();
    for t in tabs {
        println!("title: {}\nurl: {}\n", t.title, t.url);
    }
}

```
