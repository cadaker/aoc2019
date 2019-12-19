#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub struct CoordinateSystem {
    dx_east: i64,
    dy_south: i64,
}


pub fn step(dir: Dir, coords: CoordinateSystem) -> (i64, i64) {
    match dir {
        Dir::North => (0, -coords.dy_south),
        Dir::East => (coords.dx_east, 0),
        Dir::South => (0, coords.dy_south),
        Dir::West => (-coords.dx_east, 0),
    }
}

pub trait Directional {
    fn as_dir(&self) -> Dir;
    fn from_dir(dir: Dir) -> Self;
    fn step(&self) -> (i64, i64) {
        step(self.as_dir(), Self::coord_system())
    }
    fn coord_system() -> CoordinateSystem;
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub enum Turn {
    Left,
    Right,
}

pub fn turn<D: Directional>(dir: D, turn: Turn) -> D {
    match (dir.as_dir(), turn) {
        (Dir::North, Turn::Left) => D::from_dir(Dir::West),
        (Dir::North, Turn::Right) => D::from_dir(Dir::East),
        (Dir::East, Turn::Left) => D::from_dir(Dir::North),
        (Dir::East, Turn::Right) => D::from_dir(Dir::South),
        (Dir::South, Turn::Left) => D::from_dir(Dir::East),
        (Dir::South, Turn::Right) => D::from_dir(Dir::West),
        (Dir::West, Turn::Left) => D::from_dir(Dir::South),
        (Dir::West, Turn::Right) => D::from_dir(Dir::North),
    }
}

pub const CART_COORDS: CoordinateSystem = CoordinateSystem { dx_east: 1, dy_south: -1};
pub const SCREEN_COORDS: CoordinateSystem = CoordinateSystem { dx_east: 1, dy_south: 1};

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub enum CartesianDir {
    North,
    East,
    South,
    West,
}

impl Directional for CartesianDir {
    fn as_dir(&self) -> Dir {
        match self {
            CartesianDir::North => Dir::North,
            CartesianDir::East => Dir::East,
            CartesianDir::South => Dir::South,
            CartesianDir::West => Dir::West,
        }
    }

    fn from_dir(dir: Dir) -> Self {
        match dir {
            Dir::North => CartesianDir::North,
            Dir::East => CartesianDir::East,
            Dir::South => CartesianDir::South,
            Dir::West => CartesianDir::West,
        }
    }

    fn coord_system() -> CoordinateSystem {
        CART_COORDS
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq, Copy, Clone, Debug)]
pub enum ScreenDir {
    North,
    East,
    South,
    West,
}

impl Directional for ScreenDir {
    fn as_dir(&self) -> Dir {
        match self {
            ScreenDir::North => Dir::North,
            ScreenDir::East => Dir::East,
            ScreenDir::South => Dir::South,
            ScreenDir::West => Dir::West,
        }
    }

    fn from_dir(dir: Dir) -> Self {
        match dir {
            Dir::North => ScreenDir::North,
            Dir::East => ScreenDir::East,
            Dir::South => ScreenDir::South,
            Dir::West => ScreenDir::West,
        }
    }

    fn coord_system() -> CoordinateSystem {
        SCREEN_COORDS
    }
}
