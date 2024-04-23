## ch03 ##

### Constant ###

- You aren’t allowed to use mut with constants.
- declare constants using the _const_ keyword instead of the _let_ keyword
- the type of the value must be annotated
- Constants can be declared in any scope
- may be set only to a constant expression, not the result of a value that could only be computed at runtime

### Scalar Types ###

A scalar type represents a single value

- integers
- floating-point numbers
- Booleans
- characters


#### Integer Overflow ####
- debug mode: panic at runtime
- release mode: wrap around
- explicitly handling
 - _wrapping_*_
 - _checked_*_
 - _overflowing_*_
 - _saturating_*_


#### Numeric Operations ####
- Integer division truncates toward zero to the nearest integer.

#### type size ####
- bool: 1 byte
- char: 4 bytes

#### questions ####
- zero-width spaces
- char and character in rust

### Compound Types ###