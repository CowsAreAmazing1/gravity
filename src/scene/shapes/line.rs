// pub fn center_vec(mut self, center: Vec2) -> Self {
//     self.center = center;
//     self
// }

// pub fn center_xy(mut self, x: f32, y: f32) -> Self {
//     self.center = vec2(x, y);
//     self
// }

// pub fn velocity_vec(mut self, velocity: Vec2) -> Self {
//     self.velocity += velocity;
//     self
// }

// pub fn velocity_xy(mut self, x: f32, y: f32) -> Self {
//     self.velocity += vec2(x, y);
//     self
// }

// pub fn horizontal(mut self) -> Self {
//     self.horizontal = true;
//     self
// }

// pub fn vertical(mut self) -> Self {
//     self.horizontal = false;
//     self
// }

// pub fn build(self) -> Vec<Particle> {
//     (0..self.num_particles)
//         .map(|i| {
//             let t = i as f64 / (self.num_particles - 1) as f64;

//             let pos = if self.horizontal {
//                 [
//                     (self.center.x as f64 - self.length as f64 * 0.5 + t * self.length as f64)
//                         as f32,
//                     self.center.y,
//                 ]
//             } else {
//                 [
//                     self.center.x,
//                     (self.center.y as f64 - self.length as f64 * 0.5 + t * self.length as f64)
//                         as f32,
//                 ]
//             };

//             let vel = self.velocity.into();

//             Particle { pos, vel }
//         })
//         .collect::<Vec<_>>()
// }
