use std::collections::{HashMap, HashSet};
use crate::puzzle::AoCPuzzle;
use std::fs::read_to_string;
use petgraph::{Graph};
use petgraph::graph::{NodeIndex};

pub struct Day8 {
    pub filename: String,

}

#[derive(Copy,Clone, Eq, PartialEq,Hash,Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64
}

fn dist(a : &Point, b : &Point) -> i64 {
    (a.x - b.x).abs().pow(2) + (a.y - b.y).abs().pow(2) + (a.z - b.z).abs().pow(2)
}

impl AoCPuzzle for Day8 {
    fn puzzle_name(&self) -> String {
        "Day 8 NOT DONE".to_string()
    }

    fn first_puzzle(&self) -> i64 {

        let mut points : Vec<Point> = Vec::new();

        for line in read_to_string(&self.filename).unwrap().lines() {
            let vals : Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            points.push(Point{x:vals[0],y:vals[1],z:vals[2]})
        }

        let mut distance_points : Vec<(i64,Point,Point)> = Vec::new();
        for i in 0..points.len() {
            for j in i+1..points.len() {
                distance_points.push((dist(&points[i], &points[j]), points[i], points[j]))
            }
        }

        distance_points.sort_by(|x,y| x.0.cmp(&y.0));


        let mut graph = Graph::<Point, i64>::new();
        let mut node_to_id : HashMap<Point,NodeIndex> = HashMap::new();
        let mut id_to_node : HashMap<NodeIndex,Point>= HashMap::new();

        for i in 0..10{
            let (dist, n1, n2) = distance_points[i];
            let mut add_if_necessary = |x| {
                match node_to_id.get(&x) {
                    Some(id) => *id,                 // id is &NodeId; return owned copy
                    None => {
                        let id = graph.add_node(x);  // id: NodeId
                        node_to_id.insert(x, id);    // store owned id
                        id_to_node.insert(id, x);    // also store owned id
                        id                            // return id (ownership to caller)
                    }
                }
            };

            let f_node = add_if_necessary(n1);
            let s_node = add_if_necessary(n2);

            graph.add_edge(f_node, s_node, dist);
        }



        distance_points[0].0.abs() + distance_points[1].0.abs() + distance_points[2].0.abs()

    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}