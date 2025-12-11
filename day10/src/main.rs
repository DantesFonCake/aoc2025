use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Read, Result};
use microlp::{LinearExpr, OptimizationDirection, Problem};
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day10/src/input.txt")
}

struct Task;

impl util::Task for Task {
    type Input = Machines;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let mut res = 0usize;
        for machine in input.0 {
            res += count_enable(&machine);
        }

        res
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let mut res = 0usize;
        for machine in input.0 {
            res += count_joltage(&machine);
            //println!("one done");
        }

        res
    }
}

fn count_enable(machine: &Machine) -> usize {
    let mut to_see = VecDeque::new();
    let mut seen = HashMap::new();
    let root = vec![false; machine.light_req.len()];
    seen.insert(root.clone(), None);
    to_see.push_front(root);
    while let Some(state) = to_see.pop_back() {
        if state.eq(machine.light_req.as_slice()) {
            return collect_path(seen, state);
        }

        for button in machine.buttons.iter() {
            let mut new_state = state.clone();
            button.iter().for_each(|&b| new_state[b] = !new_state[b]);

            match seen.entry(new_state) {
                Entry::Occupied(_) => continue,
                Entry::Vacant(entry) => {
                    to_see.push_front(entry.key().clone());
                    entry.insert(Some(state.clone()));
                }
            }
        }
    }

    unreachable!();

    fn collect_path(
        mut seen: HashMap<Vec<bool>, Option<Vec<bool>>>,
        mut end_state: Vec<bool>,
    ) -> usize {
        let mut res = 0usize;
        while let Some(Some(prev_state)) = seen.remove(&end_state) {
            end_state = prev_state;
            res += 1;
        }

        res
    }
}

//cheating
fn count_joltage(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let target_max = machine.joltages.iter().copied().max().unwrap();
    let buttons = machine.buttons.iter().map(|_| problem.add_integer_var(1.0, (0, target_max as i32))).collect::<Vec<_>>();
    for (joltagei, &req) in machine.joltages.iter().enumerate() {
        let mut expr = LinearExpr::empty();
        for (b, b_var) in machine.buttons.iter().zip(buttons.iter()) {
            if b.contains(&joltagei) {
                expr.add(*b_var, 1.0);
            }
        }
        problem.add_constraint(expr, microlp::ComparisonOp::Eq, req as f64);
    }

    let solution = problem.solve().unwrap();
    solution.objective().round() as usize
}

// fn count_joltage(machine: &Machine) -> usize {
//     let h = |state: &[usize]| *state.iter().max().unwrap();
//     let start: Rc<[usize]> = Rc::from(machine.joltages.clone());
//     let mut open_set = BTreeSet::<Node>::new();
//     open_set.insert(Node(h(start.as_ref()), Rc::clone(&start)));
//     let mut came_from: HashMap<Rc<_>, Rc<[usize]>> = HashMap::new();
//     let mut g_score = HashMap::new();
//     g_score.insert(Rc::clone(&start), 0usize);
//     let mut f_score = HashMap::new();
//     f_score.insert(Rc::clone(&start), h(start.as_ref()));
//
//     while let Some(current) = open_set.pop_first() {
//         if current.1.as_ref().iter().all(|&j| j == 0) {
//             let mut path = Rc::clone(&current.1);
//             while let Some(state) = came_from.remove(&path) {
//                 let diff = state.iter().zip(path.iter()).map(|(&a, &b)| a - b).collect::<Vec<_>>();
//                 println!("{state:?} - {diff:?}");
//                 path = state;
//             }
//             let f_score = f_score.get(&current.1).unwrap();
//             return *f_score;
//         }
//
//         for button in machine.buttons.iter() {
//             //let max_presses = button.iter().map(|&b| current.1.as_ref()[b]).min().unwrap();
//             for presses in (1..=1).rev() {
//                 let mut neighbor = current.1.as_ref().to_owned();
//                 button.iter().for_each(|&b| neighbor[b] -= presses);
//                 let neighbor = Rc::from(neighbor);
//                 let tentative_g_score = g_score.get(&current.1).unwrap().saturating_add(presses);
//                 if tentative_g_score < *g_score.entry(Rc::clone(&neighbor)).or_insert(usize::MAX) {
//                     came_from.insert(Rc::clone(&neighbor), Rc::clone(&current.1));
//                     g_score.insert(Rc::clone(&neighbor), tentative_g_score);
//                     let heuristic = h(neighbor.as_ref());
//                     let node_f_score = tentative_g_score.saturating_add(heuristic);
//                     f_score.insert(Rc::clone(&neighbor), node_f_score);
//                     open_set.insert(Node(node_f_score, neighbor));
//                 }
//             }
//         }
//     }
//
//     unreachable!();
//     struct Node(usize, Rc<[usize]>);
//
//     impl Eq for Node {}
//     impl PartialEq<Self> for Node {
//         fn eq(&self, other: &Self) -> bool {
//             self.1 == other.1
//         }
//     }
//     impl PartialOrd<Self> for Node {
//         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//             Some(self.cmp(other))
//         }
//     }
//     impl Ord for Node {
//         fn cmp(&self, other: &Self) -> Ordering {
//             self.0.cmp(&other.0).then(self.1.cmp(&other.1))
//         }
//     }
// }

struct Machines(Vec<Machine>);

struct Machine {
    light_req: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl TaskInput for Machines {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let mut res = vec![];
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let mut components = line.split(' ');
            let light_req = components
                .next()
                .unwrap()
                .trim_matches(|c| matches!(c, '[' | ']'))
                .chars()
                .map(|c| c == '#')
                .collect();
            let joltages = components
                .next_back()
                .unwrap()
                .trim_matches(|c| matches!(c, '{' | '}'))
                .split(',')
                .map(|l| l.parse().unwrap())
                .collect();
            let buttons = components
                .take_while(|b| b.starts_with('('))
                .map(|b| b.trim_matches(|c| matches!(c, '(' | ')')))
                .map(|b| b.split(',').map(|l| l.parse().unwrap()).collect())
                .collect();

            res.push(Machine {
                light_req,
                buttons,
                joltages,
            });
        }

        Ok(Machines(res))
    }
}
