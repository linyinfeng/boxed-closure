/// A Closure whose function takes mutable reference of its environment
pub struct RefMutClosure<Env, Args, Out> {
    env: Env,
    f: fn(&mut Env, Args) -> Out,
}

impl<Env, Args, Out> RefMutClosure<Env, Args, Out> {
    pub fn new(env: Env, f: fn(&mut Env, Args) -> Out) -> Self {
        Self { env, f }
    }
}

impl<Env, Args, Out> FnOnce<Args> for RefMutClosure<Env, Args, Out> {
    type Output = Out;
    extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output {
        (self.f)(&mut self.env, args)
    }
}

impl<Env, Args, Out> FnMut<Args> for RefMutClosure<Env, Args, Out> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        (self.f)(&mut self.env, args)
    }
}
