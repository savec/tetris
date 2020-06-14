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
use rand::Rng;

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

#[derive(Debug, Clone, Copy)]
enum TetrominoAngle {
    Degree0,
    Degree90,
    Degree180,
    Degree270
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TetrominoRotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, Clone)]
enum TetrominoColor {
    Cyan,
    Blue,
    Yellow,
    LightYellow,
    Green,
    Magenta,
    Red
}

#[derive(Debug)]
enum TetrisEvent {
    MoveDown,
    MoveRight,
    MoveLeft,
    RotateCW,
    RotateCCW,
    Drop,
    Quit,
    Skip
}

#[derive(Debug, Clone)]
struct Tetromino {
    points_0: HashSet<Point>,
    points_90: HashSet<Point>,
    points_180: HashSet<Point>,
    points_270: HashSet<Point>,
    origin: Point,
    color: TetrominoColor,
    angle: TetrominoAngle
}

impl Tetromino {
    fn random() -> Self {
        match rand::thread_rng().gen_range(1, 8) {
            1 => Tetromino::new(TetrominoType::I),
            2 => Tetromino::new(TetrominoType::L),
            3 => Tetromino::new(TetrominoType::J),
            4 => Tetromino::new(TetrominoType::O),
            5 => Tetromino::new(TetrominoType::S),
            6 => Tetromino::new(TetrominoType::T),
            7 => Tetromino::new(TetrominoType::Z),
            _ => panic!()
        }
    }

    fn new(t: TetrominoType) -> Self {
        match t {
            TetrominoType::I => Tetromino {
                points_0: [Point(0, 1), Point(1, 1), Point(2, 1), Point(3, 1)].
                    iter().cloned().collect(),
                points_90: [Point(2, 0), Point(2, 1), Point(2, 2), Point(2, 3)].
                    iter().cloned().collect(),
                points_180: [Point(0, 2), Point(1, 2), Point(2, 2), Point(3, 2)].
                    iter().cloned().collect(),
                points_270: [Point(1, 0), Point(1, 1), Point(1, 2), Point(1, 3)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: TetrominoColor::Cyan,
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::J => Tetromino {
                points_0: [Point(0, 0), Point(0, 1), Point(1, 1), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(1, 1), Point(1, 2), Point(2, 0)].
                    iter().cloned().collect(),
                points_180: [Point(0, 1), Point(1, 1), Point(2, 1), Point(2, 2)].
                    iter().cloned().collect(),
                points_270: [Point(0, 2), Point(1, 0), Point(1, 1), Point(1, 2)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: TetrominoColor::Blue,
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
                color: TetrominoColor::Yellow,
                angle: TetrominoAngle::Degree0,
            },
            TetrominoType::O => Tetromino {
                points_0: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_90: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_180: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                points_270: [Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)].
                    iter().cloned().collect(),
                origin: Point(2, 0),
                color: TetrominoColor::LightYellow,
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
                color: TetrominoColor::Green,
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
                color: TetrominoColor::Magenta,
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
                color: TetrominoColor::Red,
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

    fn get_angle_after_rotation(&self, r: TetrominoRotation) -> TetrominoAngle {
        match self.angle {
            TetrominoAngle::Degree0 => if r == TetrominoRotation::Clockwise {
                TetrominoAngle::Degree90
            } else {
                TetrominoAngle::Degree270
            },
            TetrominoAngle::Degree90 => if r == TetrominoRotation::Clockwise {
                TetrominoAngle::Degree180
            } else {
                TetrominoAngle::Degree0
            },
            TetrominoAngle::Degree180 => if r == TetrominoRotation::Clockwise {
                TetrominoAngle::Degree270
            } else {
                TetrominoAngle::Degree90
            },
            TetrominoAngle::Degree270 => if r == TetrominoRotation::Clockwise {
                TetrominoAngle::Degree0
            } else {
                TetrominoAngle::Degree180
            }
        }
    }

    fn get_color(&self) -> Box<dyn Color> {
        match self.color {
            TetrominoColor::Cyan => Box::new(color::Cyan),
            TetrominoColor::Blue => Box::new(color::Blue),
            TetrominoColor::Yellow => Box::new(color::Yellow),
            TetrominoColor::LightYellow => Box::new(color::LightYellow),
            TetrominoColor::Green => Box::new(color::Green),
            TetrominoColor::Magenta => Box::new(color::Magenta),
            TetrominoColor::Red => Box::new(color::Red),
        }
    }

    fn rotate(&mut self, rotation: TetrominoRotation, well: &Well) {
        if self.is_move_allowed(0, 0, Some(rotation), well) {
            self.angle = self.get_angle_after_rotation(rotation);
        }
    }

    fn is_move_allowed(&self, dx: i8, dy: i8, rotation: Option<TetrominoRotation>, well: &Well) -> bool {
        let future = Tetromino {
            origin: Point(self.origin.0 + dx, self.origin.1 + dy),
            angle: match rotation {
                Some(rotation) => self.get_angle_after_rotation(rotation),
                None => self.angle
            },
            .. self.clone()
        };

        well.points.intersection(&future.get_current_set()).cloned().
            collect::<HashSet<Point>>().is_empty()
    }

    fn move_left(&mut self, well: &Well) {
        if self.is_move_allowed(-1, 0, None, well) {
            self.origin.0 -= 1;
        }
    }

    fn move_right(&mut self, well: &Well) {
        if self.is_move_allowed(1, 0, None, well) {
            self.origin.0 += 1;
        }
    }

    fn move_down(&mut self, well: &mut Well) -> bool {
        if self.is_move_allowed(0, 1, None, well) {
            self.origin.1 += 1;
            false
        } else {
            well.consume(self);
            true
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

    fn consume(&mut self, tetromino: &Tetromino) {
        self.points = self.points.union(&tetromino.get_current_set()).cloned().collect();
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
                                        format!("{}XX{}", color::Fg(tet.get_color().deref()), style::Reset)
                                    } else {
                                        format!("  ")
                                    }).unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }
    // write!(stdout, "{}", format!("{:?}", tet.angle)).unwrap();
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
                Key::Backspace  => Some(TetrisEvent::Skip),
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


    let mut tet = Tetromino::random();
    let mut well = Well::new();

    let mut stdout = stdout().into_raw_mode().unwrap(); // move out

    loop {
        match rx.recv().unwrap() {
            TetrisEvent::RotateCCW  => tet.rotate(TetrominoRotation::Counterclockwise, &well),
            TetrisEvent::RotateCW   => tet.rotate(TetrominoRotation::Clockwise, &well),
            TetrisEvent::MoveLeft   => tet.move_left(&well),
            TetrisEvent::MoveRight  => tet.move_right(&well),
            TetrisEvent::MoveDown   => if tet.move_down(&mut well) {tet = Tetromino::random()},
            TetrisEvent::Quit       => break,
            TetrisEvent::Skip       => tet = Tetromino::random(),
            _ => (), // implement drop
        }

        render(&mut stdout, &well, &tet);
        stdout.flush().unwrap();
    }

}