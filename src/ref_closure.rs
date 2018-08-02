/// A Closure whose function takes reference of its environment
pub struct RefClosure<Env, Args, Out> {
    env: Env,
    f: fn(&Env, Args) -> Out,
}

impl<Env, Args, Out> RefClosure<Env, Args, Out> {
    pub fn new(env: Env, f: fn(&Env, Args) -> Out) -> Self {
        Self { env, f }
    }
}

impl<Env, Args, Out> FnOnce<Args> for RefClosure<Env, Args, Out> {
    type Output = Out;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        (self.f)(&self.env, args)
    }
}

impl<Env, Args, Out> FnMut<Args> for RefClosure<Env, Args, Out> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output {
        (self.f)(&self.env, args)
    }
}

impl<Env, Args, Out> Fn<Args> for RefClosure<Env, Args, Out> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output {
        (self.f)(&self.env, args)
    }
}