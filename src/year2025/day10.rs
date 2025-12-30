use crate::parsers::parse_with_delimiters;
use itertools::Itertools;

#[derive(Debug)]
pub struct Machine {
    light_diagram: Vec<bool>,
    button_wiring_schematics: Vec<Button>,
    joltage_requirements: Vec<u32>,
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

fn parse_light_diagram(input: &str) -> Vec<bool> {
    parse_with_delimiters('[', ']', input, |lights| {
        lights
            .chars()
            .map(|l| match l {
                '.' => false,
                '#' => true,
                _ => panic!("Light diagram cannot contain {l}"),
            })
            .collect::<Vec<_>>()
    })
}

fn parse_button_wiring_schematic(input: &str) -> Button {
    parse_with_delimiters('(', ')', input, |content| {
        content.split(',').map(|s| s.parse().unwrap()).collect()
    })
}

fn parse_joltage_requirements(input: &str) -> Vec<u32> {
    parse_with_delimiters('{', '}', input, |content| {
        content.split(',').map(|s| s.parse().unwrap()).collect()
    })
}

pub fn part1(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(start_machine).sum()
}

fn start_machine(machine: &Machine) -> u32 {
    for presses in 0..machine.light_diagram.len() {
        if machine
            .button_wiring_schematics
            .iter()
            .combinations(presses)
            .any(|buttons| buttons_turn_on_machine(buttons, &machine.light_diagram))
        {
            return presses as u32;
        }
    }
    panic!("No solution found for machine {machine:?}");
}

fn buttons_turn_on_machine(buttons: Vec<&Button>, light_diagram: &Vec<bool>) -> bool {
    let mut lights = vec![false; light_diagram.len()];
    press_buttons_for_lights(&mut lights, buttons);
    &lights == light_diagram
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

pub fn part2(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(configure_machine).sum()
}

struct RunningMachine<'a> {
    current_joltages: Vec<u32>,
    buttons: Vec<Button>,
    joltage_requirements: &'a Vec<u32>,
    buttons_pressed: u32,
}

impl RunningMachine<'_> {
    fn from_machine(machine: &'_ Machine) -> RunningMachine<'_> {
        let lights = machine.joltage_requirements.len();
        RunningMachine {
            current_joltages: vec![0; lights],
            buttons: machine.button_wiring_schematics.clone(),
            joltage_requirements: &machine.joltage_requirements,
            buttons_pressed: 0,
        }
    }
}

fn configure_machine(machine: &Machine) -> u32 {
    let mut running_machine = RunningMachine::from_machine(machine);
    let result = find_least_presses(&mut running_machine, None);
    println!("Result for machine {machine:?} is {result:?}.");
    result.expect(&format!("Machine {machine:?} should have a solution."))
}

fn find_least_presses(machine: &mut RunningMachine, previous_result: Option<u32>) -> Option<u32> {
    // Guaranties:
    // return value is less than or equal to previous_result
    // buttons list is restored to initial position
    if requirements_met(machine) {
        return Some(machine.buttons_pressed);
    }
    if requirements_impossible(machine)
        || previous_result.is_some_and(|x| x <= machine.buttons_pressed)
    {
        return previous_result;
    }

    let required: Option<Button> = required_buttons(&machine);
    if let Some(button) = required {
        press_button(machine, &button);
        let result = find_least_presses(machine, previous_result);
        unpress_button(machine, &button);
        return result;
    }

    // Try pressing the first button
    let button = machine
        .buttons
        .last()
        .expect("Buttons list should not be empty")
        .clone();
    press_button(machine, &button);
    let new_result = find_least_presses(machine, previous_result);
    unpress_button(machine, &button);

    // Try not pressing the first button
    machine.buttons.pop();
    let result = find_least_presses(machine, new_result);
    machine.buttons.push(button);
    result
}

fn required_buttons<'a>(machine: &RunningMachine<'_>) -> Option<Button> {
    let counters = machine.joltage_requirements.len();
    for i in 0..counters {
        if machine.current_joltages[i] >= machine.joltage_requirements[i] {
            continue;
        }

        let mut usable_buttons = machine.buttons.iter().filter(|button| button.contains(&i));
        let first_button = usable_buttons
            .next()
            .expect("requirements should be achievable")
            .clone();
        if usable_buttons.next().is_none() {
            return Some(first_button);
        }
    }
    None
}

fn requirements_impossible(machine: &RunningMachine) -> bool {
    joltages_overpowered(machine) || machine.buttons.is_empty()
}

fn requirements_met(machine: &RunningMachine) -> bool {
    machine.current_joltages == *machine.joltage_requirements
}

fn press_button(machine: &mut RunningMachine, button: &Button) {
    machine.buttons_pressed += 1;
    for i in button {
        machine.current_joltages[*i] += 1;
    }
}

fn unpress_button(machine: &mut RunningMachine, button: &Button) {
    machine.buttons_pressed -= 1;
    for i in button {
        machine.current_joltages[*i] -= 1;
    }
}

fn joltages_overpowered(machine: &RunningMachine) -> bool {
    machine
        .current_joltages
        .iter()
        .zip(machine.joltage_requirements.iter())
        .any(|(j, r)| j > r)
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
    pub fn part2() {
        let input = get_test_input(2025, 10);
        let machines = handle_input(&input);
        let res = super::part2(&machines);
        assert_eq!(res, 33);
    }
}
