use crate::parsers::parse_with_delimiters;
use itertools::Either::{Left, Right};
use itertools::{Either, Itertools};
use std::fmt::Debug;

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

pub fn part2(machines: &Vec<Machine>) -> i32 {
    machines.iter().map(configure_machine).sum()
}

struct RunningMachine {
    buttons: Vec<Button>,
    joltage_requirements: Vec<i32>,
    buttons_pressed: i32,
}

impl RunningMachine {
    fn from_machine(machine: &'_ Machine) -> RunningMachine {
        RunningMachine {
            buttons: machine.button_wiring_schematics.clone(),
            joltage_requirements: machine.joltage_requirements.clone(),
            buttons_pressed: 0,
        }
    }
}

fn configure_machine(machine: &Machine) -> i32 {
    let mut running_machine = RunningMachine::from_machine(machine);
    running_machine.buttons.sort_by_key(Vec::len);
    let result = find_least_presses(&mut running_machine, None);
    println!("Result for machine {machine:?} is {result:?}.");
    result.expect(&format!("Machine {machine:?} should have a solution."))
}

fn find_least_presses(machine: &mut RunningMachine, previous_result: Option<i32>) -> Option<i32> {
    // Guaranties:
    // return value is less than or equal to previous_result
    // buttons list is restored to initial position
    if requirements_met(machine) && previous_result.is_none_or(|x| x > machine.buttons_pressed) {
        return Some(machine.buttons_pressed);
    }
    if joltage_overpowered(machine)
        || machine.buttons.is_empty()
        || previous_result.is_some_and(|x| x <= machine.buttons_pressed)
    {
        return previous_result;
    }

    if let Some((i, button)) = useless_button(&machine) {
        machine.buttons.remove(i);
        let result = find_least_presses(machine, previous_result);
        machine.buttons.insert(i, button);
        return result;
    }

    match required_button(machine) {
        Left((button, j)) => {
            press_button_n_times(machine, &button, j);
            let result = find_least_presses(machine, previous_result);
            press_button_n_times(machine, &button, -j);
            return result;
        }
        Right(impossible) => {
            if impossible {
                return previous_result;
            }
        }
    }

    // Try pressing the last button
    let button = machine
        .buttons
        .last()
        .expect("Buttons list should not be empty")
        .clone();
    press_button(machine, &button);
    let new_result = find_least_presses(machine, previous_result);
    unpress_button(machine, &button);

    // Try not pressing the last button
    machine.buttons.pop();
    let result = find_least_presses(machine, new_result);
    machine.buttons.push(button);
    result
}

fn useless_button(machine: &RunningMachine) -> Option<(usize, Button)> {
    let filled_requirements = machine
        .joltage_requirements
        .iter()
        .enumerate()
        .filter(|(_, req)| **req == 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    for (i, button) in machine.buttons.iter().enumerate() {
        if button.iter().any(|j| filled_requirements.contains(j)) {
            return Some((i, button.clone()));
        }
    }
    None
}

fn required_button(machine: &RunningMachine) -> Either<(Button, i32), bool> {
    let counters = machine.joltage_requirements.len();
    for i in 0..counters {
        let required = machine.joltage_requirements[i];
        if required == 0 {
            continue;
        }

        let mut usable_buttons = machine.buttons.iter().filter(|button| button.contains(&i));
        match usable_buttons.next() {
            Some(button) => {
                if usable_buttons.next().is_none() {
                    return Left((button.clone(), required));
                }
            }
            None => return Right(true),
        }
    }
    Right(false)
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

fn press_button(machine: &mut RunningMachine, button: &Button) {
    press_button_n_times(machine, button, 1);
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
            assert_eq!(res, expected);
        }
    }

    #[test]
    pub fn part2() {
        let input = get_test_input(2025, 10);
        let machines = handle_input(&input);
        let res = super::part2(&machines);
        assert_eq!(res, 33);
    }
}
