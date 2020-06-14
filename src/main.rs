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
enum TetrominoType {
    I,
    L,
    J,
    O,
    S,
    T,
    Z
}
#[derive(Debug)]
enum TetrominoAngle {
    Degree0,
    Degree90,
    Degree180,
    Degree270
}

#[derive(Debug)]
enum TetrisEvent {
    MoveDown,
    MoveRight,
    MoveLeft,
    RotateCW,
    RotateCCW,
    Drop,
    Quit
}

#[derive(Debug)]
struct Tetromino {
    points_0: HashSet<Point>,
    points_90: HashSet<Point>,
    points_180: HashSet<Point>,
    points_270: HashSet<Point>,
    origin: Point,
    color: Box<dyn Color>,
    angle: TetrominoAngle
}

impl Tetromino {
    fn new(t: TetrominoType) -> Self {
        match t {
            TetrominoType::I => Tetromino {
                points_0: [Point(0, 1), Point(1, 1), Point(2, 1), Point(3, 1)].
                    iter().cloned().collect(),
                points_90: [Point(2, 0), Point(2, 1), Point(2, 2), Point(3, 2)].
                    iter().cloned().collect(),
                points_180: [Point(0, 2), Point(1, 2), Point(2, 2), Point(3, 2)].
                    iter().cloned().collect(),
                points_270: [Point(1, 0), Point(1, 1), Point(1, 2), Point(1, 3)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Cyan),
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::J => Tetromino {
                points_0: [Point(0, 0), Point(0, 1), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(2, 0), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                points_180: [Point(0, 1), Point(1, 1), Point(2, 1), Point(2, 2)].
                    iter().cloned().collect(),
                points_270: [Point(0, 2), Point(1, 0), Point(1, 1), Point(1, 2)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Blue),
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::L => Tetromino {
                points_0: [Point(0, 1), Point(1, 1), Point(2, 1), Point(2, 0)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(1, 1), Point(1, 2), Point(2, 2)].
                    iter().cloned().collect(),
                points_180: [Point(0, 1), Point(1, 1), Point(2, 1), Point(0, 2)].
                    iter().cloned().collect(),
                points_270: [Point(0, 0), Point(1, 0), Point(1, 1), Point(1, 2)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Yellow),
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::O => Tetromino {
                points_0: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(2, 1), Point(1, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_180: [Point(1, 0), Point(2, 1), Point(1, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_270: [Point(1, 0), Point(2, 1), Point(1, 0), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::LightYellow),
                angle: TetrominoAngle::Degree0,

            },
            TetrominoType::S => Tetromino {
                points_0: [Point(0, 1), Point(1, 0), Point(1, 1), Point(2, 0)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(1, 1), Point(2, 1), Point(2, 2)].
                    iter().cloned().collect(),
                points_180: [Point(0, 2), Point(1, 1), Point(1, 2), Point(2, 1)].
                    iter().cloned().collect(),
                points_270: [Point(0, 0), Point(0, 1), Point(1, 1), Point(1, 2)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Green),
                angle: TetrominoAngle::Degree0,

            },
            TetrominoType::T => Tetromino {
                points_0: [Point(1, 0), Point(0, 1), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(1, 1), Point(1, 2), Point(2, 1)].
                    iter().cloned().collect(),
                points_180: [Point(0, 1), Point(1, 1), Point(1, 2), Point(2, 1)].
                    iter().cloned().collect(),
                points_270: [Point(0, 1), Point(1, 0), Point(1, 1), Point(1, 2)].
                    iter().cloned().collect(),
                origin: Point(3, 0),
                color: Box::new(color::Magenta),
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::Z => Tetromino {
                points_0: [Point(0, 0), Point(1, 0), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 1), Point(1, 2), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_180: [Point(0, 1), Point(1, 1), Point(1, 2), Point(2, 2)].
                    iter().cloned().collect(),
                points_270: [Point(0, 1), Point(0, 2), Point(1, 0), Point(1, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: Box::new(color::Red),
                angle: TetrominoAngle::Degree0,
            }
        }
    }

    fn get_current_set(&self) -> HashSet<Point> {
        let points = match self.angle {
            TetrominoAngle::Degree0 => self.points_0.clone(),
            TetrominoAngle::Degree90 => self.points_90.clone(),
            TetrominoAngle::Degree180 => self.points_180.clone(),
            TetrominoAngle::Degree270 => self.points_270.clone(),
        };

        points.into_iter().
        map(|point| Point(point.0 + self.origin.0, point.1 + self.origin.1)).
        collect()
    }

    fn rotate_cw(&mut self) {
        self.angle = match self.angle {
            TetrominoAngle::Degree0 => TetrominoAngle::Degree90,
            TetrominoAngle::Degree90 => TetrominoAngle::Degree180,
            TetrominoAngle::Degree180 => TetrominoAngle::Degree270,
            TetrominoAngle::Degree270 => TetrominoAngle::Degree0,
        };
    }

    fn rotate_ccw(&mut self) {
        self.angle = match self.angle {
            TetrominoAngle::Degree0 => TetrominoAngle::Degree270,
            TetrominoAngle::Degree90 => TetrominoAngle::Degree0,
            TetrominoAngle::Degree180 => TetrominoAngle::Degree90,
            TetrominoAngle::Degree270 => TetrominoAngle::Degree180,
        };
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
    write!(stdout, "{}{}",
           termion::cursor::Goto(1, 1),
           termion::clear::CurrentLine).unwrap();

    for y in 0..WELL_SIZE_Y {
        for x in 0..WELL_SIZE_X {
            let p = Point(x, y);
            write!(stdout, "{}",    if well.points.contains(&p) {
                                        format!("{}XX{}", color::Fg(color::White), style::Reset)
                                    } else if tet.get_current_set().contains(&p) {
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
            let event = match c.unwrap() {
                Key::Up         => Some(TetrisEvent::RotateCCW),
                Key::Down       => Some(TetrisEvent::RotateCW),
                Key::Left       => Some(TetrisEvent::MoveLeft),
                Key::Right      => Some(TetrisEvent::MoveRight),
                Key::Ctrl('q')  => Some(TetrisEvent::Quit),
                _ => None,
            };
            if let Some(event) = event {
                tx0.send(event).unwrap();
            }
        }
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(500));
        tx1.send(TetrisEvent::MoveDown).unwrap();
    });


    let mut tet = Tetromino::new(TetrominoType::S);
    let mut well = Well::new();

    let mut stdout = stdout().into_raw_mode().unwrap(); // move out

    loop {
        match rx.recv().unwrap() {
            TetrisEvent::RotateCCW  => tet.rotate_ccw(),
            TetrisEvent::RotateCW   => tet.rotate_cw(),
            TetrisEvent::MoveLeft   => tet.origin.0 -= 1,
            TetrisEvent::MoveRight  => tet.origin.0 += 1,
            TetrisEvent::MoveDown   => tet.origin.1 += 1,
            TetrisEvent::Quit       => break,
            _ => (), // implement drop
        }

        render(&mut stdout, &well, &tet);
        stdout.flush().unwrap();
    }

}