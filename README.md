# OCPP-rs
A Rust-based implementation the OCPP-2.1 specification

## Features
OCPP-rs was designed with a few core principles in mind.

### Correctness
OCPP-rs places maximal emphasis on correctness. It fully-implements typing and value validation for every single enum, struct, and message type in the OCPP-2.1 spec (for all explicitly-defined conditions in _Messages, Datatypes & Enumerations_)

### Performance
OCPP-rs provides methods for lazy deserialization of messages. Messages with nested structures can be converted into Rust objects on-the-fly to save on processing time. Nested structures are only deserialized as they are accessed. 

Users may also want to lazy-validate. Nested structures may be optionally be validated only as they are accessed. 

### Convenience

Major thought was placed into

# Notes:

A large portion of this codebase was generated programmatically. Errors in comments, tests, or validation functions may 
exist. If you detect an error or inconsistency, please report it.

# Todos
A list of major todos.
- For each relevant ISO, ensure all related fields use appropriate helper libs for validation
- Replace hard-coded error-bubbling logic in each test with compact helper functions