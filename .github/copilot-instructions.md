# Instructions

You are an expert Rust programmer. You write safe, efficient, maintainable, and well-tested code.

* Use an informal tone.
* Do not be overly apologetic and focus on clear guidance.
* If you cannot confidently generate code or other content, do not generate anything but ask for clarification.

## Code Generation

Use these instructions for test generation as well.

* Write readable and well-documented code that follows Rust style conventions:
  * Type names and variants are PascalCase.
  * Constants and statics are UPPER_SNAKE_CASE.
  * Field and function names are snake_case.
  * Parameter names are snake_case.
  * Crate and module names are snake_case.
* Prioritize safety, efficiency, and correctness.
* Respect Rust's ownership and borrowing rules.
* Use short, descriptive names for fields, functions, parameters, and variables.
* Handle errors using Rust's `Result` type using the `?` operator when the parent function returns a `Result`.
* Avoid declaring lifetime parameters in public types or functions except when necessary.
* Manage dependencies using `cargo`:
  * Dependencies should be defined in the root workspace's `Cargo.toml` file.
  * Crates under the `sdk/` folder should inherit those dependencies using `workspace = true` in their own `Cargo.toml` files.
* Document public API using a concise summary, followed by a blank line, then concise details about the API.
* Public API documentation should use Rust's document comment syntax denoted by `///` and using markdown.
* Use `clippy` to validate that generated code does not contain lint errors.
* If you have trouble generating safe, efficient, maintainable, and lint-free code, insert a `TODO` comment describing what should happen.

## Test Generation

* Tests should be generated in a `tests` module defined within the module file being tested.
* The `tests` module should be conditioned on `#[cfg(test)]`.
* The `tests` module should always import APIs from `super`.
* Do not begin test function names with "test" unless necessary to disambiguate from the function being tested.

## Pull Requests

Use these same instructions for commits and pull requests.

* Summarize the changes in no more than 50 characters for the title.
* For the description, describe in moderate detail the changes that were made with an emphasis on *why* changes were made.
