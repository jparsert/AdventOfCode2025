use std::collections::{HashMap};
use crate::puzzle::AoCPuzzle;
use std::fs::read_to_string;
use union_find::{UnionFind, QuickFindUf, UnionByRank};

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
        "Day 8".to_string()
    }

    fn first_puzzle(&self) -> i64 {

        let mut points : Vec<Point> = Vec::new();
        let mut node_to_id : HashMap<Point, usize> = HashMap::new();

        for (i, line) in read_to_string(&self.filename).unwrap().lines().enumerate() {
            let vals : Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
            let pt = Point{x:vals[0],y:vals[1],z:vals[2]};
            points.push(pt);
            node_to_id.insert(pt, i);
        }

        let mut distance_points : Vec<(i64,Point,Point)> = Vec::new();
        for i in 0..points.len() {
            for j in i+1..points.len() {
                distance_points.push((dist(&points[i], &points[j]), points[i], points[j]))
            }
        }

        distance_points.sort_by(|x,y| x.0.cmp(&y.0));
        let mut uf = QuickFindUf::<UnionByRank>::new(node_to_id.len());

        for i in 0..1000{
            let (_, n1, n2) = distance_points[i];
            uf.union(node_to_id[&n1], node_to_id[&n2]);
        }

        let mut cnts = vec![0; node_to_id.len()];

        for i in 0..node_to_id.len() {
            cnts[uf.find(i)] += 1;
        }

        cnts.sort_by(|x,y| y.cmp(&x));

        cnts[0] * cnts[1] * cnts[2]
    }

    fn second_puzzle(&self) -> Option<i64> {
        None
    }
}