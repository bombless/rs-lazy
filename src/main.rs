use std::ops::*;
use std::fmt;

fn main() {
    println!("{}", 1.lazy() + 2.lazy());
}

fn force<T>(v: Lazy<Fn()->T>) -> T {
    v.0()
}

impl<T: fmt::Display> fmt::Display for Lazy<Fn()->T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0())
    }
}

struct Lazy<T: ?Sized>(Box<T>);

impl<T: Clone + 'static> Lazy<Fn()->T> {
    pub fn new(v: T) -> Self {
        let clone = move || v.clone();
        Lazy(Box::new(move || clone()))
    }
}

trait ToLazy {
    fn lazy(self) -> Lazy<Fn()->Self>;
}

impl<T: Clone + 'static> ToLazy for T {
    fn lazy(self) -> Lazy<Fn()->Self> {
        let clone = move || self.clone();
        Lazy(Box::new(move || clone()))
    }
}

impl<L, R> Add<Lazy<Fn()->R>> for Lazy<Fn()->L> where L: Clone + Add<R>, R: Clone {
    type Output = Lazy<Fn() -> <L as Add<R>>::Output>;
    fn add(self, rhs: Lazy<Fn()->R>) -> Self::Output {
        Lazy(Box::new(move || self.0() + rhs.0()))
    }
}
