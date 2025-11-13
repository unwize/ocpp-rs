# OCPP-rs
A Rust-based implementation the OCPP-2.1 specification

## Features
OCPP-rs was designed with a few core principles in mind.

### Correctness
OCPP-rs places maximal emphasis on correctness. It fully-implements typing and value validation for every single enum, struct, and message type in the OCPP-2.1 spec (for all explicitly-defined conditions in _Messages, Datatypes & Enumerations_)

# Todos
A list of major todos.
- For each relevant ISO, ensure all related fields use appropriate helper libs for validation
- Replace hard-coded error-bubbling logic in each test with compact helper functions

# References
- OCPP 2.1 Spec

# Aknowlegements
- [Serde](https://serde.rs/): Message serialization/deserialization
- [Miette](https://docs.rs/miette/latest/miette/): Error handling, diagnostics
