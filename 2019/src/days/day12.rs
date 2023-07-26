use std::cell::RefCell;
use std::collections::HashSet;
#[derive(Debug, Clone)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn pot(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kin(&self) -> i32 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn total(&self) -> i32 {
        self.pot() * self.kin()
    }

    fn state(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.z, self.total())
    }

    fn update_velocities(&mut self, other: &Moon) {
        self.vx += match other.x {
            x if x > self.x => 1,
            x if x < self.x => -1,
            _ => 0,
        };
        self.vy += match other.y {
            y if y > self.y => 1,
            y if y < self.y => -1,
            _ => 0,
        };
        self.vz += match other.z {
            z if z > self.z => 1,
            z if z < self.z => -1,
            _ => 0,
        };
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }
}

fn parse_moons(input: &str) -> Vec<RefCell<Moon>> {
    let mut moons = vec![];
    for line in input.lines() {
        let coords = line
            .replace(&['<', '>', '=', 'x', 'y', 'z'], "")
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        moons.push(RefCell::new(Moon::new(coords[0], coords[1], coords[2])));
    }
    moons
}

fn total_energy_in_system(moons: Vec<RefCell<Moon>>, steps: u32) -> i32 {
    for _ in 0..steps {
        for i in 0..moons.len() {
            let mut moon = moons[i].borrow_mut();
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }
                let other = moons[j].borrow();
                moon.update_velocities(&other);
            }
        }
        for moon in moons.iter() {
            moon.borrow_mut().apply_velocity();
        }
    }

    let mut total = 0;
    for moon in moons {
        total += moon.borrow().total();
    }
    total
}

fn steps_to_repeat(moons: Vec<RefCell<Moon>>) -> i64 {
    let mut seen = HashSet::new();
    let mut s = 0;

    loop {
        for i in 0..moons.len() {
            let mut moon = moons[i].borrow_mut();
            for j in 0..moons.len() {
                if i == j {
                    continue;
                }
                let other = moons[j].borrow();
                moon.update_velocities(&other);
            }
        }

        let mut state = vec![];
        for i in 0..moons.len() {
            state.push(moons[i].borrow().state());
            moons[i].borrow_mut().apply_velocity();
        }
        if seen.contains(&state) {
            return s;
        }
        seen.insert(state);
        s += 1;
    }
}

pub fn run() {
    println!("Day 12: The N-Body Problem");
    let input_raw = "<x=-1, y=-4, z=0>\n<x=4, y=7, z=-1>\n<x=-14, y=-10, z=9>\n<x=1, y=2, z=17>";
    let moons = parse_moons(&input_raw);
    let total = total_energy_in_system(moons.clone(), 1000);
    println!("Part One: {}", total);
    let steps = steps_to_repeat(moons);
    println!("Part Two: {}", steps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input_raw = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let moons = parse_moons(&input_raw);
        assert_eq!(total_energy_in_system(moons, 10), 179);

        let input_raw = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        let moons = parse_moons(&input_raw);
        assert_eq!(total_energy_in_system(moons, 100), 1940);
    }

    #[test]
    fn test_part_two() {
        let input_raw = "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>";
        let moons = parse_moons(&input_raw);
        assert_eq!(steps_to_repeat(moons), 2772);

        // let input_raw = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        // let moons = parse_moons(&input_raw);
        // assert_eq!(steps_to_repeat(moons), 4686774924);
    }
}
