use libhandheld::HandHeld;
use std::path::Path;
use std::time::Instant;

fn part_1() {
    let mut path = Path::new("inputs/input");
    let input = std::fs::read_to_string(path).unwrap();

    let input = input.split("\n").collect::<Vec<_>>();
    let mut handheld = HandHeld::new(true);
    handheld.run_code(&input);
    handheld.dump_registers();

    let trace = handheld.serialize_trace();
    std::fs::write("trace.trace", trace).expect("unable to write trace");
}

fn part_2() {
    let mut path = Path::new("inputs/input");
    let input = std::fs::read_to_string(path).unwrap();

    let mut input = input.split("\n").collect::<Vec<_>>();
    input.retain(|s| *s != "");

    let mut pc_ptr = 0;

    let mut handheld = HandHeld::new(false);
    let mut mutations = 0;
    let mut times = Vec::with_capacity(132);
    loop {
        let mut input_copy = input.clone();

        let old_inst_split = input[pc_ptr].split(" ").collect::<Vec<_>>();
        if old_inst_split[0] == "acc" {
            pc_ptr += 1;
            continue;
        }
        //println!("{:?}", old_inst_split);
        let new_inst = match old_inst_split[0] {
            "jmp" => format!("{} {}", "nop", old_inst_split[1]).to_owned(),
            "nop" => format!("{} {}", "jmp", old_inst_split[1]).to_owned(),
            _ => {
                panic!("invalid inst")
            }
        };

        input_copy[pc_ptr] = &new_inst;
        mutations += 1;

        pc_ptr += 1;

        let start = Instant::now();
        let ret = handheld.run_code(&input_copy);
        let elapsed = start.elapsed().as_nanos();
        times.push(elapsed);

        match ret {
            Ok(()) => break,
            Err(fault) => {}
        };

        handheld.reset();
    }

    let mean: u128 = (times.iter().sum::<u128>())/times.len() as u128;
    println!("needed {} mutations mean runtime {}", mutations, mean);
    handheld.dump_registers();
}

fn main() {
    part_1();
}
