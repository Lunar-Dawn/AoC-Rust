pub trait ILog10<T> {
    fn ilog10(self) -> u32;
}
pub trait Pow<T> {
    fn pow(self, exp: u32) -> T;
}

macro_rules! impl_traits {
    ($ty:ty) => {
        impl ILog10<$ty> for $ty {
            fn ilog10(self) -> u32 {
                self.ilog10()
            }
        }

        impl Pow<$ty> for $ty {
            fn pow(self, exp: u32) -> $ty {
                self.pow(exp)
            }
        }
    };
}

impl_traits!(u8);
impl_traits!(u16);
impl_traits!(u32);
impl_traits!(u64);
impl_traits!(i8);
impl_traits!(i16);
impl_traits!(i32);
impl_traits!(i64);

pub fn num_digits<T>(x: T) -> u32
where
    T: ILog10<T>,
{
    x.ilog10() + 1
}
pub fn next_power_of_10<T>(x: T) -> T
where
    T: ILog10<T> + num_traits::PrimInt,
{
    T::from(10u32).unwrap().pow(num_digits(x))
}
pub fn prev_power_of_10<T>(x: T) -> T
where
    T: ILog10<T> + num_traits::PrimInt,
{
    T::from(10u32).unwrap().pow(num_digits(x) - 1)
}

pub fn split_digits<const N: usize, T>(mut x: T) -> [T; N]
where
    T: ILog10<T> + num_traits::PrimInt + num_traits::NumAssign,
{
    let num_digits = num_digits(x);
    let divisor = T::from(10u32).unwrap().pow(num_digits / N as u32);

    let mut result = [T::from(0u32).unwrap(); N];

    for i in 0..(N - 1) {
        result[N - i - 1] = x % divisor;
        x /= divisor;
    }
    result[0] = x;

    result
}
pub fn split_digits_n<T>(mut x: T, n: u32) -> Vec<T>
where
    T: ILog10<T> + num_traits::PrimInt + num_traits::NumAssign,
{
    let num_digits = num_digits(x);
    let divisor = T::from(10u32).unwrap().pow(num_digits / n);

    let mut ret = vec![T::from(0u32).unwrap(); n as usize];
    for i in (1..n).rev() {
        ret[i as usize] = x % divisor;
        x /= divisor;
    }
    ret[0] = x;
    ret
}
