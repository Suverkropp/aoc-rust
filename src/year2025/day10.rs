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
    let light_diagram = parse_light_diagram(words[0]);
    let button_wiring_schematics = words[1..words.len() - 1]
        .iter()
        .map(|x| parse_button_wiring_schematic(*x))
        .collect();
    let joltage_requirements = parse_joltage_requirements(words.last().unwrap());
    Machine {
        light_diagram,
        button_wiring_schematics,
        joltage_requirements,
    }
}

fn parse_light_diagram(input: &str) -> Vec<bool> {
    parse_with_delimiters(
        '[',
        ']',
        input,
        |lights| {
            lights
                .chars()
                .map(|l| match l {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Light diagram cannot contain {l}"),
                })
                .collect::<Vec<_>>()
        },
    )
}

fn parse_button_wiring_schematic(input: &str) -> Button {
    parse_with_delimiters(
        '(',
        ')',
        input,
        |content| content.split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

fn parse_joltage_requirements(input: &str) -> Vec<u32> {
    parse_with_delimiters(
        '{',
        '}',
        input,
        |content| content.split(',').map(|s| s.parse().unwrap()).collect(),
    )
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
    press_buttons(&mut lights, buttons);
    lights.eq(light_diagram)
}

fn press_buttons(lights: &mut Vec<bool>, buttons: Vec<&Button>) {
    for button in buttons {
        press_button(lights, button);
    }
}

fn press_button(lights: &mut Vec<bool>, button: &Button) {
    for light in button {
        lights[*light] = !lights[*light];
    }
}

pub fn part2(machines: &Vec<Machine>) -> u32 {
    machines.iter().map(configure_machine).sum()
}

fn configure_machine(machine: &Machine) -> u32 {
    todo!()
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
