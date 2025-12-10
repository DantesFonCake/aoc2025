use std::fmt::{Debug, Formatter};
use std::io::{BufRead, BufReader, Read, Result};
use std::ops::RangeInclusive;
use std::ptr::addr_eq;
use util::TaskInput;

fn main() -> Result<()> {
    util::run::<Task>("day9/src/input.txt")
}
struct Task;

impl util::Task for Task {
    type Input = Grid;
    type Output = usize;

    fn solve_1(input: Self::Input) -> Self::Output {
        let mut max = 0usize;

        for firsti in 0..input.0.len() {
            for secondi in (firsti + 1)..input.0.len() {
                let first = input.0[firsti];
                let second = input.0[secondi];

                let rect = Rectangle::from_diag(first, second);
                max = max.max(rect.area());
            }
        }
        max
    }

    fn solve_2(input: Self::Input) -> Self::Output {
        let mut max = 0usize;

        let polygon = Polygon::new(input.0);

        for firsti in 0..polygon.points.len() {
            for secondi in (firsti + 1)..polygon.points.len() {
                let first = polygon.points[firsti];
                let second = polygon.points[secondi];

                let rectangle = Rectangle::from_diag(first, second);

                if !is_valid(&rectangle, &polygon) {
                    continue;
                }

                max = max.max(rectangle.area());
            }
        }

        max
    }
}

fn is_valid(rectangle: &Rectangle, polygon: &Polygon) -> bool {
    for point in rectangle.points() {
        if !polygon.contains(point) {
            return false;
        }
    }

    for edge in polygon.edges.iter() {
        if edge.intersects(rectangle) {
            return false;
        }
    }

    true
}

#[derive(Copy, Clone)]
struct Line((usize, usize), (usize, usize));

impl Line {
    fn contains(&self, (x, y): (usize, usize)) -> bool {
        if self.is_vertical() && self.0.0 == x {
            if self.y_range().contains(&y) {
                return true;
            }
        } else if self.0.1 == y {
            if self.x_range().contains(&x) {
                return true;
            }
        }

        false
    }

    fn intersects(&self, rectangle: &Rectangle) -> bool {
        if self.is_vertical() {
            let x = self.0.0;
            let y_range = self.y_range();
            if rectangle.left < x && x < rectangle.right {
                if *y_range.start() < rectangle.bottom && *y_range.end() > rectangle.top {
                    return true;
                }
            }
        } else {
            let y = self.0.1;
            let x_range = self.x_range();
            if rectangle.top < y && y < rectangle.bottom {
                if *x_range.start() < rectangle.right && *x_range.end() > rectangle.left {
                    return true;
                }
            }
        }

        false
    }

    fn y_range(&self) -> RangeInclusive<usize> {
        let min = self.0.1.min(self.1.1);
        let max = self.0.1.max(self.1.1);
        min..=max
    }

    fn x_range(&self) -> RangeInclusive<usize> {
        let min = self.0.0.min(self.1.0);
        let max = self.0.0.max(self.1.0);
        min..=max
    }

    fn is_vertical(&self) -> bool {
        self.0.0 == self.1.0
    }
}

impl Debug for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.0, self.1)
    }
}

struct Rectangle {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Rectangle {
    fn from_diag(first: (usize, usize), second: (usize, usize)) -> Self {
        let top = first.1.min(second.1);
        let bottom = first.1.max(second.1);
        let left = first.0.min(second.0);
        let right = first.0.max(second.0);

        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn area(&self) -> usize {
        (self.bottom - self.top + 1) * (self.right - self.left + 1)
    }

    fn points(&self) -> [(usize, usize); 4] {
        [
            (self.left, self.top),
            (self.right, self.top),
            (self.right, self.bottom),
            (self.left, self.bottom),
        ]
    }
}

impl Debug for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle({}, {}, {}, {})",
            self.left, self.top, self.right, self.bottom
        )
    }
}

struct Polygon {
    points: Vec<(usize, usize)>,
    edges: Vec<Line>,
}

impl Polygon {
    fn new(points: Vec<(usize, usize)>) -> Self {
        let edges = points
            .iter()
            .copied()
            .zip(points.iter().skip(1).copied())
            .map(|(start, end)| Line(start, end))
            .chain([Line(points[points.len() - 1], points[0])])
            .collect();

        Self { points, edges }
    }

    fn contains(&self, (x, y): (usize, usize)) -> bool {
        let mut crossings = 0;

        for edge in self.edges.iter() {
            if edge.contains((x, y)) {
                return true;
            }

            if edge.is_vertical() {
                let edge_x = edge.0.0;
                let y_range = edge.y_range();

                if edge_x < x && *y_range.start() < y && y <= *y_range.end() {
                    crossings += 1;
                }
            }
        }

        crossings % 2 == 1
    }
}

struct Grid(Vec<(usize, usize)>);

impl TaskInput for Grid {
    fn read(input: impl Read) -> Result<Self> {
        let reader = BufReader::new(input);
        let mut res = vec![];
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let mut coords = line.split(',');
            let first = coords.next().unwrap().parse().unwrap();
            let second = coords.next().unwrap().parse().unwrap();

            res.push((first, second));
        }
        Ok(Self(res))
    }
}
