use std::collections::HashMap;
use std::rc::Rc;

use crate::runner;
use crate::util::DynResult;

runner!();

fn parse(input: &str) -> DynResult<HashMap<String, Rc<Server>>> {
    let lines = input.lines();

    let mut raw_servers: HashMap<_, _> = lines.map(|s| parse_line(s)).collect::<Result<_, _>>()?;

    let mut servers = HashMap::new();
    servers.insert(
        "out".to_string(),
        Rc::new(Server {
            name: "out".to_string(),
            outputs: None,
        }),
    );

    while !raw_servers.is_empty() {
        let Some((name, outputs)) = raw_servers
            .extract_if(|_, v| v.iter().all(|o| servers.contains_key(o)))
            .next()
        else {
            return Err("Could not connect servers to their destinations".into());
        };

        let server = Rc::new(Server {
            name: name.to_string(),
            outputs: Some(outputs.iter().map(|s| servers[s].clone()).collect()),
        });
        servers.insert(name.to_string(), server);
    }

    Ok(servers)
}
fn parse_line(line: &str) -> DynResult<(String, Vec<String>)> {
    let mut split = line.split_whitespace().map(|s| s.to_string());
    let Some(name) = split.next() else {
        return Err(format!("Line appears to be empty, failed to get name {line}").into());
    };
    let name = &name[0..name.len() - 1];

    Ok((name.to_string(), split.collect()))
}

struct Server {
    name: String,
    outputs: Option<Vec<Rc<Server>>>,
}
#[derive(Hash, PartialEq, Eq)]
struct CacheState<'a> {
    name: &'a str,
    visited_dac: bool,
    visited_fft: bool,
}
impl Server {
    fn calc_paths_to_out<'a>(
        &'a self,
        mut visited_dac: bool,
        mut visited_fft: bool,
        cache: &mut HashMap<CacheState<'a>, u64>,
    ) -> u64 {
        let state = CacheState {
            name: &self.name,
            visited_dac,
            visited_fft,
        };
        if let Some(&ret) = cache.get(&state) {
            return ret;
        }

        match self.name.as_str() {
            "dac" => visited_dac = true,
            "fft" => visited_fft = true,
            _ => (),
        }

        let Some(outputs) = &self.outputs else {
            if visited_dac && visited_fft {
                return 1;
            }
            return 0;
        };

        let mut ret = 0;
        for output in outputs {
            ret += output.calc_paths_to_out(visited_dac, visited_fft, cache);
        }

        cache.insert(state, ret);
        ret
    }
}

fn part1(servers: &HashMap<String, Rc<Server>>) -> u64 {
    servers["you"].calc_paths_to_out(true, true, &mut HashMap::new())
}

fn part2(servers: &HashMap<String, Rc<Server>>) -> u64 {
    servers["svr"].calc_paths_to_out(false, false, &mut HashMap::new())
}
