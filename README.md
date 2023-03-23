# like-dotenv

[![CI Workflow](https://github.com/azujuuuuuun/like-dotenv/actions/workflows/ci.yml/badge.svg)](https://github.com/azujuuuuuun/like-dotenv/actions/workflows/ci.yml)

Something like dotenv.

## Usage

Please create a `.env` file like below.

```sh
# This is a comment.
SAMPLE_KEY1=SAMPLE_VALUE1
SAMPLE_KEY2="SAMPLE_VALUE2"
```

You can call `like_dotenv::config()` like below.

```rs
use std::env;

fn main() {
    like_dotenv::config().unwrap();

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
```

## References

- https://github.com/motdotla/dotenv
- https://github.com/dotenv-rs/dotenv
- https://github.com/allan2/dotenvy
- https://nextjs.org/docs/basic-features/environment-variables
- https://doc.rust-lang.org/reference/conditional-compilation.html
