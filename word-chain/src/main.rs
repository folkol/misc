use std::collections::{HashMap, VecDeque};
use std::env::args;
use std::io::stdin;

use edit_distance::edit_distance;

// http://codekata.com/kata/kata19-word-chains/
fn main() {
    let mut dictionary: Vec<_> = stdin().lines().map_while(Result::ok).collect();
    dictionary.sort_by_key(|x| x.len());
    let mut alphabet: Vec<_> = dictionary.iter().flat_map(|s| s.chars()).collect();
    alphabet.sort();
    alphabet.dedup();

    let from = args().nth(1).expect("Expected from");
    let to = args().nth(2).expect("Expected to");
    println!("{from} {to}");

    let mut start_queue: VecDeque<String> = VecDeque::new();
    start_queue.push_back(from.clone());
    let mut end_queue: VecDeque<String> = VecDeque::new();
    end_queue.push_back(to.clone());
    let mut seen_from_start: HashMap<String, String> = HashMap::new();
    seen_from_start.insert(from.clone(), "".to_owned());
    let mut seen_from_end: HashMap<String, String> = HashMap::new();
    seen_from_end.insert(to.clone(), "".to_owned());

    while !start_queue.is_empty() || !end_queue.is_empty() {
        // println!("queue sizes: {}, {}", start_queue.len(), end_queue.len());
        if do_step(
            &dictionary,
            &mut start_queue,
            &mut seen_from_start,
            &mut seen_from_end,
            &from,
            &to,
        ) {
            println!("found path");
            break;
        }
        if do_step(
            &dictionary,
            &mut end_queue,
            &mut seen_from_end,
            &mut seen_from_start,
            &to,
            &from,
        ) {
            println!("found path");
            break;
        }
    }
    if start_queue.is_empty() && end_queue.is_empty() {
        println!("Couldn't find a path");
    }
}

fn do_step(
    dictionary: &Vec<String>,
    my_queue: &mut VecDeque<String>,
    my_seen: &mut HashMap<String, String>,
    their_seen: &mut HashMap<String, String>,
    from: &String,
    to: &String,
) -> bool {
    if let Some(node) = my_queue.pop_front() {
        if their_seen.contains_key(&node) {
            println!("Found path, reconstructing");
            let mut path: Vec<String> = Vec::new();
            path.insert(0, node.clone());
            let mut parent = my_seen.get(&node).unwrap().to_owned();
            loop {
                if &parent == "" {
                    break;
                }
                path.insert(0, parent.to_owned());
                if &parent == from {
                    break;
                }
                parent = my_seen.get(&parent).unwrap().to_owned();
            }
            let mut parent = their_seen.get(&node).unwrap().to_owned();
            loop {
                if &parent == "" {
                    break;
                }
                path.push(parent.to_owned());
                if &parent == to {
                    break;
                }
                parent = their_seen.get(&parent).unwrap().to_owned();
            }
            println!("path: {path:?}");

            return true;
        }

        let mut from = 0;
        for l in node.len() - 2..=0 {
            if let Ok(idx) = dictionary.binary_search_by_key(&l, |x| x.len()) {
                from = idx;
                break;
            }
        }

        let mut to = dictionary.len() - 1;
        for l in node.len() + 2..25 {
            if let Ok(idx) = dictionary.binary_search_by_key(&l, |x| x.len()) {
                to = idx;
                break;
            }
        }

        for neighbour in &dictionary[from..to] {
            if my_seen.contains_key(neighbour) {
                continue;
            }
            if edit_distance(neighbour, &node) != 1 {
                continue;
            }
            my_seen.insert(neighbour.clone(), node.clone());
            my_queue.push_back(neighbour.clone());
        }
    }
    false
}
