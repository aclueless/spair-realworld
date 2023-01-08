// This sub-crate is extracted from https://github.com/jetli/rust-yew-realworld-example-app.
// Some modifications are made here and there.
// I will try to document all changes I made in this crate (compare to the original code). Both
// below in this file and at the precise place where I made the change. There are few exceptions,
// for example: changes from `String` to `&str` are not mentioned anywhere

// The exact commit that I copy from is https://github.com/jetli/rust-yew-realworld-example-app/commit/3a767a98600ee89e8b79fb23dcd9a7539ed59f36

pub mod error;
pub mod services;
pub mod types;
