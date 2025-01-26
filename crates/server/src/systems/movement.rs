use shipyard::{Component, IntoIter, View, ViewMut};
use vek::Vec3;

#[derive(Component)]
pub struct Pos(pub Vec3<f32>);

#[derive(Component)]
pub struct Vel(pub Vec3<f32>);


pub fn movement_system(v_vel: View<Vel>, mut v_pos: ViewMut<Pos>) {
    println!("- MOVEMENT");
    for (pos, vel) in (&mut v_pos, &v_vel).iter() {
        pos.0 += vel.0;
    }
}
