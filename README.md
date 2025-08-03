# OCPP-rs
A Rust-based implementation the OCPP-2.1 specification

## Features
OCPP-rs was designed with a few core principles in mind.

### Correctness
OCPP-rs places maximal emphasis on correctness. It fully-implements typing and value validation for every single enum, struct, and message type in the OCPP-2.1 spec.

# Notes:

A large portion of this codebase was generated programmatically. Errors in comments, tests, or validation functions may 
exist. If you detect an error or inconsistency, please report it.

# Todos
A list of major todos.
- For each relevant ISO, ensure all related fields use appropriate helper libs for validation
- Replace hard-coded error-bubbling logic in each test with compact helper functions