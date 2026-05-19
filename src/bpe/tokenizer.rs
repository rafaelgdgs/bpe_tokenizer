use std::collections::{HashMap, HashSet};

use log::{error, info, trace, warn};
use rayon::prelude::*;

#[derive(Default)]
enum State {
    #[default]
    New,
    ReadFile,
}

#[derive(Default)]
pub struct Bpe {
    state: State,
    max_tokens: u32,
    initial_string: String,
    final_string: Vec<u32>,
    tokens: HashMap<u32, (u32, u32)>,
    parallel: bool,
}

impl Bpe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_file(&mut self, filename: &str) {
        trace!("Inside read_file function!");
        use std::fs::read_to_string;
        let string = read_to_string(filename);
        match string {
            Ok(v) => {
                self.initial_string = v;
                self.state = State::ReadFile;
                trace!("Current state: ReadFile");
            }
            Err(v) => {
                println!("Problem openning the file!");
                error!("read_file error: {v}");
            }
        }
    }

    pub fn read_string(&mut self, content: String) {
        trace!("Inside read_string.");
        self.initial_string = content;
    }

    pub fn set_max_tokens(&mut self, num: u32) {
        self.max_tokens = num;
    }

    pub fn set_parallel(&mut self, paral: bool) {
        self.parallel = paral;
    }

    pub fn show_original_string(&self) {
        println!("{}", self.initial_string);
    }

    pub fn show_difference(&self) {
        println!("Original: {:?}", self.initial_string.as_bytes());
        println!("Modified: {:?}", self.final_string);
        println!("Tokens  : {:?}", self.tokens);
    }

    pub fn tokenize(&mut self) {
        let mut hm: HashMap<(u32, u32), u32> = HashMap::new();
        let mut current_string: Vec<u32> = self.create_current_string();
        let mut unused: u32 = 256;

        hm = self.find_token_frequency(&current_string);
        while ((unused - 255) <= self.max_tokens) {
            let max = self.get_max_key(&hm);

            let Some(max_token) = max else {
                warn!("Returning from tokenize earlier due to max not found.");
                return;
            };

            self.tokens.insert(unused, max_token);

            hm.remove(&max_token);

            let mut temp: Vec<u32> = Vec::with_capacity(current_string.len());
            let mut i = 0;

            while i < current_string.len() {
                if i + 1 < current_string.len()
                    && current_string[i] == max_token.0
                    && current_string[i + 1] == max_token.1
                {
                    temp.push(unused);
                    i += 2;
                } else {
                    temp.push(current_string[i]);
                    i += 1;
                }
            }
            current_string = temp;
            unused += 1;
        }
        self.final_string = current_string;
    }

    fn create_current_string(&self) -> Vec<u32> {
        if self.parallel {
            self.initial_string
                .as_bytes()
                .par_iter()
                .map(|&byte| u32::from(byte))
                .collect()
        } else {
            self.initial_string
                .as_bytes()
                .iter()
                .map(|&byte| u32::from(byte))
                .collect()
        }
    }

    fn find_token_frequency(&self, vector: &[u32]) -> HashMap<(u32, u32), u32> {
        if self.parallel {
            vector
                .par_windows(2)
                .map(|w| (w[0], w[1]))
                .fold(
                    || HashMap::<(u32, u32), u32>::new(),
                    |mut local_acc, pair| {
                        *local_acc.entry(pair).or_default() += 1;
                        local_acc
                    },
                )
                .reduce(
                    || HashMap::<(u32, u32), u32>::new(),
                    |mut map1, map2| {
                        for (pair, freq) in map2 {
                            *map1.entry(pair).or_default() += freq;
                        }
                        map1
                    },
                )
        } else {
            let mut hash: HashMap<(u32, u32), u32> = HashMap::new();
            vector
                .windows(2)
                .for_each(|w| *hash.entry((w[0], w[1])).or_default() += 1);
            hash
        }
    }

    fn get_max_key(&self, hm: &HashMap<(u32, u32), u32>) -> Option<(u32, u32)> {
        let return_value = match self.parallel {
            true => hm.par_iter().max_by_key(|e| e.1),
            false => hm.iter().max_by_key(|e| e.1),
        };
        return_value.map(|x| *x.0)
    }
}
