#[derive(Clone)]
pub struct Grid<T: Clone> {
    elems: Vec<T>,
    elems_width: usize,
    default: T,
}

impl<T: Clone> Grid<T> {
    pub fn new(elems: Vec<T>, elems_width: usize, default: T) -> Self {
        assert!(elems_width > 0);
        assert_eq!(elems.len() % elems_width, 0);
        Grid { elems, elems_width, default }
    }

    pub fn width(&self) -> i64 {
        self.elems_width as i64
    }

    pub fn height(&self) -> i64 {
        (self.elems.len() / self.elems_width) as i64
    }

    pub fn get_xy(&self, p: (i64, i64)) -> &T {
        self.get(p.0, p.1)
    }

    pub fn get(&self, x: i64, y: i64) -> &T {
        if 0 <= x && x < self.width() && 0 <= y && y < self.height() {
            &self.elems[(x + y * self.width()) as usize]
        } else {
            &self.default
        }
    }

    pub fn sink_elems(self) -> Vec<T> {
        self.elems
    }
}
