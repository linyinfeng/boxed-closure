/// A Closure whose function takes ownership of its environment
pub struct MoveClosure<Env, Args, Out> {
    env: Env,
    f: fn(Env, Args) -> Out,
}

impl<Env, Args, Out> MoveClosure<Env, Args, Out> {
    pub fn new(env: Env, f: fn(Env, Args) -> Out) -> Self {
        Self { env, f }
    }
}

impl<Env, Args, Out> FnOnce<Args> for MoveClosure<Env, Args, Out> {
    type Output = Out;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        (self.f)(self.env, args)
    }
}
