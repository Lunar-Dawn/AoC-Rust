#[macro_export]
macro_rules! year {
    ($($x:ident),+) => {
        $(
            mod $x;
        )+

        use crate::util::Solution;
        use crate::util::YearSolutions;

        pub fn solutions() -> YearSolutions {
            let mut i = 0;
            let mut ret = YearSolutions::new();
            $(
                i += 1;
                ret.insert(i, $x::run as Solution);
            )+
            ret
        }
    }
}
