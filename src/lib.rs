#![feature(fn_traits, unboxed_closures)]

mod move_closure;
mod ref_mut_closure;
mod ref_closure;

pub use move_closure::MoveClosure;
pub use ref_closure::RefClosure;
pub use ref_mut_closure::RefMutClosure;

/// Easily crate a closure
/// # Example:
/// ```rust
/// let mut s = String::new("Hello")
/// let mut append = closure! {
///     ref mut [s: &mut String = &mut s] (c: char) -> () {
///         s.push(c);
///         println!("{}", s);
///     }
/// }
/// append();
/// append();
/// ```
#[macro_export] macro_rules! closure {
    (move [$($env_name:ident: $env_type:ty = $env_exp:expr,)*]
        ($($arg_name:ident: $arg_type:ty,)*) -> $out:ty $body:block) => {
        {
            let env = ($($env_exp,)*);
            fn f(env: ($($env_type,)*), args: ($($arg_type,)*)) -> $out {
                let ($($env_name,)*) = env;
                let ($($arg_name,)*) = args;
                $body
            }
            $crate::MoveClosure::new(env, f)
        }
    };
    (ref mut [$($env_name:ident: $env_type:ty = $env_exp:expr,)*]
        ($($arg_name:ident: $arg_type:ty,)*) -> $out:ty $body:block) => {
        {
            let env = ($($env_exp,)*);
            fn f(env: &mut ($($env_type,)*), args: ($($arg_type,)*)) -> $out {
                let ($($env_name,)*) = env;
                let ($($arg_name,)*) = args;
                $body
            }
            $crate::RefMutClosure::new(env, f)
        }
    };
    (ref [$($env_name:ident: $env_type:ty = $env_exp:expr,)*]
        ($($arg_name:ident: $arg_type:ty,)*) -> $out:ty $body:block) => {
        {
            let env = ($($env_exp,)*);
            fn f(env: &($($env_type,)*), args: ($($arg_type,)*)) -> $out {
                let ($($env_name,)*) = env;
                let ($($arg_name,)*) = args;
                $body
            }
            $crate::RefClosure::new(env, f)
        }
    };
}

#[cfg(test)] 
mod tests {
    use super::*;
    use std::any::TypeId;
    fn type_id_of<T: ?Sized + 'static>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    #[test]
    fn test_empty_move_closure() {
        let c = closure!(move [] () -> () {});
        assert_eq!(type_id_of(&c), TypeId::of::<MoveClosure<(), (), ()>>());
        assert_eq!(c(), ());
    }

    #[test]
    fn test_empty_ref_mut_closure() {
        let mut c = closure!(ref mut [] () -> () {});
        assert_eq!(type_id_of(&c), TypeId::of::<RefMutClosure<(), (), ()>>());
        assert_eq!(c(), ());
        assert_eq!(c(), ());
    }

    #[test]
    fn test_empty_ref_closure() {
        let c = closure!(ref [] () -> () {});
        assert_eq!(type_id_of(&c), TypeId::of::<RefClosure<(), (), ()>>());
        assert_eq!(c(), ());
        assert_eq!(c(), ());
    }
}
