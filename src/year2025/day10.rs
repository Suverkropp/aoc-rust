use crate::parsers::parse_with_delimiters;
use itertools::Itertools;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

struct LightDiagram(Vec<bool>);

impl Debug for LightDiagram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lights = self
            .0
            .iter()
            .map(|l| if *l { '#' } else { '.' })
            .collect::<String>();
        let light_diagram = format!("[{lights}]");
        f.write_str(&light_diagram)
    }
}

#[derive(Debug)]
pub struct Machine {
    light_diagram: LightDiagram,
    button_wiring_schematics: Vec<Button>,
    joltage_requirements: Vec<i32>,
}

type Button = Vec<usize>;

pub fn handle_input(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(input: &str) -> Machine {
    let words: Vec<_> = input.split_whitespace().collect();
    let light_diagram = parse_light_diagram(words.first().expect("Line should have words"));
    let button_wiring_schematics = words[1..words.len() - 1]
        .iter()
        .map(|x| parse_button_wiring_schematic(*x))
        .collect();
    let joltage_requirements =
        parse_joltage_requirements(words.last().expect("Line should have words"));
    Machine {
        light_diagram,
        button_wiring_schematics,
        joltage_requirements,
    }
}

fn parse_light_diagram(input: &str) -> LightDiagram {
    parse_with_delimiters('[', ']', input, |lights| {
        LightDiagram(
            lights
                .chars()
                .map(|l| match l {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Light diagram cannot contain {l}"),
                })
                .collect::<Vec<_>>(),
        )
    })
}

fn parse_button_wiring_schematic(input: &str) -> Button {
    parse_with_delimiters('(', ')', input, |content| {
        content.split(',').map(|s| s.parse().unwrap()).collect()
    })
}

fn parse_joltage_requirements(input: &str) -> Vec<i32> {
    parse_with_delimiters('{', '}', input, |content| {
        content.split(',').map(|s| s.parse().unwrap()).collect()
    })
}

pub fn part1(machines: &Vec<Machine>) -> i32 {
    machines.iter().map(start_machine).sum()
}

fn start_machine(machine: &Machine) -> i32 {
    for presses in 0..machine.light_diagram.0.len() {
        if machine
            .button_wiring_schematics
            .iter()
            .combinations(presses)
            .any(|buttons| buttons_turn_on_machine(buttons, &machine.light_diagram))
        {
            return presses as i32;
        }
    }
    panic!("No solution found for machine {machine:?}");
}

fn buttons_turn_on_machine(buttons: Vec<&Button>, light_diagram: &LightDiagram) -> bool {
    let mut lights = vec![false; light_diagram.0.len()];
    press_buttons_for_lights(&mut lights, buttons);
    lights == light_diagram.0
}

fn press_buttons_for_lights(lights: &mut Vec<bool>, buttons: Vec<&Button>) {
    for button in buttons {
        press_button_for_lights(lights, button);
    }
}

fn press_button_for_lights(lights: &mut Vec<bool>, button: &Button) {
    for light in button {
        lights[*light] = !lights[*light];
    }
}

pub fn part2(machines: Vec<Machine>) -> i32 {
    let counter = Arc::new(AtomicU32::new(0));
    let handles: Vec<_> = machines
        .into_iter()
        .map(|machine| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                let result = configure_machine(&machine);
                let number = counter.fetch_add(1, Ordering::Relaxed);
                println!("Result for machine {number}: {machine:?} is {result:?}.");
                result.expect(&format!("Machine {machine:?} should have a solution."))
            })
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

struct RunningMachine {
    buttons: Vec<Button>,
    button_availability: Vec<bool>,
    joltage_requirements: Vec<i32>,
    buttons_pressed: i32,
}

impl RunningMachine {
    fn from_machine(machine: &'_ Machine) -> RunningMachine {
        RunningMachine {
            buttons: machine.button_wiring_schematics.clone(),
            button_availability: vec![true; machine.button_wiring_schematics.len()],
            joltage_requirements: machine.joltage_requirements.clone(),
            buttons_pressed: 0,
        }
    }
}

fn configure_machine(machine: &Machine) -> Option<i32> {
    let mut running_machine = RunningMachine::from_machine(machine);
    running_machine.buttons.sort_by_key(Vec::len);
    running_machine.buttons.reverse();
    find_least_presses(&mut running_machine, None)
}

fn find_least_presses(machine: &mut RunningMachine, previous_result: Option<i32>) -> Option<i32> {
    // Guaranties:
    // return value is less than or equal to previous_result
    // buttons list is restored to initial position
    if requirements_met(machine) && previous_result.is_none_or(|x| x > machine.buttons_pressed) {
        return Some(machine.buttons_pressed);
    }
    if joltage_overpowered(machine)
        || !buttons_available(machine)
        || previous_result.is_some_and(|x| x <= machine.buttons_pressed)
    {
        return previous_result;
    }

    let useless_buttons = useless_buttons(machine);
    if !useless_buttons.is_empty() {
        disable_buttons(machine, &useless_buttons);
        let result = find_least_presses(machine, previous_result);
        enable_buttons(machine, &useless_buttons);
        return result;
    }

    match best_button(machine) {
        None => previous_result,
        Some(i) => {
            let button = machine.buttons[i].clone();
            let j = min_joltage(machine, &button);
            press_button_n_times(machine, &button, j);
            let mut result = previous_result;
            machine.button_availability[i] = false;
            for _ in 0..j {
                result = find_least_presses(machine, result);
                unpress_button(machine, &button);
            }
            result = find_least_presses(machine, result);
            machine.button_availability[i] = true;
            result
        }
    }
}

fn best_button(machine: &RunningMachine) -> Option<usize> {
    let mut buttons_per_joltage = vec![0; machine.joltage_requirements.len()];
    for (_, button) in available_buttons(machine) {
        for j in button {
            buttons_per_joltage[*j] += 1;
        }
    }
    let joltage_with_least_buttons = buttons_per_joltage
        .iter()
        .enumerate()
        .filter(|(i, _)| machine.joltage_requirements[*i] > 0)
        .min_by_key(|(_, buttons)| **buttons)
        .map(|(i, _)| i)
        .expect("There should still be unresolved joltages");
    available_buttons(machine)
        .iter()
        .find(|(_i, b)| b.contains(&joltage_with_least_buttons))
        .map(|(i, _)| *i)
}

fn available_buttons(machine: &RunningMachine) -> Vec<(usize, &Button)> {
    machine
        .buttons
        .iter()
        .enumerate()
        .filter(|(i, _b)| machine.button_availability[*i])
        .collect()
}

fn min_joltage(machine: &RunningMachine, button: &Button) -> i32 {
    button
        .iter()
        .map(|i| machine.joltage_requirements[*i])
        .min()
        .unwrap()
}

fn disable_buttons(machine: &mut RunningMachine, buttons: &Vec<usize>) {
    for button in buttons {
        machine.button_availability[*button] = false;
    }
}

fn enable_buttons(machine: &mut RunningMachine, buttons: &Vec<usize>) {
    for button in buttons {
        machine.button_availability[*button] = true;
    }
}

fn buttons_available(machine: &RunningMachine) -> bool {
    machine.button_availability.iter().any(|x| *x)
}

fn useless_buttons(machine: &RunningMachine) -> Vec<usize> {
    let filled_requirements = (0..machine.joltage_requirements.len())
        .filter(|i| machine.joltage_requirements[*i] == 0)
        .collect::<Vec<usize>>();
    available_buttons(machine)
        .iter()
        .filter(|(_i, button)| button.iter().any(|j| filled_requirements.contains(&j)))
        .map(|(i, _button)| *i)
        .collect()
}

fn requirements_met(machine: &RunningMachine) -> bool {
    machine.joltage_requirements.iter().all(|j| *j == 0)
}

fn joltage_overpowered(machine: &RunningMachine) -> bool {
    machine.joltage_requirements.iter().any(|j| *j < 0)
}

fn press_button_n_times(machine: &mut RunningMachine, button: &Button, n: i32) {
    machine.buttons_pressed += n;
    for i in button {
        machine.joltage_requirements[*i] -= n;
    }
}

fn unpress_button(machine: &mut RunningMachine, button: &Button) {
    press_button_n_times(machine, button, -1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::get_test_input;

    #[test]
    pub fn part1() {
        let input = get_test_input(2025, 10);
        let machines = handle_input(&input);
        let res = super::part1(&machines);
        assert_eq!(res, 7);
    }

    #[test]
    pub fn test_machine_configuring() {
        let input = [
            ("[#.##] (0,2,3) (1,3) {197,15,197,212}", 212),
            ("[.] (0) {0}", 0),
            ("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10),
            (
                "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
                12,
            ),
            (
                "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
                11,
            ),
        ];
        let machines = input.map(|(machine, expected)| (parse_machine(machine), expected));
        for (machine, expected) in machines {
            let res = configure_machine(&machine);
            assert_eq!(res, Some(expected));
        }
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 10);
        let machines = handle_input(&input);
        let res = super::part2(machines);
        assert_eq!(res, 33);
    }
}
