# js-sandbox

[<img alt="crates.io" src="https://img.shields.io/crates/v/js-sandbox?logo=rust&color=A6854D" />](https://crates.io/crates/js-sandbox)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-js--sandbox-4D8AA6?&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" />](https://docs.rs/js-sandbox)

`js-sandbox` is a Rust library for executing JavaScript code from Rust in a secure sandbox. It is based on the [Deno] project and uses [serde_json]
for serialization.


This library's primary focus is **embedding JS as a scripting language into Rust**. It does not provide all possible integrations between the two
languages, and is not tailored to JS's biggest domain as a client/server side language of the web.

Instead, `js-sandbox` focuses on calling standalone JS code from Rust, and tries to remain as simple as possible in doing so.
The typical use case is a core Rust application that integrates with scripts from external users, for example a plugin system or a game that runs
external mods.

This library is in early development, with a basic but powerful API. The API may still evolve considerably.

## Examples

### Print from JavaScript

The _Hello World_ example -- print something using JavaScript -- is one line, as it should be:
```rust
fn main() {
	js_sandbox::eval_json("console.log('Hello Rust from JS')").expect("JS runs");
}
```

### Call a JS function

A very basic application calls a JavaScript function `triple()` from Rust. It passes an argument and accepts a return value, both serialized via JSON:

```rust
use js_sandbox::{Script, AnyError};

fn main() -> Result<(), AnyError> {
	let js_code = "function triple(a) { return 3 * a; }";
	let mut script = Script::from_string(js_code)?;

	let arg = 7;
	let result: i32 = script.call("triple", &arg, None)?;

	assert_eq!(result, 21);
	Ok(())
}
```

An example that serializes a JSON object (Rust -> JS) and formats a string (JS -> Rust):

```rust
use js_sandbox::{Script, AnyError};
use serde::Serialize;

#[derive(Serialize, PartialEq)]
struct Person {
	name: String,
	age: u8,
}

fn main() -> Result<(), AnyError> {
	let src = r#"
		function toString(person) {
			return "A person named " + person.name + " of age " + person.age;
		}"#;

	let mut script = Script::from_string(src)
		.expect("Initialization succeeds");

	let person = Person { name: "Roger".to_string(), age: 42 };
	let result: String = script.call("toString", &person, None).unwrap();

	assert_eq!(result, "A person named Roger of age 42");
	Ok(())
}
```

### Maintain state in JavaScript

It is possible to initialize a stateful JS script, and then use functions to modify that state over time.
This example appends a string in two calls, and then gets the result in a third call:

```rust
use js_sandbox::{Script, AnyError};

fn main() -> Result<(), AnyError> {
	let src = r#"
		var total = '';
		function append(str) { total += str; }
		function get()       { return total; }"#;

	let mut script = Script::from_string(src)
		.expect("Initialization succeeds");

	let _: () = script.call("append", &"hello", None).unwrap();
	let _: () = script.call("append", &" world", None).unwrap();
	let result: String = script.call("get", &(), None).unwrap();

	assert_eq!(result, "hello world");
	Ok(())
}
```

#### Call a script with timeout

The JS code may contain long or forever running loops, that block Rust code. It is possible to set
a timeout after which JS script execution is aborted.

```rust
use js_sandbox::{Script, AnyError};

fn main() -> Result<(), AnyError> {
	let js_code = "function run_forever() { for(;;){} }";
	let mut script = Script::from_string(js_code)?;

	let result: Result<String, AnyError> = script.call("run_forever", &(), Some(1000));

	debug_assert_eq!(result.unwrap_err().to_string(), "Uncaught Error: execution terminated".to_string());

	Ok(())
}
```

[Deno]: https://deno.land/
[serde_json]: https://docs.serde.rs/serde_json
