# API concepts

## Key

A type alias for API key as `secrecy::SecretString`.

## Client

A Rust struct that contains the fields for data that is shared between API requests.

Requirements:

- Must have attributes:
  - `#[derive(Clone, Debug)]`
- Must have fields:
  - `pub inner: HttpClient` (`use reqwest::Client as HttpClient;`)
  - `pub base: Url`
  - `pub limits: RateLimits`
- Must have methods:
  - `pub fn new(key: impl Into<Key>) -> Result<Self, ClientNewError>`
    - Must call `Self::try_from`
  - `pub fn default_base_url() -> Url` (`use url-macro::url`)
- Must have impls:
  - `TryFrom<Key>`
    - Must call `Self::try_from((key, Self::default_base_url()))`
  - `TryFrom<(Key, Url)>`
    - Must construct `inner` client
      - If the API key is passed via headers:
        - Must set the API key header via `default_headers`
          - Must mark the header as sensitive
    - Must call `Self::from((inner, base))`
  - `From<(HttpClient, Url)>`
  - `From<(HttpClient, Url, RateLimits)>`

## RateLimits

A Rust struct that has one field per documented rate limit.

- Must have attributes:
  - `#[derive(Debug)]`
- Every field must be a `LazyLock<DefaultDirectRateLimiter>` (`use governor::DefaultDirectRateLimiter`)
- Must have an `impl Default`
  - Must construct rate limiters according to documentation
