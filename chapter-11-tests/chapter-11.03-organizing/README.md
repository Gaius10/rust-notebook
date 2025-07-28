# Conventions about tests in Rust

## Definition of Unit tests:

Tests private isolated functions.

## Definition of integration tests:

Tests public API, uses our app the same way as external programmers
do.

## Where to put tests:

### Unit

- Put them in the same file as the code to be tested;
- Create a module named `tests` in each file to contain the test
  functions;
- Annotate the module with `#[cfg(test)]`.
- Annotate each test function with #[test]

### Integration

- Put them in `tests/` directory;
- Annotate each test function with `#[test]`;
- Each file in `tests/` is a separate crate, so we need to include
  our modules in all of them;

