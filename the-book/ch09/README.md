# Chapter 9 - Error Handling

- Unrecoverable errors (`panic!`); and,
- Recoverable errors (`Result<T, E>`).

Note, that exceptions don't exist in Rust - you won't find the try/except pattern, but will instead find functions that return `Result<T, E>` enums that achieve the same thing when handled using pattern matching.
