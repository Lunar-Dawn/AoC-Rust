#[macro_export]
macro_rules! year {
    ($($x:ident),+) => {
        $(
            mod $x;
        )+

        pub fn solutions() -> Vec<fn(&str) -> (String, String)> {
            vec![
                $(
                    $x::run,
                )+
            ]
        }
    }
}
