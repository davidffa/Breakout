use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{
    util::{Point, State, Vector2},
    SPEED, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const BAR_WIDTH: u32 = 80;
const BAR_THICKNESS: u32 = 6;

const TARGET_WIDTH: u32 = 75;
const TARGET_THICKNESS: u32 = 20;

const PROJ_SIZE: u32 = 15;

const TARGET_COLORS: [Color; 7] = [
    Color::RGB(255, 0, 0),
    Color::RGB(255, 127, 0),
    Color::RGB(255, 255, 0),
    Color::RGB(0, 255, 0),
    Color::RGB(0, 0, 255),
    Color::RGB(75, 0, 130),
    Color::RGB(148, 0, 211),
];

struct Target {
    rect: Rect,
    destroyed: bool,
    color: Color,
}

pub struct Game {
    proj: Rect,
    proj_dir: Vector2,
    bar: Rect,
    pub bar_dir_x: f32,
    pub state: State,
    targets: Vec<Target>,
}

impl Game {
    pub fn new() -> Self {
        let mut targets = Vec::new();

        for i in 0..9 {
            for j in 0..7 {
                let rect = Rect::new(
                    40 + i * TARGET_WIDTH as i32 + i * 5,
                    40 + j * TARGET_THICKNESS as i32 + j * 5,
                    TARGET_WIDTH,
                    TARGET_THICKNESS,
                );

                targets.push(Target {
                    rect,
                    destroyed: false,
                    color: TARGET_COLORS[j as usize],
                });
            }
        }

        Game {
            proj: Rect::new(
                (WINDOW_WIDTH) as i32 / 2 - PROJ_SIZE as i32 / 2,
                (WINDOW_HEIGHT - 30 - PROJ_SIZE) as i32,
                PROJ_SIZE,
                PROJ_SIZE,
            ),
            proj_dir: Vector2::new(1.0, -1.0),
            state: State::Paused,
            bar: Rect::new(
                WINDOW_WIDTH as i32 / 2 - BAR_WIDTH as i32 / 2,
                WINDOW_HEIGHT as i32 - 30,
                BAR_WIDTH,
                BAR_THICKNESS,
            ),
            bar_dir_x: 0.0,
            targets,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let (proj, proj_dir) = (&mut self.proj, &mut self.proj_dir);

        let mut np = Point::new(
            proj.x() as f32 + proj_dir.x * SPEED * dt,
            proj.y() as f32 + proj_dir.y * SPEED * dt,
        );

        if np.x < 0.0 || np.x > (WINDOW_WIDTH - PROJ_SIZE) as f32 {
            proj_dir.x *= -1.0;
            np.x = proj.x() as f32 + proj_dir.x * SPEED * dt;
        }

        if np.y < 0.0 || np.y > (WINDOW_HEIGHT - PROJ_SIZE) as f32 {
            proj_dir.y *= -1.0;
            np.y = proj.y() as f32 + proj_dir.y * SPEED * dt;
        }

        if proj.has_intersection(self.bar) {
            proj_dir.y = -1.0;

            if self.bar_dir_x == 0.0 {
                proj_dir.x = if proj_dir.x > 0.0 { 0.5 } else { -0.5 };
            } else {
                proj_dir.x = self.bar_dir_x;
            }

            np.x = proj.x() as f32 + proj_dir.x * SPEED * dt;
            np.y = proj.y() as f32 + proj_dir.y * SPEED * dt;
        }

        for target in self.targets.iter_mut() {
            if !target.destroyed && proj.has_intersection(target.rect) {
                target.destroyed = true;
                proj_dir.y *= -1.0;
                proj_dir.x *= -1.0;

                np.x = proj.x() as f32 + proj_dir.x * SPEED * dt;
                np.y = proj.y() as f32 + proj_dir.y * SPEED * dt;
                break;
            }
        }

        proj.set_x(np.x as i32);
        proj.set_y(np.y as i32);

        let bar = &mut self.bar;

        // Prevent bar from going off screen
        let mut nx = bar.x() as f32 + self.bar_dir_x * SPEED * dt;
        if nx < 0.0 {
            self.bar_dir_x = 0.0;
            nx = 0.0;
        } else if nx > (WINDOW_WIDTH - BAR_WIDTH) as f32 {
            self.bar_dir_x = 0.0;
            nx = (WINDOW_WIDTH - BAR_WIDTH) as f32;
        }
        bar.set_x(nx as i32);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(40, 42, 55));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(240, 240, 240));
        canvas.fill_rect(self.proj).unwrap();

        canvas.set_draw_color(Color::RGB(0, 210, 0));
        canvas.fill_rect(self.bar).unwrap();

        for target in &self.targets {
            if !target.destroyed {
                canvas.set_draw_color(target.color);
                canvas.fill_rect(target.rect).unwrap();
            }
        }

        canvas.present();
    }
}
