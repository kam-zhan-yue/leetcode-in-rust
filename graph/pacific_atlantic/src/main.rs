use std::collections::{HashSet, VecDeque};

struct Queue(VecDeque<(usize, usize, Ocean)>);

impl Queue {
    pub fn new() -> Self {
        Queue(VecDeque::new())
    }
}

struct Ocean(HashSet<(usize, usize)>);

impl Ocean {
    pub fn new() -> Self {
        Ocean(HashSet::new())
    }
}

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows: usize = heights.len();
        let cols: usize = heights[0].len();
        let mut graph: Vec<Vec<i32>> = Vec::new();
        let mut pacific: Ocean = Ocean::new();
        let mut atlantic: Ocean = Ocean::new();
        let mut queue: Queue = Queue::new();
        // Multi Source BFS
        for i in 0..rows {
            queue.0.push_back((0 as usize, i as usize, pacific));
        }
        for i in 0..cols {}
        graph
    }
}

fn main() {
    let ocean = vec![
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    println!("{:?}", Solution::pacific_atlantic(ocean));
}
