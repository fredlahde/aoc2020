use std::collections::HashMap;
mod custom_integer;

use custom_integer::CustomInteger;
use regex::Regex;

struct Mem {
    mem: [CustomInteger; 65396+1],
}

impl Mem {
    fn new() -> Self {
        Self {
            mem: [CustomInteger::default(); 65396+1],
        }
    }

    fn write(&mut self, addr: u64, value: &str, mask: &str) {
        let mut custom: CustomInteger = value.parse().unwrap();
        custom.apply_mask(mask);
        self.mem[addr as usize] = custom;
    }
    
    fn value(&self) -> u128 {
        let mut ret = 0u128;
        for m in &self.mem {
            ret += m.get_value() as u128;
        }
        ret
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/input").unwrap();
    let mut mem = Mem::new();
    let mut current_mask = "";
    let re = Regex::new(r"^mem\[(\d*)\]\s=\s(\d*)$").unwrap();
    for line in input.split("\n") {
        if line.starts_with("mask") {
            current_mask = line.split("mask = ").nth(1).unwrap();
            continue;
        }

        let groups = &re.captures_iter(&line).collect::<Vec<_>>();
        if groups.len()> 0 {
            let groups = &groups[0];
            let addr = groups[1].to_owned();
            let addr: u64 = addr.parse().unwrap();
            let val = groups[2].to_owned();
            
            mem.write(addr, &val, current_mask);
        }
    }
    println!("{}", mem.value());
}
