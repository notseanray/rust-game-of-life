use core::fmt;
use std::{time::{SystemTime, UNIX_EPOCH}, sync::atomic::AtomicUsize};

#[derive(Clone, Copy, Debug)]
enum Pos {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Top,
    Bottom,
    Other,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    state: bool,
    next: bool,
    pos: Pos,
}

pub struct Board {
    data: Vec<Cell>,
    width: usize,
    height: usize,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::with_capacity(self.height * self.width);
        for (i, e) in self.data.iter().enumerate() {
            output.push_str(&format!("{:?}", e));
            if (i + self.width + 1) % self.width == 0 {
                output.push('\n');
            }
        }
        write!(f, "{}", output)
    }
}

const WIDTH: usize = 470;
const HEIGHT: usize = 100;

static GENERATION: AtomicUsize = AtomicUsize::new(0);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::with_capacity(self.width * self.height + 50);
        output.push_str("\x1B[H");
        for (e, i) in self.data.iter().enumerate() {
            if e % self.width == 0 {
                output.push('\n');
            }
            output.push(if i.state { '█' } else { ' ' });
        }
        output.push_str(&format!("\nGENERATION {}", GENERATION.load(std::sync::atomic::Ordering::Relaxed)));
        write!(f, "{}", output)
    }
}

impl Board {
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::with_capacity(WIDTH * HEIGHT),
            width: WIDTH,
            height: HEIGHT,
        }
    }
    pub(crate) fn fill_random(&mut self) {
        let positions = [
            Pos::TopLeft,
            Pos::Top,
            Pos::TopRight,
            Pos::Left,
            Pos::Other,
            Pos::Right,
            Pos::BottomLeft,
            Pos::Bottom,
            Pos::BottomRight,
        ];
        let mut posindex = 0;
        for i in 0..self.width * self.height {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros();
            if posindex != 0 {
                if i > 0 && i < self.width - 1 {
                    posindex = 1;
                } else if i == self.width - 1 {
                    posindex = 2;
                } else if i % self.width == 0 && self.width * self.height - self.width != i {
                    posindex = 3;
                } else if (i + self.width + 1) % self.width == 0
                    && self.width * self.height - 1 != i
                {
                    posindex = 5;
                } else if i == self.width * self.height - (self.width) {
                    posindex = 6;
                } else if i > self.width * self.height - (self.width + 1)
                    && i < self.width * self.height - 1
                {
                    posindex = 7;
                } else if i == self.width * self.height - 1 {
                    posindex = 8;
                } else {
                    posindex = 4;
                }
            }
            self.data.push(Cell {
                state: time % (i + 1) as u128 > (i as f32 / 2.0 ) as u128,
                next: true,
                pos: positions[posindex],
            });
            if posindex == 0 {
                posindex += 1;
            }
        }
        self.tick();
    }
    pub(crate) fn tick(&mut self) {
        GENERATION.swap(GENERATION.load(std::sync::atomic::Ordering::Relaxed) + 1, std::sync::atomic::Ordering::Relaxed);
        for (index, i) in self.data.clone().iter().enumerate() {
            self.data[index].next = match i.pos {
                Pos::TopLeft => {
                    let mut offset: u8 = 0;
                    if self.data[1].state {
                        offset += 1;
                    }
                    if self.data[self.width].state {
                        offset += 1;
                    }
                    if self.data[self.width + 1].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::TopRight => {
                    let mut offset: u8 = 0;
                    if self.data[self.width * 2 - 1].state {
                        offset += 1;
                    }
                    if self.data[self.width - 2].state {
                        offset += 1;
                    }
                    if self.data[self.width * 2 - 2].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::BottomLeft => {
                    let mut offset: u8 = 0;
                    if self.data[index + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width].state {
                        offset += 1;
                    }

                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::BottomRight => {
                    let mut offset: u8 = 0;
                    if self.data[self.width * self.height - (self.width + 2)].state {
                        offset += 1;
                    }
                    if self.data[self.width * self.height - (self.width + 1)].state {
                        offset += 1;
                    }
                    if self.data[self.width * self.height - 1].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::Top => {
                    let mut offset: u8 = 0;
                    if self.data[index - 1].state {
                        offset += 1;
                    }
                    if self.data[index + 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width].state {
                        offset += 1;
                    }
                    if self.data[index + self.width + 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width - 1].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::Left => {
                    let mut offset: u8 = 0;
                    if self.data[index - self.width].state {
                        offset += 1;
                    }
                    if self.data[index + 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width].state {
                        offset += 1;
                    }
                    if self.data[index + (self.width - 1)].state {
                        offset += 1;
                    }
                    if self.data[index - (self.width - 1)].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::Right => {
                    let mut offset: u8 = 0;
                    if self.data[index - self.width].state {
                        offset += 1;
                    }
                    if self.data[index - 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width].state {
                        offset += 1;
                    }
                    if self.data[index + (self.width - 1)].state {
                        offset += 1;
                    }
                    if self.data[index - (self.width - 1)].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::Bottom => {
                    let mut offset: u8 = 0;
                    if self.data[index - 1].state {
                        offset += 1;
                    }
                    if self.data[index + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width].state {
                        offset += 1;
                    }
                    if self.data[index - self.width + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width - 1].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
                Pos::Other => {
                    let mut offset: u8 = 0;
                    if self.data[index - 1].state {
                        offset += 1;
                    }
                    if self.data[index + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width].state {
                        offset += 1;
                    }
                    if self.data[index + self.width].state {
                        offset += 1;
                    }
                    if self.data[index - self.width + 1].state {
                        offset += 1;
                    }
                    if self.data[index - self.width - 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width + 1].state {
                        offset += 1;
                    }
                    if self.data[index + self.width - 1].state {
                        offset += 1;
                    }
                    matches!(
                        (offset == 2, offset == 3, self.data[index].state),
                        (false, true, false) | (false, true, true) | (true, false, true)
                    )
                }
            }
        }
        for i in self.data.iter_mut() {
            i.state = i.next;
        }
    }
}
