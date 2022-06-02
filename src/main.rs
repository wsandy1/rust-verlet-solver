use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use cgmath::Vector2;
use rand::Rng;

struct VerletObject {
    position_current: Vector2<f32>,
    position_old: Vector2<f32>,
    acceleration: Vector2<f32>,
    radius: i16,
    color: Color,
}

impl VerletObject {
    pub fn draw(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>) {
        let pos_x = i16::try_from(self.position_current[0].round() as i32).unwrap();
        let pos_y = i16::try_from(self.position_current[1].round() as i32).unwrap();
        canvas.filled_circle(pos_x, pos_y, self.radius, self.color).unwrap();
    }

    fn update_position(&mut self, dt: f32) {
        let velocity: Vector2<f32> = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;
        self.acceleration = Vector2::new(0f32, 0f32);
    }

    fn accelerate(&mut self, acc: Vector2<f32>) {
        self.acceleration += acc;
    }
}

struct Solver {
    gravity: Vector2<f32>,
    objects: Vec<VerletObject>,
}

impl Solver {
    fn add_object(&mut self, x: f32, y: f32, radius: i16, color: Color) {
        self.objects.push(VerletObject { position_current: Vector2::new(x, y), position_old: Vector2::new(x, y), acceleration: Vector2::new(0f32, 0f32), radius: radius, color: color });
    }

    fn update(&mut self, dt: f32) {
        const SUB_STEPS: u32 = 16;
        let sub_dt: f32 = dt / SUB_STEPS as f32;
        for _ in 0..SUB_STEPS {
            self.apply_gravity();
            self.apply_constraint();
            self.solve_collisions();
            self.update_positions(sub_dt);
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for obj in self.objects.iter_mut() {
            obj.update_position(dt);
        }
    }
    
    fn apply_gravity(&mut self) {
        for obj in self.objects.iter_mut() {
            obj.accelerate(self.gravity);
        }
    }

    fn apply_constraint(&mut self) {
        let position: Vector2<f32> = Vector2::new(600f32, 400f32);
        let radius: f32 = 300f32;
        for obj in self.objects.iter_mut() {
            let to_obj: Vector2<f32> = obj.position_current - position;
            let dist: f32 = (to_obj[0].powf(2f32) + to_obj[1].powf(2f32)).sqrt();

            if dist > radius - obj.radius as f32 {
                let n: Vector2<f32> = to_obj / dist;
                obj.position_current = position + n * (radius - obj.radius as f32);
            }
        }
    }

    fn solve_collisions(&mut self) {
        let object_count: &usize = &self.objects.len();
        for i in 0..*object_count {
            for k in (&i+1)..*object_count {
                let collision_axis: Vector2<f32> = self.objects[i].position_current - self.objects[k].position_current;
                let dist: f32 = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                let min_dist: i16 = self.objects[i].radius + self.objects[k].radius;
                if dist < min_dist as f32 {
                    let n: Vector2<f32> = collision_axis / dist;
                    let delta: f32 = min_dist as f32 - dist;
                    self.objects[i].position_current += 0.5f32 * delta * n;
                    self.objects[k].position_current -= 0.5f32 * delta * n;
                }
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-physics", 1200, 800)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut solver: Solver = Solver { gravity: Vector2::new(0f32, 1000f32), objects: vec![]};

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.filled_circle(600, 400, 300, Color::RGB(150, 150, 150)).unwrap();

        for object in solver.objects.iter() {
            object.draw(&canvas);
        }

        solver.update(0.0167f32);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    let mut rng = rand::thread_rng();
                    solver.add_object(x as f32, y as f32, rng.gen_range(5..50), Color::RGB(rng.gen_range(0..=255), rng.gen_range(0..=255), rng.gen_range(0..=255)));
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
