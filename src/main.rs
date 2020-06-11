use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style, cursor};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::color::Color;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point (i8, i8);
enum Type {
    I,
    L,
    J,
    O,
    S,
    T,
    Z
}

#[derive(Debug)]
struct Tetromino {
    points: HashSet<Point>,
    origin: Point,
    color: Box<dyn Color>
}

impl Tetromino {
    fn new(t: Type) -> Self {
        match t {
            Type::I => Tetromino {
                points: [Point(0, 1), Point(1, 1), Point(2, 1), Point(3, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Cyan)
            },
            Type::J => Tetromino {
                points: [Point(0, 0), Point(0, 1), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Blue)
            },
            Type::L => Tetromino {
                points: [Point(0, 1), Point(1, 1), Point(2, 1), Point(2, 0)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Yellow)
            },
            Type::O => Tetromino {
                points: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::LightYellow)
            },
            Type::S => Tetromino {
                points: [Point(0, 1), Point(1, 0), Point(1, 1), Point(2, 0)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Green)
            },
            Type::T => Tetromino {
                points: [Point(1, 0), Point(0, 1), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(3, 0),
                color: Box::new(color::Magenta)
            },
            Type::Z => Tetromino {
                points: [Point(0, 0), Point(1, 0), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Red)
            }
        }
    }
}

const WELL_SIZE_X: i8 = 10;
const WELL_SIZE_Y: i8 = 20;

#[derive(Debug)]
struct Well {
    points: HashSet<Point>
}

impl Well {
    fn new() -> Self {
        let mut well= Well {points: HashSet::new()};
        for y in 0..WELL_SIZE_Y {
            for x in 0..WELL_SIZE_X {
                if x == 0 || x == WELL_SIZE_X-1 || y == WELL_SIZE_Y-1 {
                    well.points.insert(Point(x, y));
                }
            }
        }
        well
    }
}

fn render(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, well: &Well, tet: &Tetromino) {
    let tet_set: HashSet<_> = tet.points.clone().into_iter().
        map(|point| Point(point.0 + tet.origin.0, point.1 + tet.origin.1)).
        collect();

    write!(stdout, "{}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::CurrentLine).unwrap();

    for y in 0..WELL_SIZE_Y {
        for x in 0..WELL_SIZE_X {
            let p = Point(x, y);
            write!(stdout, "{}",    if well.points.contains(&p) {
                                        format!("{}XX{}", color::Fg(color::White), style::Reset)
                                    } else if tet_set.contains(&p) {
                                        format!("{}XX{}", color::Fg(tet.color.deref()), style::Reset)
                                    } else {
                                        format!("  ")
                                    }).unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }
}

fn main() {
    let (tx0, rx) = mpsc::channel();
    let tx1 = tx0.clone();

    thread::spawn(move || loop {
        for c in stdin().keys() {
            tx0.send(c.unwrap()).unwrap();
        }
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        tx1.send(Key::Down).unwrap();
    });


    let mut tet = Tetromino::new(Type::S);
    let mut well = Well::new();

    let mut stdout = stdout().into_raw_mode().unwrap(); // move out

    loop {
        match rx.recv().unwrap() {
            Key::Up => tet.origin.1 -= 1,
            Key::Down => tet.origin.1 += 1,
            Key::Left => tet.origin.0 -= 1,
            Key::Right => tet.origin.0 += 1,
            Key::Ctrl('q') => break,
            _ => (),
        }

        render(&mut stdout, &well, &tet);
        stdout.flush().unwrap();
    }

}