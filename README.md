# zlisp

"zlisp" is the name of a Lisp-like data representation. There are two data formats for zlisp data, a binary and a text format. The zlisp project consists of multiple crates for serializing and deserializing data from the data formats. The aim is to be compatible with the data representation and formats used by some game engines written by the company Zipper Interactive.

This project is not endorsed by or associated with Zipper Interactive.

## Status

The maintenance status is "as-is", and no support is provided.

## Changelog

### [0.1.0] - unreleased

* Initial version

## License

Licensed under the European Union Public Licence (EUPL) 1.2 ([LICENSE](LICENSE) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).

## Data representation

There are four data types: integers/ints, floats, strings, and lists.

In this documentation, a value refers to any data type. A scalar refers to an int, float, or string, but never a list.

Ints are always 32-bit signed integers.

Floats are single-precision IEEE 754 floating-point numbers (32 bits).

Strings are sequences of bytes (8 bits), where each byte must also be a valid ASCII character (in the range 1-127). Null characters and quote characters are not permitted. Null characters are too much trouble for C code, and quote characters are used for string quoting, but cannot be escaped. In principle, the maximum length for strings is 2147483647/0x7FFFFFFF bytes. However, since at least one data format does not allow strings of more than 255 bytes, for compatibility this is chosen as the maximum length in all formats.

Lists are sequences of zero or more values. This means the sequence is heterogeneous. The maximum length for sequences is 2147483647 - 1/0x7FFFFFFF - 1 values; for compatibility in all formats.

## Binary data format

For the binary data format, the input and output are byte sequences.

In the binary data format, each value is tagged with a 32-bit signed integer for the data type:

* Int: 1
* Float: 2
* String: 3
* List: 4

Any other tag is invalid.

Ints are encoded as little-endian 32-bit signed integers.

Floats are encoded as little-endian 32-bit/single-precision IEEE 754 floating-point numbers.

Strings are encoded with the string length as an int, followed by the characters encoded into an ASCII byte sequence with no embedded null or quote characters (`\0` or `"`, respectively). The byte sequence is also not terminated by a null character (not zero terminated). As mentioned above, because the maximum encoded string length is 255 in the text data format, this is also used as the limit in the binary data format to retain compatibility.

For a list, the encoded list length is the length + 1. This is encoded as an int, followed by each value. This means the encoded length of an empty list is 1, and an encoded length of 0 or less is invalid. We haven't tested what the actual maximum length of a list is, so the maximum length is simply a restriction based on the encoding of the length.

An additional restriction on the binary data format is that the outermost value must be a list with the length of 1. The binary data format read methods strip this outer list, and the binary data format write methods add it. There are no examples of the value this outer list contains being anything other than a list, but this isn't validated.

## Text data format

For the text data format, the input and output are ASCII byte sequence with no embedded null characters.

An unsupported feature in the text format is comments. They are supported in some engines. Comments must start with `;`, and any characters until the next line feed (`\n`) are then ignored. We have tested that a Windows newline (`\r\n`) is not required. Again, this is not implemented.

Valid whitespace characters are `"`/space, `\t`/tab, `\r`/carriage return, and `\n`/line feed. Whitespace is used as a delimiter for values, or ignored - except for quoted strings.

Lists are started/opened with `(`, and ended/closed with `)`. Valid delimiter characters for values in lists are whitespace characters, `(`, or `)`.

In this documentation, a token refers to the ASCII representation of any scalar, which is a sequence of ASCII bytes.

### Tokens/strings

Tokens may be unquoted, or quoted. In any case, tokens may not be longer than 255 bytes.

#### Unquoted

For unquoted tokens, the token is read until a valid delimiter character is found, or until the end of the data. An unquoted token may be an int, a float, or a string.

As an implementation detail, when deserializing text data, all scalars are read as tokens first. The parsing of the scalar into either int, float, or strings is left as late as possible. This is helpful if the data is being deserialized into a data structure, where the desired data type is known. Otherwise, token are attempted to be parsed into an int first, then a float, and finally, if the previous two steps failed, left as a string.

To prevent this conversion, see quoting.

#### Quoted

Quoting tokens is complex. First, when reading, a quoted token is always interpreted as a string. This means that values that would otherwise be interpreted as an int or a float may be quoted to avoid this.  Conversely, when serializing text data, it is necessary to quote strings that could be interpreted as an int or a float. However, doing this automatically would make serialization very expensive. There is also no manual quoting support yet, so this is a problem for now. Still, it is very uncommon.

Second, when a quote character (`"`) is found, further characters are read until another quote is found. This means quotes may contain any otherwise valid delimiter character. There is no provision for escape characters, so a quoted string still cannot contain a quote character (`"`) itself. Also note the quote characters do not count towards the token length.

A quote may appear at any point in the value. It also seems like multiple quotes are supported. Therefore, the following tokens are equivalent:

* `KEYS`
* `"KEYS"`
* `"KE"YS`
* `KE"YS"`
* `"KE""YS"`
* `"K"EYS`
* etc

Because text deserialization may need to handle quoted values, borrowed deserialization is not supported. In effect, `&str` will not work, but `String` will.

### Int representation

We don't yet quite know how ints are parsed in engines. We have examples of ints being represented in decimal, and in hexadecimal. But in general, the hexadecimal representation does not seem to work.

In the text format, ints are represented in decimal, with an optional sign followed by one or more digits. This regular expression approximates a valid decimal representation: `[-+]?[0-9]+`. (The parsing does not use a regular expression.)

For specific values, ints may be represented in hexadecimal. From testing, it seems a sign is not supported. So the hexadecimal text representation has the prefix `0x`, followed by one or more hex digits of any case. This regular expression approximates a valid hexadecimal representation: `0x[0-9a-fA-F]+`. (The parsing does not use a regular expression.) Since it seems to be contextual, a newtype helper is provided for deserializing and serializing hexadecimal ints. This also ensures these values round-trip properly. The underlying data type is still a 32-bit integer.

### Float representation

Floats are represented starting with an optional sign, followed by zero or more digits, followed by a decimal point (`.`), followed by zero or more digits. This regular expression approximates a valid decimal representation: `[-+]?[0-9]*\.[0-9]*`, however, there must be at least one digit before or after the decimal point (so `-`, `+`, `.`, `-.`, `+.` are not valid). (The parsing does not use a regular expression.)

## Lisp data structure to Rust/serde data structure mapping

In principle, the Lisp data structures are simple. However, it's somewhat possible to represent maps/dictionaries, e.g. using `(K1 V1 K2 V2 ...)`.

Rust data structures are much richer, and [serde](https://serde.rs/) supports many of them. Please [refer to the Serde data model](https://serde.rs/data-model.html) for reference.

The following types from Serde's data model are not supported:

* Primitive types: bool, i8, i16, i64, i128, u8, u16, u32, u64, u128, f64, char
* Byte arrays

The following types and mappings are supported:

* Primitive types `i32` and `f32`: the value or it's representation
* `String`, and `&str` (for binary deserialization only): the value
* Options: either `()` for `None` or `(...)` for `Some(...)`
* Units: always `()`
* Sequences: for example, `(V1 V2...)`, or `()` for an empty sequence
* Tuples: for example, `(V1 V2...)`, or `()` for an empty tuple
* Maps: for example, `(K1 V1 K2 V2...)`, or `()` for an empty map. Note that if the key ordering in the underlying data structure is not deterministic (like `HashMap`), the serialization also won't be
* Structures: see maps
* Newtype structures: transparent
* Tuple structures: see tuples
* Enum unit variants: for example, `V` for the variant `E::V` in `enum E { V, ... }`
* Enum newtype variants: for example, `V(1)` for the variant `E::V(1)` in `enum E { V(i32), ... }`
* Enum tuple variants: for example, `V(1 2)` for the variant `E::V(1, 2)` in `enum E { V(i32, i32), ... }`
* Enum structure variants: for example, `V(a 1 b 2)` for the variant `E::V { a = 1, b = 2 }` in  `enum E { V { a: i32, b: i32 } }`
