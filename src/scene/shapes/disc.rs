use std::f32::consts::TAU;

use nannou::{
    glam::{Vec2, vec2},
    math::Vec2Angle,
    rand::{random, random_range},
};

use crate::{
    scene::scene_layout::{CreationOperation, FillWithDust, SetupObject},
    sim::system::Dust,
};

#[derive(Debug)]
pub struct Disc {
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    ops: Vec<CreationOperation>,
}

impl SetupObject for Disc {
    fn add_operation(mut self, op: CreationOperation) -> Self {
        self.ops.push(op);
        self
    }
}

impl Disc {
    pub fn new() -> Self {
        Disc {
            inner_radius: 0.0,
            outer_radius: 1.0,
            start_angle: 0.0,
            end_angle: TAU,
            ops: vec![],
        }
    }

    pub fn start_angle(mut self, angle: f32) -> Self {
        self.start_angle = angle;
        self
    }
    pub fn end_angle(mut self, angle: f32) -> Self {
        self.end_angle = angle;
        self
    }
    pub fn inner_radius(mut self, radius: f32) -> Self {
        self.inner_radius = radius;
        self
    }
    pub fn outer_radius(mut self, radius: f32) -> Self {
        self.outer_radius = radius;
        self
    }
    pub fn radius(mut self, radius: f32) -> Self {
        self.inner_radius = 0.0;
        self.outer_radius = radius;
        self
    }
    pub fn ring(mut self, radius: f32) -> Self {
        self.inner_radius = radius;
        self.outer_radius = radius;
        self
    }
}

impl Default for Disc {
    fn default() -> Self {
        Self::new()
    }
}

impl FillWithDust for Disc {
    fn build(&self, num: u32, target: &mut Vec<Dust>) {
        let Self {
            inner_radius: inner,
            outer_radius: outer,
            start_angle: start,
            end_angle: end,
            ops,
        } = self;

        let phi = (1.0 + 5_f64.sqrt()) / 2.0;
        let i_2 = (*inner) as f64 * (*inner) as f64;
        let o_2 = (*outer) as f64 * (*outer) as f64;
        let delta_radius = o_2 - i_2;
        let delta_angle = *end as f64 - *start as f64;

        for i in 0..num {
            let r_2 = i_2 + delta_radius * (i as f64 / num as f64);
            let r = r_2.sqrt();

            let angle = *start as f64 + delta_angle * (i as f64 / phi).fract();

            let mut pos = vec2((r * angle.cos()) as f32, (r * angle.sin()) as f32);
            let mut vel = Vec2::ZERO;

            for op in ops {
                match op {
                    CreationOperation::CenterOffset(v) => {
                        pos += *v;
                    }
                    CreationOperation::VelocityOffset(v) => {
                        vel += *v;
                    }
                    CreationOperation::VelocityScale(s) => {
                        vel *= *s;
                    }
                    CreationOperation::Orbit(center, mass, clockwise) => {
                        let rel_pos = *center - pos;
                        let dist = rel_pos.length();
                        let angle = rel_pos.angle();
                        let speed = (mass / dist).sqrt();
                        let sign = if *clockwise { -1.0 } else { 1.0 };
                        vel += sign * speed * vec2(angle.sin() as f32, -angle.cos() as f32)
                    }
                    CreationOperation::RotateAround(center, angle) => {
                        let rel_pos = pos - *center;
                        let cos_angle = angle.cos();
                        let sin_angle = angle.sin();
                        pos = *center
                            + vec2(
                                rel_pos.x * cos_angle - rel_pos.y * sin_angle,
                                rel_pos.x * sin_angle + rel_pos.y * cos_angle,
                            );
                    }
                }
            }
            target.push(Dust::new(pos, vel));
        }
    }

    fn build_random(&self, num: u32, target_vec: &mut Vec<Dust>) {
        let Self {
            inner_radius: inner,
            outer_radius: outer,
            start_angle: start,
            end_angle: end,
            ops,
        } = self;

        target_vec.append(
            &mut (0..num)
                .map(|_| {
                    let r = (inner.powi(2)) + (outer.powi(2) - inner.powi(2)) * random::<f32>();
                    let a = random_range(*start, *end);
                    let mut pos = r.sqrt() * vec2(a.cos(), a.sin());
                    let mut vel = Vec2::ZERO;

                    for op in ops {
                        match op {
                            CreationOperation::CenterOffset(v) => {
                                pos += *v;
                            }
                            CreationOperation::VelocityOffset(v) => {
                                vel += *v;
                            }
                            CreationOperation::VelocityScale(s) => {
                                vel *= *s;
                            }
                            CreationOperation::Orbit(center, mass, clockwise) => {
                                let rel_pos = *center - pos;
                                let tangent = rel_pos.perp().normalize();
                                let dist = rel_pos.length();
                                let speed = (mass / dist).sqrt();
                                let sign = if *clockwise { 1.0 } else { -1.0 };
                                vel += sign * speed * tangent;
                            }
                            CreationOperation::RotateAround(center, angle) => {
                                let rel_pos = pos - *center;
                                let cos_angle = angle.cos();
                                let sin_angle = angle.sin();
                                pos = *center
                                    + vec2(
                                        rel_pos.x * cos_angle - rel_pos.y * sin_angle,
                                        rel_pos.x * sin_angle + rel_pos.y * cos_angle,
                                    );
                            }
                        }
                    }

                    Dust::new(pos, vel)
                })
                .collect::<Vec<Dust>>(),
        );
    }
}
