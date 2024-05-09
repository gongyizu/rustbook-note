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
- char: 4 bytes, it can represent a lot more than just ASCII

#### questions ####
- zero-width spaces: 零宽度字符，不可见、不可打印的字符
- char and character in Rust: [see more](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

### Compound Types ###
- tuple: fixed length, once declared, they cannot grow or shrink in size
    - unit: tuple without any values. The value and its corresponding type are both written _()_

- array
    - same type elements
    - fixed length
    - data allocated on the stack rather than the heap. [More about the stack and heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
    - [vector](https://doc.rust-lang.org/book/ch08-01-vectors.html)
    - if index is greater than or equal to the array length, Rust will panic 

### Functions ###
- Rust code uses snake case as the conventional style for function and variable names
- parameter: the variables in a function’s definition
- arguments: the concrete values passed in when you call a function

- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value.

### Functions ###
- Rust will not automatically try to convert non-Boolean types to a Boolean.
