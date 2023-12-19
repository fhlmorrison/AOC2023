use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test19A.txt").unwrap();
    let reader = read_to_string("./inputs/day19.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    // let lines = reader.lines();
    let mut split_1 = reader.split("\r\n\r\n");
    let workflow_lines = split_1.next().unwrap().lines();
    let part_lines = split_1.next().unwrap().lines();

    let workflows = workflow_lines
        .map(|line| Workflow::from(line))
        .collect::<Vec<Workflow>>();

    let parts = part_lines
        .map(|line| Part::from(line))
        .collect::<Vec<Part>>();

    let mut workflow_map = HashMap::new();

    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow);
    }

    let start_workflow = workflow_map.get("in").unwrap();

    let accepted_parts = parts.iter().filter(|part| {
        let mut current_flow = start_workflow;

        loop {
            for rule in current_flow.rules.iter() {
                if rule.condition == Condition::Any {
                    if rule.output == "A" {
                        return true;
                    }
                    if rule.output == "R" {
                        return false;
                    }
                    current_flow = workflow_map.get(&rule.output).unwrap();
                    break;
                }
                let val_to_check = match rule.field {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    c => {
                        panic!(
                            "Unknown field: \'{}\' in part {} workflow {}",
                            c, part.x, current_flow.name
                        )
                    }
                };
                match rule.condition {
                    Condition::Any => {}
                    Condition::Lesser => {
                        if val_to_check < rule.value {
                            if rule.output == "A" {
                                return true;
                            }
                            if rule.output == "R" {
                                return false;
                            }
                            current_flow = workflow_map.get(&rule.output).unwrap();
                            break;
                        }
                    }
                    Condition::Greater => {
                        if val_to_check > rule.value {
                            if rule.output == "A" {
                                return true;
                            }
                            if rule.output == "R" {
                                return false;
                            }
                            current_flow = workflow_map.get(&rule.output).unwrap();
                            break;
                        }
                    }
                }
            }
        }
    });

    let sum = accepted_parts.fold(0, |acc, part| acc + part.score());

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();

    let mut split_1 = reader.split("\r\n\r\n");
    let workflow_lines = split_1.next().unwrap().lines();

    let workflows = workflow_lines
        .map(|line| Workflow::from(line))
        .collect::<Vec<Workflow>>();

    let mut workflow_map = HashMap::new();

    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow);
    }

    let start_range = PartRange {
        x_min: 1,
        x_max: 4000,
        m_min: 1,
        m_max: 4000,
        a_min: 1,
        a_max: 4000,
        s_min: 1,
        s_max: 4000,
    };

    let mut sum = 0;

    let mut stack = Vec::new();
    stack.push(("in".to_string(), start_range));

    while let Some((workflow_name, mut current_range)) = stack.pop() {
        let current_flow = workflow_map.get(&workflow_name).unwrap();

        for rule in current_flow.rules.iter() {
            if rule.condition == Condition::Any {
                if rule.output == "A" {
                    sum += current_range.permutations();
                    break;
                }
                if rule.output == "R" {
                    continue;
                }
                stack.push((rule.output.clone(), current_range));
                break;
            }
            let mut range_to_check = match rule.field {
                'x' => (current_range.x_min, current_range.x_max),
                'm' => (current_range.m_min, current_range.m_max),
                'a' => (current_range.a_min, current_range.a_max),
                's' => (current_range.s_min, current_range.s_max),
                c => {
                    panic!("Unknown field: \'{}\' in workflow {}", c, current_flow.name)
                }
            };

            let mut range_to_mutate = range_to_check;

            if rule.condition == Condition::Lesser {
                if range_to_check.0 < rule.value && range_to_check.1 >= rule.value {
                    // Push reduced max with output workflow
                    range_to_check.1 = rule.value - 1;
                    // Mutate current range to be increased min
                    range_to_mutate.0 = rule.value;
                } else {
                    continue;
                }
            } else {
                if range_to_check.0 <= rule.value && range_to_check.1 > rule.value {
                    // Push increased min with output workflow
                    range_to_check.0 = rule.value + 1;
                    // Mutate current range to be reduced max
                    range_to_mutate.1 = rule.value;
                } else {
                    continue;
                }
            }

            let mut new_range = current_range;

            match rule.field {
                'x' => {
                    (current_range.x_min, current_range.x_max) = range_to_mutate;
                    (new_range.x_min, new_range.x_max) = range_to_check;
                }
                'm' => {
                    (current_range.m_min, current_range.m_max) = range_to_mutate;
                    (new_range.m_min, new_range.m_max) = range_to_check;
                }
                'a' => {
                    (current_range.a_min, current_range.a_max) = range_to_mutate;
                    (new_range.a_min, new_range.a_max) = range_to_check;
                }
                's' => {
                    (current_range.s_min, current_range.s_max) = range_to_mutate;
                    (new_range.s_min, new_range.s_max) = range_to_check;
                }
                c => {
                    panic!("Unknown field: \'{}\' in workflow {}", c, current_flow.name)
                }
            }

            if rule.output == "A" {
                sum += new_range.permutations();
                continue;
            }
            if rule.output == "R" {
                continue;
            }
            stack.push((rule.output.clone(), new_range));
        }
    }

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

#[derive(Debug, PartialEq, Eq)]
enum Condition {
    Any,
    Lesser,
    Greater,
}

struct Rule {
    field: char,
    condition: Condition,
    value: isize,
    output: String,
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        if s.contains('<') {
            let split = s.split('<').collect::<Vec<&str>>();
            let field = split[0].chars().next().unwrap();
            let condition = Condition::Lesser;
            let split_2 = split[1].split(':').collect::<Vec<&str>>();
            let value = split_2[0].parse::<isize>().unwrap();
            let output = split_2[1].to_string();
            return Self {
                field,
                condition,
                value,
                output,
            };
        } else if s.contains('>') {
            let split = s.split('>').collect::<Vec<&str>>();
            let field = split[0].chars().next().unwrap();
            let condition = Condition::Greater;
            let split_2 = split[1].split(':').collect::<Vec<&str>>();
            let value = split_2[0].parse::<isize>().unwrap();
            let output = split_2[1].to_string();
            return Self {
                field,
                condition,
                value,
                output,
            };
        } else {
            return Self {
                condition: Condition::Any,
                field: ' ',
                value: 0,
                output: s.to_string(),
            };
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        let mut split = s.strip_suffix("}").unwrap().split("{");
        let name = split.next().unwrap().to_string();
        let rules = split
            .next()
            .unwrap()
            .split(',')
            .map(|s| Rule::from(s))
            .collect::<Vec<Rule>>();

        Self { name, rules }
    }
}

struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}
impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let stripped = s.strip_prefix("{").unwrap().strip_suffix("}").unwrap();
        let split = stripped.split(",").collect::<Vec<&str>>();
        let x = split[0].split("=").collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();
        let m = split[1].split("=").collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();
        let a = split[2].split("=").collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();
        let s = split[3].split("=").collect::<Vec<&str>>()[1]
            .parse::<isize>()
            .unwrap();
        Self { x, m, a, s }
    }
}

impl Part {
    fn score(&self) -> isize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    x_min: isize,
    x_max: isize,
    m_min: isize,
    m_max: isize,
    a_min: isize,
    a_max: isize,
    s_min: isize,
    s_max: isize,
}
impl PartRange {
    fn permutations(&self) -> isize {
        (self.x_max - self.x_min + 1)
            * (self.m_max - self.m_min + 1)
            * (self.a_max - self.a_min + 1)
            * (self.s_max - self.s_min + 1)
    }
}
