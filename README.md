# Roux

[![Build](https://github.com/halcyonnouveau/roux/actions/workflows/rust.yml/badge.svg)](https://github.com/halcyonnouveau/roux/actions/workflows/rust.yml)
[![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/roux)
[![Crate](https://img.shields.io/crates/v/roux.svg)](https://crates.io/crates/roux)
![GitHub](https://img.shields.io/github/license/halcyonnouveau/roux.svg)

Roux is a simple, asynchronous Reddit API wrapper implemented in Rust.

## Usage

### Using OAuth

To create an OAuth client with the Reddit API, use the `Reddit` class.

```rust
use roux::Reddit;
let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .username("USERNAME")
    .password("PASSWORD")
    .login()
    .await;

let me = client.unwrap();
```

It is important that you pick a good user agent. The ideal format is
`platform:program:version (by /u/yourname)`, e.g. `macos:roux:v0.3.0 (by /u/beanpup_py)`.
This will authticate you as the user given in the username function.

### Usage

Using the OAuth client, you can:

#### Submit A Text Post

```rust
use roux::Reddit;
let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .username("USERNAME")
    .password("PASSWORD")
    .login()
    .await;

let me = client.unwrap();
me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT");
```

#### Submit A Link Post

```rust
use roux::Reddit;
let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .username("USERNAME")
    .password("PASSWORD")
    .login()
    .await;

let me = client.unwrap();
me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
```

### Read-Only Modules

There are also read-only modules that don't need authentication:

- [Subreddits](https://docs.rs/roux/latest/roux/subreddit/index.html)
- [Users](https://docs.rs/roux/latest/roux/user/index.html)

## Features

- (Default) `tokio_new` - use tokio 1.1.0 based reqwest.
- `tokio_back_compat` - use tokio 0.2 based reqwest.
- `blocking` - Use a blocking (sync) API instead of tokio. Note that `no-default-features` must be set along with `blocking`.

## 3rd-Party Libraries

- [`roux-stream`](https://github.com/torfsen/roux-stream) provides an API for continuously streaming new submissions and comments

## Contributing

Roux is not in active development but is still being maintained and currently covers the
most common and useful endpoints. If you see something missing or encounter a bug, feel
free to open an issue or create a pull request.

## License

Roux is licensed under the MIT license (see [LICENSE file](/LICENSE)).
