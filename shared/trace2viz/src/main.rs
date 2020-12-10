use std::collections::BTreeMap;
use std::env;
use std::fs;

use libhandheld::trace::TraceEntry;

struct Node {
    instruction: String,
    pc: usize,
    r0: i64
}

struct Relation {
    from: usize,
    to: usize,
}

#[derive(Default)]
struct Graph {
    nodes: BTreeMap<usize, Node>,
    relations: Vec<Relation>,
}

impl Graph {
    fn add_trace_entry(&mut self, trace: TraceEntry, last_pc: Option<usize>) {
        self.nodes.insert(
            trace.pc,
            Node {
                instruction: trace.instruction.encode(),
                pc: trace.pc,
                r0: trace.r0
            },
        );

        if let Some(last_pc) = last_pc {
            self.relations.push(Relation {
                from: last_pc,
                to: trace.pc,
            });
        }
    }

    fn render(mut self) -> String {
        self.relations.push(Relation { from: 5, to: 15 });
        let mut ret = String::new();
        ret.push_str("digraph trace {\n");

        for (clock, node) in self.nodes {
            ret.push_str(&format!(
                "c{}[label=\"pc={} r0={} last_inst={}\"];\n",
                clock, node.pc,node.r0, node.instruction
            ));
        }

        for relation in self.relations {
            ret.push_str(&format!("c{} -> c{};\n", relation.from, relation.to));
        }

        ret.push_str("}\n");
        ret
    }
}

fn main() {
    let path = env::args()
        .nth(1)
        .expect("please specify a path to a .trace file");

    let content = fs::read_to_string(&path).expect("failed to read trace file");

    let traces = content
        .split("\n")
        .map(|s| TraceEntry::parse(s).expect("failed to parse trace entry"))
        .collect::<Vec<TraceEntry>>();

    let mut graph = Graph::default();
    let mut last_pc = Option::None;
    for trace in traces {
        let pc = trace.pc.clone();
        graph.add_trace_entry(trace, last_pc);
        last_pc = Some(pc);
    }
    println!("{}", graph.render());
}
