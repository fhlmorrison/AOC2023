use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    // let reader = read_to_string("./inputs/test20A.txt").unwrap();
    // let reader = read_to_string("./inputs/test20B.txt").unwrap();
    let reader = read_to_string("./inputs/day20.txt").unwrap();

    part_1(&reader);
    part_2(&reader);
}

fn part_1(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let modules = lines.map(|s| Module::from(s)).collect::<Vec<Module>>();

    // Initialize module map
    let mut module_map =
        HashMap::<String, Module>::from_iter(modules.iter().map(|m| (m.name().clone(), m.clone())));

    // Initialize conjunction states
    modules.iter().for_each(|module| {
        module.outputs().iter().for_each(|output| {
            if let Some(m) = module_map.get_mut(output) {
                if let Module::Conjunction(c) = m {
                    c.state.insert(module.name().clone(), false);
                }
            }
        })
    });

    let mut queue = VecDeque::<Pulse>::new();

    let start_pulse = Pulse {
        to: "broadcaster".to_string(),
        from: "button".to_string(),
        value: false,
    };

    let mut high_pulses = 0;
    let mut low_pulses = 0;

    for _ in 0..1000 {
        // One button press
        queue.push_back(start_pulse.clone());

        while let Some(current_pulse) = queue.pop_front() {
            if current_pulse.value {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }

            if let Some(m) = module_map.get_mut(&current_pulse.to) {
                m.process_pulse(current_pulse, &mut queue);
            }
        }
    }

    let sum = high_pulses * low_pulses;

    println!("Part 1: {} in {:?}", sum, start.elapsed());
}

fn part_2(reader: &str) {
    let start = Instant::now();
    let lines = reader.lines();

    let modules = lines.map(|s| Module::from(s)).collect::<Vec<Module>>();

    // Initialize module map
    let mut module_map =
        HashMap::<String, Module>::from_iter(modules.iter().map(|m| (m.name().clone(), m.clone())));

    // Initialize conjunction states
    modules.iter().for_each(|module| {
        module.outputs().iter().for_each(|output| {
            if let Some(m) = module_map.get_mut(output) {
                if let Module::Conjunction(c) = m {
                    c.state.insert(module.name().clone(), false);
                }
            }
        })
    });

    // Target is conjunction leading into rx
    let target = modules
        .iter()
        .find(|m| m.outputs().contains(&"rx".to_string()))
        .unwrap()
        .name()
        .clone();

    // Get period of conjunction leading into rx
    let sum = get_period(&mut module_map, target);

    println!("Part 2: {} in {:?}", sum, start.elapsed());
}

fn get_period(map: &mut HashMap<String, Module>, target: String) -> usize {
    let mut queue = VecDeque::<Pulse>::new();

    let start_pulse = Pulse {
        to: "broadcaster".to_string(),
        from: "button".to_string(),
        value: false,
    };

    // Trying to figure out what button press sends high pulses from each input to target at same time

    let inputs;

    // get all inputs to target
    if let Some(m) = map.get_mut(&target) {
        if let Module::Conjunction(c) = m {
            inputs = c.state.keys().map(|k| k.clone()).collect::<Vec<String>>();
        } else {
            panic!("Target not conjunction");
        }
    } else {
        panic!("Target not found");
    }

    let mut input_periods =
        HashMap::<String, usize>::from_iter(inputs.iter().map(|t| (t.clone(), 0)));

    let mut idx = 0;
    while input_periods.values().any(|v| v == &0) {
        idx += 1;
        // One button press
        queue.push_back(start_pulse.clone());

        while let Some(current_pulse) = queue.pop_front() {
            if current_pulse.to == target && current_pulse.value {
                input_periods.insert(current_pulse.from.clone(), idx);
            }

            if let Some(m) = map.get_mut(&current_pulse.to) {
                m.process_pulse(current_pulse, &mut queue);
            }
        }
    }
    input_periods.values().product::<usize>()
}

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    outputs: Vec<String>,
}
#[derive(Clone, Debug)]
struct Conjunction {
    name: String,
    state: HashMap<String, bool>,
    outputs: Vec<String>,
}
#[derive(Clone, Debug)]
struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

#[derive(Clone, Debug)]
struct Pulse {
    to: String,
    from: String,
    value: bool,
}

impl Module {
    fn outputs(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop(f) => &f.outputs,
            Module::Conjunction(c) => &c.outputs,
            Module::Broadcaster(b) => &b.outputs,
        }
    }
    fn name(&self) -> &String {
        match self {
            Module::FlipFlop(f) => &f.name,
            Module::Conjunction(c) => &c.name,
            Module::Broadcaster(b) => &b.name,
        }
    }
    fn process_pulse(&mut self, pulse: Pulse, queue: &mut VecDeque<Pulse>) {
        match self {
            Module::FlipFlop(f) => {
                // Do nothing on high pulses
                if pulse.value {
                    return ();
                }

                f.state = !f.state; // Flip the flop on low pulses

                // Push to outputs
                f.outputs.iter().for_each(|output| {
                    queue.push_back(Pulse {
                        to: output.clone(),
                        from: f.name.clone(),
                        value: f.state,
                    })
                })
            }
            Module::Conjunction(c) => {
                // Update received state
                c.state.insert(pulse.from, pulse.value);

                // Low if all are high, high otherwise
                let result = !c.state.iter().all(|(_, v)| *v);

                c.outputs.iter().for_each(|output| {
                    queue.push_back(Pulse {
                        to: output.clone(),
                        from: c.name.clone(),
                        value: result,
                    })
                })
            }
            Module::Broadcaster(b) => b.outputs.iter().for_each(|output| {
                queue.push_back(Pulse {
                    to: output.clone(),
                    from: b.name.clone(),
                    value: pulse.value,
                })
            }),
        }
    }
}

impl From<&str> for Module {
    fn from(s: &str) -> Self {
        let parts = s.split(" -> ").collect::<Vec<&str>>();
        let name;
        let outputs = parts[1].split(", ").map(|s| s.to_string()).collect();
        if s.starts_with('%') {
            name = parts[0][1..].to_string();
            Module::FlipFlop(FlipFlop {
                name,
                state: false,
                outputs,
            })
        } else if s.starts_with('&') {
            name = parts[0][1..].to_string();
            Module::Conjunction(Conjunction {
                name,
                state: HashMap::new(),
                outputs,
            })
        } else {
            name = parts[0].to_string();
            Module::Broadcaster(Broadcaster { name, outputs })
        }
    }
}
