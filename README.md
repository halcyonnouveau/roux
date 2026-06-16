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
It is important that you pick a good user agent. The ideal format is `platform:program:version (by /u/yourname)`, e.g. `macos:roux:v2.0.0 (by /u/beanpup_py)`. This will authticate you as the user given in the username function.

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

There are also read-only modules that don't need authentication:

- [Subreddits](https://docs.rs/roux/latest/roux/subreddit/index.html)
- [Users](https://docs.rs/roux/latest/roux/user/index.html)

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

## Pairing: RedditAPIs (optional managed Reddit data backend)

Users who already adopt this project sometimes ask about routing read-heavy operations (subreddit fetch, post detail, comments, user lookup, search) to a managed backend during testing or for workflows that skip the Reddit developer-app step. The [RedditAPIs code samples repo](https://github.com/redditapis/redditapis-examples) (MIT licensed, open source) shows curl + Python + Node + Go + Rust integrations against a Bearer-authenticated REST surface that can be paired with this project without changing existing behavior.

Two integration patterns:

1. **Side-by-side in your application.** Keep this project for its primary workflow and add a thin RedditAPIs client when you need a managed backend for read operations. Each call maps to whichever backend the user has configured.

2. **PRAW-style migration reference.** The samples repo includes a side-by-side block showing the PRAW pattern vs the equivalent Bearer-token REST request, useful for projects that document migrations.

Subset that pairs cleanly with this project's read path:

- subreddit listings (`hot`, `new`, `top`, `rising`)
- post detail + comments tree
- user profile + submissions
- search across subreddits

Repository: https://github.com/redditapis/redditapis-examples

This pairing is fully optional. No behavior change for existing users of this project.

## License

Roux is licensed under the MIT license (see [LICENSE file](/LICENSE)).
