mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

#[derive(Debug)]
enum Command {
    N,
    S,
    E,
    W,
    R,
    L,
    F
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {

    fn add_x(&mut self, x: i32) {
        self.x += x;
    }

    fn add_y(&mut self, y: i32) {
        self.y += y;
    }

    fn go_in_direction(&mut self, direction: i32, mag: i32) {
        match direction {
            0 => self.x += mag,
            90 => self.y += mag,
            180 => self.x -= mag,
            270 => self.y -= mag,
            _ => {}
        }
    }

    fn rotate(&mut self, mut by: i32) {
        by = modulo(by, 360);
        match by {
            90 => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            },
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            },
            270 => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            },
            _ => {}
        }
    }

    fn move_to_waypoint(&mut self, waypoint: &Point, mag: i32) {
        self.x += waypoint.x * mag;
        self.y += waypoint.y * mag;
    }

    fn manhatten_distance_to_origin(&self) -> i32 {
        return i32::abs(self.x) + i32::abs(self.y);
    }
}

fn modulo(num: i32, modulos: i32) -> i32 {
    let modulo = num % modulos;
    if modulo < 0 {
        return modulo + modulos;
    }
    return modulo;
}

fn command_from_string(string: &str) -> Command {
    return match string {
        "N" => Command::N,
        "S" => Command::S,
        "E" => Command::E,
        "W" => Command::W,
        "R" => Command::R,
        "L" => Command::L,
        "F" => Command::F,
        _ => Command::N
    }
}

fn read_input_file(file_name: &str) -> Vec<(Command, i32)> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut commands: Vec<(Command, i32)> = Vec::new();

    let mut command: Command;
    let mut mag: i32;

    for line in f.lines() {
        let line = line.unwrap();
        command = command_from_string(&line[0..1]);
        mag = line[1..].parse().unwrap();
        commands.push((command, mag))
    }
    return commands;
}

fn solution_part_1(file_name: &str) -> i32 {
    let commands: Vec<(Command, i32)> = read_input_file(file_name);
    let mut current_position: Point = Point {x: 0, y: 0};
    let mut current_direction: i32 = 0;
    for (command, mag) in commands {
        match command {
            Command::N => current_position.add_y(mag),
            Command::S => current_position.add_y(-mag),
            Command::E => current_position.add_x(mag),
            Command::W => current_position.add_x(-mag),
            Command::R => current_direction = modulo(current_direction - mag, 360),
            Command::L => current_direction = modulo(current_direction + mag, 360),
            Command::F => current_position.go_in_direction(current_direction, mag)
        }
        debug!("Command: {:?}, Mag: {:?}, Position: {:?}, Direction {:?}",
               command, mag, current_position, current_direction);
    }
    return current_position.manhatten_distance_to_origin();
}

fn solution_part_2(file_name: &str) -> i32 {
    let commands: Vec<(Command, i32)> = read_input_file(file_name);
    let mut current_position: Point = Point {x: 0, y: 0};
    let mut waypoint: Point = Point {x: 10, y: 1};
    for (command, mag) in commands {
        match command {
            Command::N => waypoint.add_y(mag),
            Command::S => waypoint.add_y(-mag),
            Command::E => waypoint.add_x(mag),
            Command::W => waypoint.add_x(-mag),
            Command::R => waypoint.rotate(-mag),
            Command::L => waypoint.rotate(mag),
            Command::F => current_position.move_to_waypoint(&waypoint, mag)
        }
        debug!("Command: {:?}, Mag: {:?}, Position: {:?}, Waypoint {:?}",
               command, mag, current_position, waypoint);
    }
    return current_position.manhatten_distance_to_origin();
}

fn main() {
    env_logger::init();
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
