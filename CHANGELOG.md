
## v0.6.2

Documentation

## v0.6.1 
all setters now accept the more generic type ```&[u8]```. 
 ```rust
let header = request.header();
header.add(&Bytes::from("X-Foo"), &Bytes::from("foo"));
header.add(b"X-Bar", b"bar"); // this is now possible
```        

## v0.6.0
### New
#### Header
- provide new method ```pub fn get(&self) -> HashMap<Bytes, Vec<Bytes>>``` which returns a map of header names together with the corresponding values

### API Change
#### Bytes
- as_str renamed to to_str to better adhere to rust naming standards
- to_str now returns a Result<&str, Utf8Error> to be able to react to non-utf8 input

## v0.5.1
### New
Ability to construct Bytes from &[u8]
```rust
let b = Bytes::from(b"test");
```
## v0.5.0
### New
Change own log macros to the standard log-crate