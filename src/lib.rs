// Copyright (c) 2020-2021 Jan Haller. zlib/libpng license.

// Note: the crate documentation is copied to ReadMe.md using cargo-readme:  cargo readme --no-license > ReadMe.md
// Alternatives:
// * once stable: #![feature(external_doc)] #![doc(include = "../ReadMe.md")]
// * doc_comment crate + doctest!("../ReadMe.md");  -- works for running doc-tests, but not for doc on crate level

//! `js-sandbox` is a Rust library for executing JavaScript code from Rust in a secure sandbox. It is based on the [Deno] project and uses [serde_json]
//! for serialization.
//!
//!
//! This library's primary focus is **embedding JS as a scripting language into Rust**. It does not provide all possible integrations between the two
//! languages, and is not tailored to JS's biggest domain as a client/server side language of the web.
//!
//! Instead, `js-sandbox` focuses on calling standalone JS code from Rust, and tries to remain as simple as possible in doing so.
//! The typical use case is a core Rust application that integrates with scripts from external users, for example a plugin system or a game that runs
//! external mods.
//!
//! This library is in early development, with a basic but powerful API. The API may still evolve considerably.
//!
//! # Examples
//!
//! ## Print from JavaScript
//!
//! The _Hello World_ example -- print something using JavaScript -- is one line, as it should be:
//! ```rust
//! fn main() {
//! 	js_sandbox::eval_json("console.log('Hello Rust from JS')").expect("JS runs");
//! }
//! ```
//!
//! ## Call a JS function
//!
//! A very basic application calls a JavaScript function `triple()` from Rust. It passes an argument and accepts a return value, both serialized via JSON:
//!
//! ```rust
//! use js_sandbox::{Script, AnyError};
//!
//! fn main() -> Result<(), AnyError> {
//! 	let js_code = "function triple(a) { return 3 * a; }";
//! 	let mut script = Script::from_string(js_code)?;
//!
//! 	let arg = 7;
//! 	let result: i32 = script.call("triple", &arg, None)?;
//!
//! 	assert_eq!(result, 21);
//! 	Ok(())
//! }
//! ```
//!
//! An example that serializes a JSON object (Rust -> JS) and formats a string (JS -> Rust):
//!
//! ```rust
//! use js_sandbox::{Script, AnyError};
//! use serde::Serialize;
//!
//! #[derive(Serialize, PartialEq)]
//! struct Person {
//! 	name: String,
//! 	age: u8,
//! }
//!
//! fn main() -> Result<(), AnyError> {
//! 	let src = r#"
//! 		function toString(person) {
//! 			return "A person named " + person.name + " of age " + person.age;
//! 		}"#;
//!
//! 	let mut script = Script::from_string(src)
//! 		.expect("Initialization succeeds");
//!
//! 	let person = Person { name: "Roger".to_string(), age: 42 };
//! 	let result: String = script.call("toString", &person, None).unwrap();
//!
//! 	assert_eq!(result, "A person named Roger of age 42");
//! 	Ok(())
//! }
//! ```
//!
//! ## Maintain state in JavaScript
//!
//! It is possible to initialize a stateful JS script, and then use functions to modify that state over time.
//! This example appends a string in two calls, and then gets the result in a third call:
//!
//! ```rust
//! use js_sandbox::{Script, AnyError};
//!
//! fn main() -> Result<(), AnyError> {
//! 	let src = r#"
//! 		var total = '';
//! 		function append(str) { total += str; }
//! 		function get()       { return total; }"#;
//!
//! 	let mut script = Script::from_string(src)
//! 		.expect("Initialization succeeds");
//!
//! 	let _: () = script.call("append", &"hello", None).unwrap();
//! 	let _: () = script.call("append", &" world", None).unwrap();
//! 	let result: String = script.call("get", &(), None).unwrap();
//!
//! 	assert_eq!(result, "hello world");
//! 	Ok(())
//! }
//! ```
//!
//! ## Call a script with timeout
//!
//! The JS code may contain long or forever running loops, that block Rust code. It is possible to set
//! a timeout after which JS script execution is aborted.
//!
//! ```rust
//! use js_sandbox::{Script, AnyError};
//!
//! fn main() -> Result<(), AnyError> {
//! 	let js_code = "function run_forever() { for(;;){} }";
//! 	let mut script = Script::from_string(js_code)?;
//!
//! 	let result: Result<String, AnyError> = script.call("run_forever", &(), Some(1000));
//!
//! 	debug_assert_eq!(result.unwrap_err().to_string(), "Uncaught Error: execution terminated".to_string());
//!
//! 	Ok(())
//! }
//! ```
//!
//! [Deno]: https://deno.land/
//! [serde_json]: https://docs.serde.rs/serde_json


pub use script::Script;
pub use util::eval_json;

/// Represents a value passed to or from JavaScript.
///
/// Currently aliased as serde_json's Value type.
pub type JsValue = serde_json::Value;

/// Polymorphic error type able to represent different error domains.
///
/// Currently reusing [anyhow::Error](../anyhow/enum.Error.html), this type may change slightly in the future depending on js-sandbox's needs.
// use through deno_core, to make sure same version of anyhow crate is used
pub type AnyError = deno_core::error::AnyError;


mod script;
mod util;
