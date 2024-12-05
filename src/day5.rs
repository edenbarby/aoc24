use crate::utils::*;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

enum RuleCheckState {
    FoundNothing,
    FoundFirst,
    FoundSecond,
}

pub fn part1() {
    let file_path = "input/05/input1.txt";

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    {
        let file_contents = read_to_string(file_path).unwrap();
        let mut lines_iter = file_contents.trim().lines();
        loop {
            match lines_iter.next() {
                Some(line) => {
                    let values = seperate_string_into_numbers(line, "|");
                    // let tokens: Vec<&str> = line
                    //     .trim()
                    //     .split("|")
                    //     .skip_while(|t| t.len() == 0)
                    //     .collect();

                    match values.len() {
                        0 => break,
                        2 => rules.push((values[0], values[1])),
                        _ => panic!("bad line {}", line),
                    }
                }
                None => panic!("reach EOF before update list"),
            }
        }

        loop {
            match lines_iter.next() {
                Some(line) => {
                    let values = seperate_string_into_numbers(line, ",");
                    if values.len() > 0 {
                        if values.len() % 2 == 1 {
                            updates.push(values);
                        } else {
                            panic!("bad update page len {}", line);
                        }
                    }
                }
                None => break,
            }
        }
    }

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

            match rules_map_after.get(page) {
                Some(future_pages) => {
                    for future_page in future_pages {
                        if visited_pages.contains(future_page) {
                            rule_violation = true;
                            println!("1 violation {}|{}", page, future_page);
                            break;
                        }
                    }
                }
                None => {}
            }

            match rules_map_before.get(page) {
                Some(past_pages) => {
                    for past_page in past_pages {
                        if unvisited_pages.contains(past_page) {
                            rule_violation = true;
                            println!("2 violation {}|{}", past_page, page);
                            break;
                        }
                    }
                }
                None => {}
            }

            visited_pages.insert(page);

            if rule_violation {
                break;
            }
        }

        if !rule_violation {
            let midpoint = (update.len() >> 2) + 1;
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
    //         let midpoint = (pages.len() >> 2) + 1;
    //         sum += pages[midpoint];
    //         println!("good!");
    //     } else {
    //         println!("bad!");
    //     }
    // }

    println!("day 5 part 1: {}", sum);
}
