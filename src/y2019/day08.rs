use std::fs;

pub(crate) fn day08() {
    let input = fs::read_to_string("input/2019/day08/input.txt").unwrap();
    let image: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let layers: Vec<&[u32]> = image.chunks(25 * 6).collect();

    part_a(&layers);
    part_b(&layers);
}

fn part_a(layers: &Vec<&[u32]>) {
    let min_layer = layers
        .iter()
        .min_by(|layer1, layer2| count_zeroes(&layer1).cmp(&count_zeroes(&layer2)))
        .unwrap();

    println!("{}", checksum(min_layer));
}

fn count_zeroes(layer: &[u32]) -> usize {
    count_numbers(layer, 0)
}

fn count_numbers(layer: &[u32], number: u32) -> usize {
    layer.iter().filter(|n| **n == number).count()
}

fn checksum(layer: &[u32]) -> usize {
    count_numbers(layer, 1) * count_numbers(layer, 2)
}

fn part_b(layers: &Vec<&[u32]>) {
    let mut image = vec![0; layers[0].len()];
    for i in 0..(25 * 6) {
        for j in (0..layers.len()).rev() {
            if layers[j][i] == 2 {
                continue;
            }
            image[i] = layers[j][i];
        }
    }
    print_image(&image);
}

fn print_image(image: &Vec<u32>) {
    for i in 0..6 {
        for j in 0..25 {
            let pixel_color = image[(i * 25) + j];
            print!("{}", if pixel_color > 0 { 'X' } else { ' ' });
        }
        println!();
    }
}
