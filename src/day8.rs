use std::collections::HashMap;

use crate::parser;

pub fn answer() {
    match part1(true) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
    match part2(true) {
        Ok(_) => (),
        Err(e) => panic!("{e}"),
    };
}

fn part1(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day8/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day8/test.txt")?;
    }

    let instructions = input.iter().next().unwrap().chars().collect::<Vec<char>>();

    let nodes: HashMap<String, (String, String)> = input
        .iter()
        .skip(2)
        .map(|x| {
            let mut split = x.split("=");
            (
                split.next().unwrap().trim().to_string(),
                split.next().unwrap().trim().to_string(),
            )
        })
        .map(|(k, v)| {
            let mut split = v.split(",");
            (
                k,
                (
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .chars()
                        .filter(|x| *x != '(')
                        .collect::<String>(),
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .chars()
                        .filter(|x| *x != ')')
                        .collect::<String>(),
                ),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    let mut current_node = "AAA".to_string();
    let mut counter = 0;
    let mut instructions = instructions.iter().cycle();
    while current_node != "ZZZ" {
        match instructions.next().unwrap() {
            'L' => current_node = nodes[&current_node].0.clone(),
            'R' => current_node = nodes[&current_node].1.clone(),
            _ => panic!("Shouldn't be here!"),
        }

        counter += 1;
    }

    println!("Part 1: {counter}");

    Ok(())
}

fn part2(real: bool) -> Result<(), String> {
    let input: Vec<String>;
    if real {
        input = parser::read_by_line("inputs/day8/real.txt")?;
    } else {
        input = parser::read_by_line("inputs/day8/test.txt")?;
    }

    let instructions = input.iter().next().unwrap().chars().collect::<Vec<char>>();

    let nodes: HashMap<String, (String, String)> = input
        .iter()
        .skip(2)
        .map(|x| {
            let mut split = x.split("=");
            (
                split.next().unwrap().trim().to_string(),
                split.next().unwrap().trim().to_string(),
            )
        })
        .map(|(k, v)| {
            let mut split = v.split(",");
            (
                k,
                (
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .chars()
                        .filter(|x| *x != '(')
                        .collect::<String>(),
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .chars()
                        .filter(|x| *x != ')')
                        .collect::<String>(),
                ),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    let mut current_nodes = nodes
        .iter()
        .map(|(k, _)| k.clone())
        .filter(|x| x.chars().last().unwrap() == 'A')
        .collect::<Vec<String>>();
    let count = current_nodes.iter().count();
    let mut counter = 0;
    let mut instructions = instructions.iter().cycle();

    while !(current_nodes
        .iter()
        .filter(|x| x.chars().last().unwrap() == 'Z')
        .count()
        == count)
    {
        let instruction = instructions.next().unwrap();
        let mut new_nodes = Vec::<String>::new();
        for cn in &current_nodes {
            // println!("{:?}: {:?}, {:?}", cn, nodes[cn], instruction);
            match instruction {
                'L' => new_nodes.push(nodes[cn].0.clone()),
                'R' => new_nodes.push(nodes[cn].1.clone()),
                _ => panic!("Shouldn't be here"),
            }
        }
        current_nodes = new_nodes;
        // println!("{:?}", current_nodes);
        // thread::sleep(time::Duration::from_millis(500));
        counter += 1;
        if counter == 10 {
            break;
        }
    }

    println!("Part 2: unfinished");

    Ok(())
}
