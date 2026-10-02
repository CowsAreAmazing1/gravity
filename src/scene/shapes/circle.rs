// pub fn center_vec(mut self, center: Vec2) -> Self{
//     self.center = center;
//     self
// }

// pub fn center_xy(mut self, x: f32, y: f32) -> Self{
//     self.center = vec2(x,y);
//     self
// }

// pub fn velocity_vec(mut self, velocity: Vec2) -> Self{
//     self.velocity += velocity;
//     self
// }

// pub fn velocity_xy(mut self, x: f32, y: f32) -> Self{
//     self.velocity += vec2(x,y);
//     self
// }

// pub fn orbit(mut self, orbit: bool) -> Self {
//     self.orbit = orbit;
//     self
// }

// pub fn velocity_scale(mut self, scale: f32) -> Self {
//     self.velocity_scale = scale;
//     self
// }

// pub fn build(self) -> Vec<Particle> {
//     (0..self.num_particles).map(|i| {
//         let angle = f64::TAU() / self.num_particles as f64 * i as f64;
//         let pos = [(self.radius as f64 * angle.cos() + self.center.x as f64) as f32, (self.radius as f64 * angle.sin() + self.center.y as f64) as f32];
//         if self.orbit {
//             let speed = (SOLAR_MASS as f64 * G as f64 / self.radius as f64).sqrt() * self.velocity_scale as f64;
//             Particle {
//                 pos,
//                 vel: [(self.velocity.x as f64 + speed * angle.sin()) as f32, (self.velocity.y as f64 - speed * angle.cos()) as f32],
//             }
//         } else {
//             Particle {
//                 pos,
//                 vel: [self.velocity.x, self.velocity.y],
//             }
//         }
//     }).collect::<Vec<_>>()
// }
