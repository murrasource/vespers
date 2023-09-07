use macroquad::prelude::*;

pub type Point = (f32, f32);

type Size = (f32, f32);

enum Stimulus {
    Filled,
    Empty
}

impl Stimulus {
    fn color(&self) -> Color {
        match *self {
            Stimulus::Filled => SKYBLUE,
            Stimulus::Empty => YELLOW,
        }
    }

    fn invert(&self) -> Stimulus{
        match *self {
            Stimulus::Filled => Stimulus::Empty,
            Stimulus::Empty => Stimulus::Filled,
        }
    }
}

pub struct Patch {
    location: Point,
    size: Size,
    state: Stimulus
}

impl Patch {
    fn draw(&self) {
        let color = &self.state.color();
        draw_rectangle(self.location.0, self.location.1, self.size.0, self.size.1, *color)
    }

    fn invert(&mut self) {
        self.state = self.state.invert();
        let color = &self.state.color();
        draw_rectangle(self.location.0, self.location.1, self.size.0, self.size.1, *color);
    }
}

pub struct Pattern {
    pub patches: Vec::<Patch>,
    pub rows: i16,
    pub columns: i16
}

impl Pattern {
    pub fn init(&mut self, point: &Point, size: &Size) {
        let mut count = 1;

        for r in 1..(self.rows + 1) {
            for c in 1..(self.columns + 1) {
                let offset_x: f32 = (r as f32) * size.0;
                let offset_y: f32 = (c as f32) * size.1;
                
                let patch = Patch{
                    location: find_point(&point, offset_x, offset_y),
                    size: *size,
                    state: assign_state(&count)
                };

                patch.draw();

                self.patches.push(patch);
                
                count += 1;
            }
            count += 1;
        }
    }

    pub fn draw(&self) {
        for patch in &self.patches {
            &patch.draw();
        }
    }

    pub fn invert(&mut self) {
        for patch in &mut self.patches {
            let _ = &patch.invert();
        }
    }
}

fn assign_state(n: &i16) -> Stimulus {
    if (*n) % 2 == 0 {
        return Stimulus::Empty;
    } else {
        return Stimulus::Filled;
    }
}

fn find_point(start: &Point, offset_x: f32, offset_y: f32) -> Point {
    let point = (start.0 + offset_x, start.1 + offset_y);
    return point;
}