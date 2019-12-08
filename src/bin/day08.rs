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
}
