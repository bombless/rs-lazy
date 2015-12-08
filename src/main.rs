
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

impl<F> Lazy<F> {
    fn by<T>(f: F) -> Self where F: Fn()->T {
        Lazy(Box::new(f))
    }
}

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

impl<T, V> ::std::ops::Not for Lazy<T>
where T: Fn()->V + 'static, V: ::std::ops::Not {
    type Output = Lazy<Fn() -> <V as ::std::ops::Not>::Output>;
    fn not(self) -> Self::Output {
        Lazy(Box::new(move || !self.0()))
    }
}

macro_rules! impl_bin_op {
    ($name:ident, $method:ident) => {
        impl<L, R> ::std::ops::$name<Lazy<Fn()->R>> for Lazy<Fn()->L>
        where L: Clone + ::std::ops::$name<R>, R: Clone {
            type Output = Lazy<Fn() -> <L as ::std::ops::$name<R>>::Output>;
            fn $method(self, rhs: Lazy<Fn()->R>) -> Self::Output {
                use ::std::ops::$name;
                Lazy(Box::new(move || $name::$method(self.0(), rhs.0())))
            }
        }
    }
}

impl_bin_op!{Add, add}
impl_bin_op!{BitAnd, bitand}
impl_bin_op!{BitOr, bitor}
impl_bin_op!{BitXor, bitxor}
impl_bin_op!{Mul, mul}
impl_bin_op!{Rem, rem}
impl_bin_op!{Shl, shl}
impl_bin_op!{Shr, shr}
impl_bin_op!{Sub, sub}
