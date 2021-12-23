#![feature(derive_default_enum)]
#![feature(hash_drain_filter)]
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
enum Cave {
    #[default]
    Start,
    Big(String),
    Small(String),
    End,
}

impl Cave {
    fn new(label: &str) -> Self {
        if label == "start" {
            Cave::Start
        } else if label == "end" {
            Cave::End
        } else {
            if label.chars().all(|y| y.is_uppercase()) {
                Cave::Big(label.to_owned())
            } else {
                Cave::Small(label.to_owned())
            }
        }
    }
}

#[derive(Default, Debug)]
struct Graph {
    edges: HashMap<Cave, HashSet<Cave>>,
}

impl Graph {
    fn add_edge(&mut self, start: Cave, end: Cave) {
        if let Some(set) = self.edges.get_mut(&start) {
            set.insert(end.clone());
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(end.clone());
            self.edges.insert(start.clone(), new_set);
        }
        if let Some(set) = self.edges.get_mut(&end) {
            set.insert(start);
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(start);
            self.edges.insert(end, new_set);
        }
    }

    fn get_neighbours(&self, c: &Cave) -> Option<&HashSet<Cave>> {
        self.edges.get(&c)
    }
}

#[derive(Clone, Debug)]
struct GraphPath {
    visits: Vec<Cave>,
}

impl GraphPath {
    fn new() -> Self {
        GraphPath {
            visits: vec![Cave::Start],
        }
    }

    fn has_terminated(&self) -> bool {
        *self.visits.last().unwrap() == Cave::End
    }

    fn has_already_visisted_small_cave_twice(&self) -> bool {
        let visited_small_caves: Vec<&Cave> = self
            .visits
            .iter()
            .filter(|y| if let Cave::Small(_) = y { true } else { false })
            .collect();

        for visited_cave in &visited_small_caves {
            if visited_small_caves
                .iter()
                .filter(|&y| y == visited_cave)
                .count()
                >= 2
            {
                return true;
            }
        }

        false
    }

    fn get_possible_next_paths(&self, graph: &Graph) -> VecDeque<GraphPath> {
        let mut next_paths = VecDeque::new();

        if self.has_terminated() {
            return next_paths;
        }

        let mut neighbours_to_last_visited = if let Some(last_visit) = self.visits.last() {
            graph.get_neighbours(last_visit)
        } else {
            graph.get_neighbours(&Cave::Start)
        }
        .unwrap()
        .clone();

        //eliminate start and visited small caves
        neighbours_to_last_visited.drain_filter(|y| {
            if *y == Cave::Start {
                true
            } else if let Cave::Small(_) = y {
                let has_twice_visited = self.has_already_visisted_small_cave_twice();
                self.visits.contains(&y) && has_twice_visited
            } else {
                false
            }
        });

        for possible_neighbour in neighbours_to_last_visited {
            //build a new path with the contents of the self
            let mut copy_of_current = (*self).clone();
            copy_of_current.visits.push(possible_neighbour);
            next_paths.push_back(copy_of_current);
        }

        next_paths
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Provide the cave input text file!");
    }

    let file = File::open(&args[1]).expect("Error opening file");
    let reader = BufReader::new(file);

    let mut graph = Graph::default();
    for line in reader.lines() {
        let text_line = line.unwrap();
        let components: Vec<&str> = text_line.trim().split('-').collect();
        let start = Cave::new(components[0]);
        let end = Cave::new(components[1]);
        graph.add_edge(start, end);
    }

    let mut graph_paths: VecDeque<GraphPath> = VecDeque::new();
    graph_paths.push_back(GraphPath::new());
    loop {
        if let Some(current_path) = graph_paths.pop_front() {
            if !current_path.has_terminated() {
                let mut possible_next_paths = current_path.get_possible_next_paths(&graph);
                graph_paths.append(&mut possible_next_paths);
            } else {
                graph_paths.push_back(current_path);
            }
        } else {
            break;
        }
        if graph_paths.iter().all(|y| y.has_terminated()) {
            break;
        }
    }

    println!("There are {} paths", graph_paths.len());
}
