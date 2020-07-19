mod projectile;


fn main() {
    println!("Hello, world!");
    println!("Cannon ball xp");

    cannon_ball(0.0, 1.0, 0.0, 1.0, 1.0, 0.0);
    cannon_ball(0.0, 1.0, 0.0, 5.0, 3.0, 0.0);

}

fn cannon_ball(pos_x: f32, pos_y:f32, pos_z: f32, v_x: f32, v_y: f32, v_z: f32) {
    println!("Projectile a cannon ball simulation");
    println!("Start from ({}, {}, {}) with velocity ({}, {}, {})", pos_x, pos_y, pos_z, v_x, v_y, v_z);
    let pos_by_ticks = projectile::launch(pos_x, pos_y, pos_z, v_x, v_y, v_z);
    println!("use {} ticks to arrive on floor", pos_by_ticks.len());
    projectile::trace_trajectory(&pos_by_ticks);
}