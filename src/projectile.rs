#[path = "./ray.rs"]
mod ray;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Projectile {
    position: ray::Tuple,
    velocity: ray::Tuple
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Environment {
    gravity: ray::Tuple,
    wind: ray::Tuple
}

fn ticks(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position.add(&proj.velocity);
    let velocity = proj.velocity.add(&env.gravity).add(&env.wind);
    Projectile {
        position: position,
        velocity: velocity
    }
}

pub fn launch(pos_x: f32, pos_y:f32, pos_z: f32, v_x: f32, v_y: f32, v_z: f32) -> Vec<Projectile> {
    let initial = Projectile {
        position: ray::Tuple::point3(pos_x, pos_y, pos_z),
        velocity: ray::Tuple::vector3(v_x, v_y, v_z).normalize()
    };
    let env = Environment {
        gravity: ray::Tuple::vector3(0.0, -0.1, 0.0),
        wind: ray::Tuple::vector3(-0.001, 0.0, 0.0)
    };

    let mut moving = initial.clone();
    let mut vec = Vec::new();
    while moving.position.y() > 0.0 {
        moving = ticks(&env, &moving);
        vec.push(moving.clone());
    }
    vec
}

pub fn trace_trajectory(pos_by_ticks: &Vec<Projectile>) {
    let full_trace = pos_by_ticks.iter().fold(String::new(), |acc, w| {
        if acc.is_empty() {
            format!("{:?}", w)
        } else {
            format!("{}\n{:?}", acc, w)
        }
    });
    println!("{}", full_trace);
}