use std::collections::{HashMap, VecDeque, HashSet};
use intcode::*;

#[aoc_generator(day23)]
pub fn generator(input: &str) -> IntcodeProgram {
    intcode_parser(input)
}

#[derive(Clone)]
struct Packet {
    x: isize,
    y: isize
}

#[aoc(day23, part1)]
pub fn part1(program: &IntcodeProgram) -> isize {

    let mut machines = Vec::new();
    for i in 0..50 {
        let mut machine = Machine::new(program);
        machine.push_input(i);
        machines.push(machine);
    }

    let mut receive_queues: HashMap<usize, VecDeque<Packet>> = HashMap::new();
    let mut send_queue: HashMap<usize, VecDeque<isize>> = HashMap::new();

    'main: loop {
        for (i, machine) in machines.iter_mut().enumerate(){

            match machine.step() {
                Some(Action::Halt) => unimplemented!(),
                Some(Action::Output(value)) => {
                    let queue = send_queue.entry(i).or_default();
                    queue.push_back(value);

                    if queue.len() >= 3 {
                        let destination = queue.pop_front().unwrap() as usize;
                        let x = queue.pop_front().unwrap();
                        let y = queue.pop_front().unwrap();

                        let packet = Packet {
                            x,
                            y
                        };

                        if destination == 255 {
                            return y;
                        }

                        receive_queues.entry(destination).or_default().push_back(packet);
                    }
                },
                Some(Action::RequiresInput) => {
                    let queue = receive_queues.entry(i).or_default();
                    if let Some(packet) = queue.pop_front() {
                        machine.push_input(packet.x);
                        machine.push_input(packet.y);
                    } else {
                        machine.push_input(-1);
                    }
                },
                None => {}
            }
        }
    }
}

#[aoc(day23, part2)]
pub fn part2(program: &IntcodeProgram) -> isize {

    let mut machines = Vec::new();
    for i in 0..50 {
        let mut machine = Machine::new(program);
        machine.push_input(i);
        machines.push(machine);
    }

    let mut receive_queues: HashMap<usize, VecDeque<Packet>> = HashMap::new();
    let mut send_queue: HashMap<usize, VecDeque<isize>> = HashMap::new();
    let mut nat_packet: Option<Packet> = None;
    let mut last_nat_packet: Option<Packet> = None;
    let mut is_blocked: HashSet<usize> = HashSet::new();

    'main: loop {
        if is_blocked.len() == 50 && receive_queues.iter().all(|(_, queue)| queue.is_empty()) && nat_packet.is_some(){
            let packet = nat_packet.unwrap();
            if let Some(last_nat_packet) = last_nat_packet {
                if packet.y == last_nat_packet.y {
                    return packet.y;
                }
            }

            receive_queues.entry(0).or_default().push_back(packet.clone());
            is_blocked.remove(&0);
            nat_packet = None;
            last_nat_packet = Some(packet);
        }

        for (i, machine) in machines.iter_mut().enumerate(){
            match machine.step() {
                Some(Action::Halt) => unimplemented!(),
                Some(Action::Output(value)) => {
                    let queue = send_queue.entry(i).or_default();
                    queue.push_back(value);

                    if queue.len() >= 3 {
                        let destination = queue.pop_front().unwrap() as usize;
                        let x = queue.pop_front().unwrap();
                        let y = queue.pop_front().unwrap();

                        let packet = Packet {
                            x,
                            y
                        };

                        if destination == 255 {
                            nat_packet = Some(packet);
                        } else {
                            receive_queues.entry(destination).or_default().push_back(packet);
                            is_blocked.remove(&destination);
                        }
                    }
                },
                Some(Action::RequiresInput) => {
                    let queue = receive_queues.entry(i).or_default();
                    if let Some(packet) = queue.pop_front() {
                        machine.push_input(packet.x);
                        machine.push_input(packet.y);
                    } else {
                        is_blocked.insert(i);
                        machine.push_input(-1);
                    }
                },
                None => {}
            }
        }
    }
}