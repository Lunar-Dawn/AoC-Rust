#[macro_export]
macro_rules! runner {
    () => {
        pub fn run(input: &str) -> (String, String) {
            let parsed = match parse(input) {
                Ok(parsed) => parsed,
                Err(error) => {
                    let ret = format!("Parsing Error: {error}");
                    return (ret, String::new());
                }
            };
            return (part1(&parsed).to_string(), part2(&parsed).to_string());
        }
    };
}
