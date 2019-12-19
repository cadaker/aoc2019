#[derive(Clone)]
pub struct Grid<T> {
    elems: Vec<T>,
    elems_width: usize,
    default: T,
}

impl<T> Grid<T> {
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

    pub fn find_first(&self, item: &T) -> Option<(i64, i64)>
        where T: PartialEq
    {
        self.find_first_if(|x| *x == *item)
    }

    pub fn find_all(&self, item: &T) -> Vec<(i64, i64)>
        where T: PartialEq
    {
        self.find_all_if(|x| *x == *item)
    }

    pub fn find_first_if<P>(&self, pred: P) -> Option<(i64, i64)>
        where P: Fn(&T) -> bool
    {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if pred(self.get(x, y)) {
                    return Some((x,y))
                }
            }
        }
        None
    }

    pub fn find_all_if<P>(&self, pred: P) -> Vec<(i64, i64)>
        where P: Fn(&T) -> bool
    {
        let mut ret = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if pred(self.get(x, y)) {
                    ret.push((x,y))
                }
            }
        }
        ret
    }
}

pub struct GridBuilder<T> {
    elems: Vec<T>,
    width: Option<usize>,
}

impl<T> GridBuilder<T> {
    pub fn new() -> Self {
        GridBuilder { elems: Vec::new(), width: None }
    }

    pub fn push(&mut self, elem: T) {
        self.elems.push(elem)
    }

    pub fn eol(&mut self) {
        if self.width.is_none() {
            self.width = Some(self.elems.len());
        } else {
            assert_eq!(self.elems.len() % self.width.unwrap(), 0);
        }
    }

    pub fn build(self, default: T) -> Grid<T> {
        Grid::new(self.elems, self.width.unwrap(), default)
    }
}
