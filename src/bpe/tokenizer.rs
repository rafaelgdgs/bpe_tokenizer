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

    pub fn set_max_tokens(&mut self, num: u32) {
        self.max_tokens = num;
    }

    pub fn set_parallel(&mut self, paral: bool) {
        self.parallel = paral;
    }

    pub fn read_string(&mut self, content: String) {
        trace!("Inside read_string.");
        self.initial_string = content;
    }

    pub fn show_original_string(&self) {
        println!("{}", self.initial_string);
    }

    pub fn show_difference(&self) {
        println!("Original: {:?}", self.initial_string.clone().into_bytes());
        println!("Modified: {:?}", self.final_string.clone());
    }

    pub fn tokenize(&mut self) {
        let mut hm: HashMap<(u32, u32), u32> = HashMap::new();
        let mut current_string: Vec<u32> = self.create_current_string();
        let mut unused: u32 = 256;

        while ((unused - 255) <= self.max_tokens) {
            current_string
                .windows(2)
                .for_each(|w| *hm.entry((w[0], w[1])).or_default() += 1);

            let max = hm.iter().max_by_key(|e| e.1);

            let Some(maxx) = max else {
                warn!("Returning from tokenize earlier due to max not found.");
                return;
            };
            let max_token = *maxx.0;

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
}
