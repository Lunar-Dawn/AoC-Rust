use crate::util;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::rc::Rc;

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

fn read_server_line(line: &str) -> (String, Vec<String>) {
    let mut split = line.split_whitespace().map(|s| s.to_string());
    let name = split.next().unwrap();
    let name = &name[0..name.len() - 1];

    (name.to_string(), split.collect())
}
fn read_servers(path: &PathBuf) -> util::Result<HashMap<String, Rc<Server>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();

    let mut raw_servers: HashMap<_, _> =
        lines.iter().map(|s| read_server_line(s.as_str())).collect();

    let mut servers = HashMap::new();
    servers.insert(
        "out".to_string(),
        Rc::new(Server {
            name: "out".to_string(),
            outputs: None,
        }),
    );

    while !raw_servers.is_empty() {
        let (name, outputs) = raw_servers
            .extract_if(|_, v| v.iter().all(|o| servers.contains_key(o)))
            .next()
            .unwrap();

        let server = Rc::new(Server {
            name: name.to_string(),
            outputs: Some(outputs.iter().map(|s| servers[s].clone()).collect()),
        });
        servers.insert(name.to_string(), server);
    }

    Ok(servers)
}

pub fn part1(path: &PathBuf) -> util::Result<String> {
    let servers = read_servers(&path)?;

    let total = servers["you"].calc_paths_to_out(true, true, &mut HashMap::new());

    Ok(total.to_string())
}

pub fn part2(path: &PathBuf) -> util::Result<String> {
    let servers = read_servers(&path)?;

    let total = servers["svr"].calc_paths_to_out(false, false, &mut HashMap::new());

    Ok(total.to_string())
}
