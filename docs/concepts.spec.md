# API concepts spec

- Must not require `Client` or `RateLimits` to implement `Clone`
  - Reasons:
    - `governor::RateLimiter` doesn't implement `Clone`
