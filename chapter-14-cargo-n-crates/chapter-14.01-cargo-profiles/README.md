# Profiles in cargo

Profiles are compilation configurations defaults that you can change.

Example with optimization level:

File: Cargo.toml
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Here, no optimization is done for dev compilations, but, for release builds, the maximum
level of it is desireable.

