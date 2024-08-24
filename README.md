# Numlang
An esoteric programming language that uses hexadecimal.

## Usage

Install the compiler with ``cargo install --locked numlang``. Get help using ``numlang --help``.

## Syntax

### Basic Operations
1 -> Move pointer right by one<br>
2 -> Move pointer left by one<br>
3 -> Increment value by one<br>
4 -> Decrement value by one<br>

### File I/O
5 -> Read byte from stdin to pointer location<br>
6 -> Print current byte to stdout<br>
7 -> Print bytes in stack to stdout<br>

### Stack Manipulation
8 -> Add current byte to stack<br>
9 -> Remove newest item from stack<br>
a -> Clear stack<br>
b -> Write stack length to current byte<br>

### Looping
c -> Open loop<br>
d -> Close loop<br>
