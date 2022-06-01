use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::gfx::primitives::DrawRenderer;
use cgmath::Vector2;

struct VerletObject {
    position_current: Vector2<f32>,
    position_old: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl VerletObject {
    pub fn draw(&self, canvas: &sdl2::render::Canvas<sdl2::video::Window>) {
        let pos_x = i16::try_from(self.position_current[0].round() as i32);
        let pos_y = i16::try_from(self.position_current[1].round() as i32);

        let pos_x = match pos_x {
            Ok(pos_x) => pos_x,
            Err(e) => panic!("{}", e),
        };

        let pos_y = match pos_y {
            Ok(pos_y) => pos_y,
            Err(e) => panic!("{}", e),
        };

        let res: Result<(), String> = canvas.filled_circle(pos_x, pos_y, 20, Color::RGB(255, 255, 255));
        match res {
            Ok(()) => {},
            Err(e) => panic!("{}", e),
        }
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
    fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_constraint();
        self.solve_collisions();
        self.update_positions(dt);

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

            if dist > radius - 20f32 {
                let n: Vector2<f32> = to_obj / dist;
                obj.position_current = position + n * (radius - 20f32);
            }
        }
    }

    fn solve_collisions(&mut self) {
        let object_count: &usize = &self.objects.len();
        let mut i = 0;
        while i < *object_count {
            let mut k = &i + 1;
            while k < *object_count {
                let collision_axis: Vector2<f32> = self.objects[i].position_current - self.objects[k].position_current;
                let dist: f32 = (collision_axis[0].powf(2f32) + collision_axis[1].powf(2f32)).sqrt();
                if dist < 40f32 {
                    let n: Vector2<f32> = collision_axis / dist;
                    let delta: f32 = 40f32 - dist;
                    self.objects[i].position_current += 0.96f32 * delta * n;
                    self.objects[k].position_current -= 0.96f32 * delta * n;
                }
            
                k += 1;
            }

            i += 1;
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

    let objects: Vec<VerletObject> = vec![VerletObject { position_current: Vector2::new(700f32, 400f32), position_old: Vector2::new(700f32, 400f32), acceleration: Vector2::new(0f32, 0f32) }, VerletObject { position_current: Vector2::new(400f32, 500f32), position_old: Vector2::new(400f32, 500f32), acceleration: Vector2::new(0f32, 0f32) }];
    let mut solver: Solver = Solver { gravity: Vector2::new(0f32, 1000f32), objects: objects};


    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _constr = canvas.filled_circle(600, 400, 300, Color::RGB(150, 150, 150));

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
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
