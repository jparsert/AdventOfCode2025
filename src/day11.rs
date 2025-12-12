use std::collections::HashMap;
use std::fs::read_to_string;
use std::hash::RandomState;
use petgraph::graph::NodeIndex;
use petgraph::graph::DiGraph;
use crate::puzzle::AoCPuzzle;

pub struct Day11 {
    pub filename: String,
}

fn build_graph(filename : &String) -> (HashMap<String, NodeIndex>, DiGraph<String,()>){

    let mut device_to_id : HashMap<String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::new();


    for line in read_to_string(filename).unwrap().lines() {
        let l = line.split(":").map(|x| x.trim().to_string()).collect::<Vec<String>>();

        let out = l[1].split(" ").map(|x: &str| x.trim().to_string()).collect::<Vec<String>>();

        let index = if !device_to_id.contains_key(&l[0].to_string()) {
            let x : NodeIndex = graph.add_node(l[0].clone());
            device_to_id .insert(l[0].to_string().clone(), x);
            x
        } else{
            device_to_id.get(&l[0].to_string()).unwrap().clone()
        };

        for tgt in out {
            let out_idx  = if !device_to_id.contains_key(&tgt) {
                let x : NodeIndex = graph.add_node(tgt.clone());
                device_to_id .insert(tgt.to_string(), x);
                x
            } else{
                *device_to_id.get(&tgt).unwrap()
            };

            graph.add_edge(index.clone(), out_idx.clone(), ());
        }
    }

    (device_to_id, graph)
}

impl AoCPuzzle for Day11 {
    fn puzzle_name(&self) -> String {
        "Day 11".to_string()
    }

    fn first_puzzle(&self) -> i64 {

        let (device_to_id, graph) = build_graph(&self.filename);

        let source = device_to_id.get("you");
        let target = device_to_id.get("out");

        let paths = petgraph::algo::all_simple_paths::<Vec<_>,_,RandomState>(&graph, *source.unwrap(), *target.unwrap(), 0, None);

        paths.collect::<Vec<_>>().len() as i64
    }

    fn second_puzzle(&self) -> Option<i64> {
        return None;
        /*
        let (device_to_id, graph) = build_graph(&self.filename);

        let source = device_to_id.get("svr");
        let target = device_to_id.get("out");

        let paths = petgraph::algo::all_simple_paths::<Vec<_>,_,RandomState>(&graph, *source.unwrap(), *target.unwrap(), 0, None);

        let fft = device_to_id.get("fft").unwrap();
        let dac = device_to_id.get("dac").unwrap();


        for path in paths {
            println!("{}", path.len());
        }


        //Some(paths.filter(|x| x.contains(fft) && x.contains(dac)).count() as i64)
        */
    }
}