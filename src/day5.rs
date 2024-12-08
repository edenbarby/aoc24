use crate::utils::*;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

// enum RuleCheckState {
//     FoundNothing,
//     FoundFirst,
//     FoundSecond,
// }

fn load_input(input_file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let file_contents = read_to_string(input_file_path).unwrap();
    let mut lines_iter = file_contents.trim().lines();

    loop {
        match lines_iter.next() {
            Some(line) => {
                let values = seperate_string_into_numbers(line, "|");

                match values.len() {
                    0 => break,
                    2 => rules.push((values[0], values[1])),
                    _ => panic!("bad line {}", line),
                }
            }
            None => panic!("reached EOF before update list"),
        }
    }

    for line in lines_iter {
        let values = seperate_string_into_numbers(line, ",");
        if !values.is_empty() {
            if values.len() % 2 == 1 {
                updates.push(values);
            } else {
                panic!("bad update page len {}", line);
            }
        }
    }

    (rules, updates)
}

pub fn part1() {
    let file_path = "input/05/input1.txt";

    let (rules, updates) = load_input(file_path);

    for (a, b) in &rules {
        println!("{}|{}", a, b);
    }

    let mut sum = 0;

    let mut rules_map_after: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rules_map_before: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules {
        rules_map_after
            .entry(rule.0)
            .and_modify(|v| v.push(rule.1))
            .or_insert(vec![rule.1]);
        rules_map_before
            .entry(rule.1)
            .and_modify(|v| v.push(rule.0))
            .or_insert(vec![rule.0]);
        // rules_map.insert(rule.0, rule.1);
    }
    for update in &updates {
        println!("update: {:?}", update);

        let mut rule_violation = false;

        // let mut all_pages = HashSet::new();
        let mut unvisited_pages = HashSet::new();
        for page in update {
            // all_pages.insert(page);
            unvisited_pages.insert(page);
        }
        let mut visited_pages = HashSet::new();

        for page in update {
            unvisited_pages.remove(page);

            if let Some(future_pages) = rules_map_after.get(page) {
                for future_page in future_pages {
                    if visited_pages.contains(future_page) {
                        rule_violation = true;
                        println!("1 violation {}|{}", page, future_page);
                        break;
                    }
                }
            }

            if let Some(past_pages) = rules_map_before.get(page) {
                for past_page in past_pages {
                    if unvisited_pages.contains(past_page) {
                        rule_violation = true;
                        println!("2 violation {}|{}", past_page, page);
                        break;
                    }
                }
            }

            visited_pages.insert(page);

            if rule_violation {
                break;
            }
        }

        if !rule_violation {
            let midpoint = update.len() / 2;
            sum += update[midpoint];
            println!("good!");
        } else {
            println!("bad!");
        }
    }

    // for pages in &updates {
    //     println!("update: {:?}", pages);

    //     let mut rules_violated = false;

    //     for (first, last) in &rules {
    //         let mut rule_check_state = RuleCheckState::FoundNothing;

    //         for page in pages {
    //             match rule_check_state {
    //                 RuleCheckState::FoundNothing => {
    //                     if page == first {
    //                         rule_check_state = RuleCheckState::FoundFirst;
    //                     } else if page == last {
    //                         rule_check_state = RuleCheckState::FoundSecond;
    //                     }
    //                 }
    //                 RuleCheckState::FoundFirst => {
    //                     if page == first {
    //                         panic!("?")
    //                     } else if page == last {
    //                         break;
    //                     }
    //                 }
    //                 RuleCheckState::FoundSecond => {
    //                     if page == first {
    //                         rules_violated = true;
    //                     } else if page == last {
    //                         panic!("?")
    //                     }
    //                 }
    //             }

    //             if rules_violated {
    //                 break;
    //             }
    //         }

    //         if rules_violated {
    //             println!("broken rule: {}|{}", first, last);
    //             break;
    //         }
    //     }

    //     if !rules_violated {
    //         let midpoint = pages.len() / 2;
    //         sum += pages[midpoint];
    //         println!("good!");
    //     } else {
    //         println!("bad!");
    //     }
    // }

    println!("day 5 part 1: {}", sum);
}

type PageOrderRules = HashMap<i32, HashSet<i32>>;

fn is_update_valid(pages: &[i32], order: &PageOrderRules) -> bool {
    let mut previous = HashSet::new();

    for &page in pages {
        if let Some(pages_that_should_come_after) = order.get(&page) {
            if !previous.is_disjoint(pages_that_should_come_after) {
                return false;
            }
        }

        previous.insert(page);
    }

    true
}

fn compare(first_page: i32, second_page: i32, order: &PageOrderRules) -> Ordering {
    if let Some(pages_that_should_come_after) = order.get(&first_page) {
        if pages_that_should_come_after.contains(&second_page) {
            return Ordering::Less;
        }
    }
    if let Some(pages_that_should_come_after) = order.get(&second_page) {
        if pages_that_should_come_after.contains(&first_page) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

pub fn part2() {
    let input_file_path = "input/05/input1.txt";
    let (rules, updates) = load_input(input_file_path);

    let mut rule_map_after: PageOrderRules = HashMap::new();
    for r in rules {
        rule_map_after.entry(r.0).or_default().insert(r.1);
    }

    let mut sum = 0;

    for update in updates {
        if !is_update_valid(&update, &rule_map_after) {
            let mut corrected_update = update;
            corrected_update.sort_by(|&a, &b| compare(a, b, &rule_map_after));

            sum += corrected_update[corrected_update.len() / 2];
        }
    }

    println!("day 5 part 2: {}", sum);
}
