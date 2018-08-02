#![feature(fn_traits, unboxed_closures)]

//! Explain how closure works and provide a clear closure implementation with macro.
//!
//! # Example:
//! ```
//! # #[macro_use] extern crate closure;
//! let mut s = String::from("Hello");
//! {
//!     let mut append = closure! {
//!         ref mut [s: &mut String = &mut s,] (c: char,) -> usize {
//!             s.push(c);
//!             println!("{}", s);
//!             s.len()
//!         }
//!     };
//!     assert_eq!(append('!'), 6);
//!     assert_eq!(append('!'), 7);
//! }
//! assert_eq!(s, "Hello!!");
//! ```

mod move_closure;
mod ref_mut_closure;
mod ref_closure;

pub use move_closure::MoveClosure;
pub use ref_closure::RefClosure;
pub use ref_mut_closure::RefMutClosure;

/// A macro for writing closures easily
#[macro_export] macro_rules! closure {
    (move [$($env_name:ident: $env_type:ty = $env_exp:expr,)*]
        ($($arg_name:ident: $arg_type:ty,)*) -> $out:ty $body:block) => ({
        fn f(($($env_name,)*): ($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) -> $out $body
        $crate::MoveClosure::new(($($env_exp,)*), f)
    });
    (move [$($env_name:ident: $env_type:ty = $env_exp:expr,)*]
        ($($arg_name:ident: $arg_type:ty,)*) $body:block) => ({
        fn f(($($env_name,)*): ($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) $body
        $crate::MoveClosure::new(($($env_exp,)*), f)
    });
    (ref mut [$($env_name:ident: $env_type:ty = $env_exp:expr),*,]
        ($($arg_name:ident: $arg_type:ty),*,) -> $out:ty $body:block) => ({
        fn f(($($env_name,)*): &mut ($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) -> $out $body
        $crate::RefMutClosure::new(($($env_exp,)*), f)
    });
    (ref mut [$($env_name:ident: $env_type:ty = $env_exp:expr),*,]
        ($($arg_name:ident: $arg_type:ty),*,) $body:block) => ({
        fn f(($($env_name,)*): &mut ($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) $body
        $crate::RefMutClosure::new(($($env_exp,)*), f)
    });
    (ref [$($env_name:ident: $env_type:ty = $env_exp:expr),*,]
        ($($arg_name:ident: $arg_type:ty),*,) -> $out:ty $body:block) => ({
        fn f(($($env_name,)*): &($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) -> $out $body
        $crate::RefClosure::new(($($env_exp,)*), f)
    });
    (ref [$($env_name:ident: $env_type:ty = $env_exp:expr),*,]
        ($($arg_name:ident: $arg_type:ty),*,) $body:block) => ({
        fn f(($($env_name,)*): &($($env_type,)*), ($($arg_name,)*): ($($arg_type,)*)) $body
        $crate::RefClosure::new(($($env_exp,)*), f)
    });
}
