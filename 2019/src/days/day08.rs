fn parse_layers(str: &str, width: usize, height: usize) -> Vec<Vec<Vec<u32>>> {
    let digits: Vec<u32> = str
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let size = width * height;
    let total = digits.len() / size;
    let mut layers = vec![];
    for n in 0..total {
        let mut layer = vec![];
        for y in 0..height {
            let row = n * size + y * width;
            layer.push(digits[row..row + width].to_vec());
        }
        layers.push(layer);
    }
    layers
}

fn fewest_zero_layer_num(layers: &Vec<Vec<Vec<u32>>>) -> usize {
    let mut fewest = usize::MAX;
    let mut num = 0;

    for layer in layers {
        let mut zeros = 0;
        for row in layer {
            zeros += row.iter().filter(|&&x| x == 0).count();
        }
        if zeros < fewest {
            fewest = zeros;
            let mut ones = 0;
            let mut twos = 0;
            for row in layer {
                ones += row.iter().filter(|&&x| x == 1).count();
                twos += row.iter().filter(|&&x| x == 2).count();
            }
            num = ones * twos;
        }
    }
    num
}

fn decode_layers(layers: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<u32>> {
    let height = layers[0].len();
    let width = layers[0][0].len();
    let mut decoded = vec![vec![0; width]; height];
    for y in 0..height {
        for x in 0..width {
            for layer in layers {
                match layer[y][x] {
                    2 => continue,
                    n => {
                        decoded[y][x] = n;
                        break;
                    }
                }
            }
        }
    }
    decoded
}

pub fn run() {
    println!("Day 8: Space Image Format");
    let input_raw = crate::load_input(module_path!());

    let layers = parse_layers(&input_raw, 25, 6);
    println!("Part One: {}", fewest_zero_layer_num(&layers));

    let decoded = decode_layers(&layers);
    println!("Part Two:");
    for row in decoded {
        println!(
            "{}",
            row.iter()
                .map(|&x| if x == 1 { "#" } else { " " })
                .collect::<Vec<_>>()
                .join("")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let layers = parse_layers("123456789012", 3, 2);
        assert_eq!(
            layers,
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]]
            ]
        );
        assert_eq!(fewest_zero_layer_num(&layers), 1);
    }

    #[test]
    fn test_part_two() {
        let layers = parse_layers("0222112222120000", 2, 2);
        assert_eq!(layers.len(), 4);
        assert_eq!(decode_layers(&layers), vec![vec![0, 1], vec![1, 0]]);
    }
}
