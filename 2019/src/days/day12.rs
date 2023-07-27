use std::cell::RefCell;
#[derive(Debug, Clone)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
    init_state: [(i32, i32); 3],
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: [x, y, z],
            velocity: [0, 0, 0],
            init_state: [(x, 0), (y, 0), (z, 0)],
        }
    }

    fn energy(&self) -> i32 {
        self.position.iter().map(|x| x.abs()).sum::<i32>()
            * self.velocity.iter().map(|x| x.abs()).sum::<i32>()
    }

    fn repeated(&self, dim: usize) -> bool {
        self.init_state[dim] == (self.position[dim], self.velocity[dim])
    }

    fn update_velocities(&mut self, other: &Moon) {
        for i in 0..3 {
            self.velocity[i] += match other.position[i] {
                x if x > self.position[i] => 1,
                x if x < self.position[i] => -1,
                _ => 0,
            };
        }
    }

    fn apply_velocity(&mut self) {
        for i in 0..3 {
            self.position[i] += self.velocity[i];
        }
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

fn run_sim_step(moons: &Vec<RefCell<Moon>>) {
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

fn total_energy_in_system(moons: Vec<RefCell<Moon>>, steps: u32) -> i32 {
    for _ in 0..steps {
        run_sim_step(&moons);
    }
    let mut total = 0;
    for moon in moons {
        total += moon.borrow().energy();
    }
    total
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut res = b % a;
    while res != 0 {
        b = a;
        a = res;
        res = b % a;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn steps_to_repeat(moons: Vec<RefCell<Moon>>) -> usize {
    let mut periods_axis = vec![0; 3];
    let mut steps = 0;
    while periods_axis.contains(&0) {
        run_sim_step(&moons);
        steps += 1;

        for i in 0..3 {
            if periods_axis[i] != 0 {
                continue;
            }
            let mut all = 0;
            for moon in moons.iter() {
                if moon.borrow().repeated(i) {
                    all += 1;
                }
            }
            if all == moons.len() {
                periods_axis[i] = steps;
            }
        }
    }
    let mut total = 1;
    for p in periods_axis.iter() {
        total = lcm(total, *p);
    }
    total
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

        let input_raw = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
        let moons = parse_moons(&input_raw);
        assert_eq!(steps_to_repeat(moons), 4686774924);
    }
}
