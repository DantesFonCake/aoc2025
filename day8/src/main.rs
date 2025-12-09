use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Result};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day8/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = JunctionBoxes;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let mut map = DistanceMap::new(input.0);
        let mut closest = map.distances.iter().map(|(&k, &v)| (k ,v)).collect::<Vec<_>>();
        closest.sort_unstable_by(|(_, v1), (_, v2)| v1.total_cmp(v2));
        for ((first, second), _) in closest.iter().take(1000) {
            map.connect(*first, *second);
        }

        let mut circuits = HashMap::new();
        for circuit in map.circuits_per_box.into_iter().filter_map(|c| c) {
            *circuits.entry(circuit).or_insert(0) += 1;
        }

        let mut circuits = circuits.into_iter().collect::<Vec<_>>();
        circuits.sort_unstable_by_key(|(_, c)| *c);

        circuits.into_iter().rev().take(3).map(|(_, c)| c).product()
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let mut map = DistanceMap::new(input.0);
        let mut closest = map.distances.iter().map(|(&k, &v)| (k ,v)).collect::<Vec<_>>();
        closest.sort_unstable_by(|(_, v1), (_, v2)| v1.total_cmp(v2));
        for ((first, second), _) in closest.iter() {
            map.connect(*first, *second);
            let mut circuits = map.circuits_per_box.iter();
            if let Some(circuit) = circuits.next() && circuit.is_some() && circuits.all(|i| i == circuit) {
                return map.boxes[*first].0 * map.boxes[*second].0;
            }
        }

        unreachable!()
    }
}

struct DistanceMap {
    boxes: Vec<Coord>,
    distances: HashMap<(usize, usize), f64>,
    boxes_per_circuit: HashMap<usize, HashSet<usize>>,
    circuits_per_box: Vec<Option<usize>>,
    cur_circuit: usize,
}

impl DistanceMap {
    fn new(boxes: Vec<Coord>) -> Self {
        let len = boxes.len();
        let mut distances = HashMap::with_capacity(len * len / 2);
        let boxes_per_circuit = HashMap::new();
        let circuits_per_box = vec![None; len];
        for first in 0..len {
            for second in (first + 1)..len {
                let distance = boxes[first].distance(boxes[second]);
                let f = first.max(second);
                let s = first.min(second);
                distances.insert((f, s), distance);
            }
        }

        Self {
            boxes,
            distances,
            boxes_per_circuit,
            circuits_per_box,
            cur_circuit: 0,
        }
    }

    fn connect(&mut self, first: usize, second: usize) {
        if let Some(first_circuit) = self.circuits_per_box[first] {
            self.merge_into(first_circuit, second);
        } else if let Some(second_circuit) = self.circuits_per_box[second] {
            self.merge_into(second_circuit, first);
        } else {
            self.circuits_per_box[first] = Some(self.cur_circuit);
            self.circuits_per_box[second] = Some(self.cur_circuit);
            self.boxes_per_circuit
                .entry(self.cur_circuit)
                .or_default()
                .extend([first, second]);
            self.cur_circuit += 1;
        }
    }

    fn merge_into(&mut self, target_circuit: usize, r#box: usize) {
        if let Some(second_circuit) = self.circuits_per_box[r#box] {
            let circuit_to_merge = self
                .boxes_per_circuit
                .remove(&second_circuit)
                .unwrap_or_default();
            self.boxes_per_circuit
                .entry(target_circuit)
                .or_default()
                .extend(circuit_to_merge);
        } else {
            self.boxes_per_circuit.entry(target_circuit).or_default().insert(r#box);
        }
        let merged_circuit = self.boxes_per_circuit.get(&target_circuit);
        for merged_box in merged_circuit.into_iter().flatten() {
            self.circuits_per_box[*merged_box] = Some(target_circuit);
        }
    }
}

struct JunctionBoxes(Vec<Coord>);

#[derive(Clone, Copy, Debug)]
struct Coord(usize, usize, usize);

impl Coord {
    fn distance(&self, other: Coord) -> f64 {
        let squared = (self.0 as f64 - other.0 as f64).powi(2)
            + (self.1 as f64 - other.1 as f64).powi(2)
            + (self.2 as f64 - other.2 as f64).powi(2);
        squared.sqrt()
    }
}

impl TaskInput for JunctionBoxes {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let mut res = vec![];
        for line in reader.lines() {
            let line = line?;
            let mut coord = line.split(',');
            let x = coord.next().unwrap().parse().unwrap();
            let y = coord.next().unwrap().parse().unwrap();
            let z = coord.next().unwrap().parse().unwrap();
            res.push(Coord(x, y, z));
        }

        Ok(JunctionBoxes(res))
    }
}
