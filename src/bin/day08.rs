use aoc2019::io::slurp_stdin;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

type Layer = Vec<char>;

struct Image {
    layers: Vec<Layer>,
}

fn parse_image() -> Image {
    let mut image = Image { layers: Vec::new() };
    let mut layer = Layer::new();
    for pixel in slurp_stdin().trim().chars() {
        layer.push(pixel);
        if layer.len() == WIDTH*HEIGHT {
            image.layers.push(layer);
            layer = Layer::new();
        }
    }
    assert!(layer.is_empty());
    image
}

fn count<T: PartialEq>(haystack: &[T], needle: &T) -> usize {
    haystack
        .iter()
        .filter(|&x| x == needle)
        .count()
}

fn flatten_layers(image: &Image) -> Layer {
    let mut combined = Layer::new();
    let join = |pos| {
        for layer in &image.layers {
            if layer[pos] == '0' {
                return '0';
            } else if layer[pos] == '1' {
                return '1';
            }
        }
        '2'
    };
    for i in 0..image.layers.first().unwrap().len() {
        combined.push(join(i));
    }
    assert_eq!(combined.len(), image.layers.first().unwrap().len());
    combined
}

fn main() {
    let image = parse_image();
    let fewest_zeroes_layer = image
        .layers
        .iter()
        .min_by_key(|&layer| count(&layer, &'0'))
        .unwrap();
    let ones = count(&fewest_zeroes_layer, &'1');
    let twos = count(&fewest_zeroes_layer, &'2');
    println!("{}", ones * twos);

    let flat = flatten_layers(&image);
    for r in 0..HEIGHT {
        for c in 0..WIDTH {
            if flat[r*WIDTH + c] == '0' {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_flatten() {
        let img = Image { layers: vec![
            vec!['0', '2', '2', '2'],
            vec!['1', '1', '2', '2'],
            vec!['2', '2', '1', '2'],
            vec!['0', '0', '0', '0']] };
        let flat = flatten_layers(&img);
        assert_eq!(flat, vec!['0', '1', '1', '0']);
    }
}
