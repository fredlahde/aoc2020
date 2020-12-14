#![feature(map_into_keys_values)]
use std::collections::BTreeMap;
mod custom_integer;

use custom_integer::CustomInteger;
use regex::Regex;

struct Mem {
    mem: BTreeMap<u128,u128>,
}

impl Mem {
    fn new() -> Self {
        Self {
            mem: BTreeMap::new(),
        }
    }

    fn write(&mut self, addr: u128, value: u128) {
        self.mem.insert(addr, value);
    }

    fn value(self) -> u128 {
        let values = self.mem.into_values().collect::<Vec<_>>();
        values.iter().sum()
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
        if groups.len() > 0 {
            let groups = &groups[0];
            let addr = groups[1].to_owned();
            let addr: u128 = addr.parse().unwrap();
            let val: u128 = groups[2].parse().unwrap();

            let addrs = generate_all_addrs(addr, current_mask);

            for addr in addrs {
                let addr = u128::from_str_radix(&addr, 2).unwrap();
                mem.write(addr, val);
            }
        }
    }
    println!("{}", mem.value());
}

fn generate_all_addrs(addr: u128, mask_in: &str) -> Vec<String> {
    let mut custom: CustomInteger = addr.into();
    custom.apply_mask(mask_in);

    let addr_string = custom.get_value_string();
    let x_count = mask_in.chars().filter(|c| *c == 'X').count();
    let permutations = generate_bit_permutations(x_count);

    let mut ret = Vec::new();

    for pp in permutations {
        let mut permutation_chars = pp.chars();
        let mut addr_chars = addr_string.clone().chars().collect::<Vec<_>>();
        for (i, cc) in mask_in.chars().enumerate() {
            if cc == 'X' {
                let p = permutation_chars.next().unwrap();
                addr_chars[i] = p;
            }
        }
        ret.push(addr_chars.iter().collect::<String>());
    }

    ret
}

fn generate_bit_permutations(n: usize) -> Vec<String> {
    let mut ret = Vec::new();
    let base: usize = 2;
    let p = base.pow(n as u32);
    for i in 0..p {
        ret.push(format!("{number:>0width$b}", number = i, width = n));
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bit_permutations() {
        let width = 3;
        let permutations = generate_bit_permutations(width);
        assert_eq!(8, permutations.len());
        assert_eq!(
            ["000", "001", "010", "011", "100", "101", "110", "111"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            permutations
        );

        let width = 2;
        let permutations = generate_bit_permutations(width);
        assert_eq!(4, permutations.len());
        assert_eq!(
            ["00", "01", "10", "11"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            permutations
        );
    }

    #[test]
    fn test_generate_all_addrs() {
        let addr = 42;
        let mask = "000000000000000000000000000000X1001X";
        let result = generate_all_addrs(addr, mask);
        println!("{:?}", result);
        assert_eq!(true, result.len() % 2 == 0);
        assert_eq!(4, result.len());
    }
}
