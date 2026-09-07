<img src="https://raw.githubusercontent.com/halcyonnouveau/roux/refs/heads/main/assets/roux.png" alt="dutch oven" style="max-width: 100%;">

# Roux

[![Build](https://github.com/halcyonnouveau/roux/actions/workflows/main.yml/badge.svg)](https://github.com/halcyonnouveau/roux/actions/workflows/main.yml)
[![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/roux)
[![Crate](https://img.shields.io/crates/v/roux.svg)](https://crates.io/crates/roux)
![GitHub](https://img.shields.io/github/license/halcyonnouveau/roux.svg)

Roux is a simple, (a)synchronous Reddit API wrapper implemented in Rust.

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
It is important that you pick a good user agent. The ideal format is `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v2.0.0 (by /u/beanpup_py)`. This will authenticate you as the user given in the username function.

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
me.submit_text("TEXT_TITLE", "TEXT_BODY", "SUBREDDIT").await?;
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
me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT").await?;
```

### Read-Only Modules

Reddit no longer serves its JSON endpoints to unauthenticated clients, so the read-only [Subreddit](https://docs.rs/roux/latest/roux/subreddit/index.html) and [User](https://docs.rs/roux/latest/roux/user/index.html) modules need a token too. If you don't set a username and password, an application-only token is requested using just the client id and secret of your Reddit app. `Subreddit::new`, `User::new` and `Subreddits::search` are deprecated and will most likely return a 403 from Reddit.

#### Get Subreddit Posts

```rust
use roux::Reddit;
let subreddit = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .subreddit("rust")
    .await
    .unwrap();

let hot = subreddit.hot(25, None).await?;
```

#### Get User Overview

```rust
use roux::Reddit;
let user = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .user("spez")
    .await
    .unwrap();

let overview = user.overview(None).await?;
```

#### Search Subreddits

```rust
use roux::Reddit;
let subreddits = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .search_subreddits("rust", Some(10), None)
    .await?;
```

## Blocking Client

You can use a blocking (synchronous) API instead of tokio by enabling the `blocking` feature.

```toml
[dependencies]
roux = { version = "2", features = ["blocking"] }
```

```rust
use roux::Reddit;
let client = Reddit::new("USER_AGENT", "CLIENT_ID", "CLIENT_SECRET")
    .username("USERNAME")
    .password("PASSWORD")
    .login();

let me = client.unwrap();
me.submit_link("LINK_TITLE", "LINK", "SUBREDDIT");
```

## 3rd-Party Libraries

- [`roux-stream`](https://github.com/torfsen/roux-stream) provides an API for continuously streaming new submissions and comments

## Contributing

Roux is not in active development but is still being maintained and currently covers the most common and useful endpoints. If you see something missing or encounter a bug, feel free to open an issue or create a pull request.

## License

Roux is licensed under the MIT license (see [LICENSE file](/LICENSE)).
