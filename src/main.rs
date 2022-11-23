use macroquad::prelude::*;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 800;
const WIDTHF: f32 = WIDTH as f32;
const HEIGHTF: f32 = HEIGHT as f32;
const CENTER: Vec2 = Vec2::new(WIDTHF/2.0, HEIGHTF/2.0);
const G: f32 = 2000.0;
const T: f32 = 20.0;

struct Object {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32
}

impl Object {
    fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32) -> Object {
        Object { position, velocity, acceleration, mass }
    }

    fn update(&mut self) {
        let f = (G*self.mass)/(self.position-CENTER).length().powi(2)*Vec2::new(CENTER.x-self.position.x, CENTER.y-self.position.y);
        self.acceleration = f*self.mass;

        self.velocity += self.acceleration * get_frame_time() * T;
        self.position += self.velocity * get_frame_time() * T;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Gravity".to_string(),
        window_width: WIDTH,
        window_height: HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut object1 = Object::new(Vec2::new(100.0, 200.0), Vec2::new(20.0, 0.0), Vec2::new(0.0, 0.0), 1.0);
    let mut object2 = Object::new(Vec2::new(100.0, 200.0), Vec2::new(20.0, 0.0), Vec2::new(0.0, 0.0), 1.0);

    loop {
        clear_background(GRAY);

        //Update
        object1.update();
        object2.update();

        //Draw
        draw_circle(CENTER.x, CENTER.y, 5.0, BLACK);

        draw_circle(object1.position.x, object.position.y, 10.0, RED);
        draw_line(object.position.x, object.position.y, object.position.x+object.velocity.x*50.0, object.position.y, 2.0, GREEN);
        draw_line(object.position.x, object.position.y, object.position.x, object.position.y+object.velocity.y*50.0, 2.0, BLUE);

        draw_circle(object.position.x, object.position.y, 10.0, RED);
        draw_line(object.position.x, object.position.y, object.position.x+object.velocity.x*50.0, object.position.y, 2.0, GREEN);
        draw_line(object.position.x, object.position.y, object.position.x, object.position.y+object.velocity.y*50.0, 2.0, BLUE);

        next_frame().await;
    }
}