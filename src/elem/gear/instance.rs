use rand::Rng;

use super::*;

/// ギアのインスタンス配列
pub struct GearInstance {
    gear: EntityArray<Gear>, 
}
impl GearInstance {
    pub fn new() -> Self { Self {
        gear: EntityArray::new([]), 
    }}

    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        enemies: &mut enemy::instance::EnemyInstance, 
    ) {
        // 歯車の更新処理
        self.gear.retain(|_, g| g.update(cycle, varea, enemies));
    }

    /// ギアのスポーン
    pub fn spawn_gear(
        &mut self, 
        position: nalgebra::Point2<f32>, 
        rotation: f32, 
        velocity: Option<nalgebra::Vector2<f32>>, 
        gear_type: gear_type::GearType, 
        speed_ratio: f32, 
        diffuse_ratio: f32, 
        spawn_count: impl std::ops::RangeBounds<u32>, 
    ) {
        for _ in {
            0..{
                let spawn_count = {
                    let min = match spawn_count.start_bound() {
                        std::ops::Bound::Included(m) => *m,
                        std::ops::Bound::Excluded(m) => *m + 1,
                        std::ops::Bound::Unbounded => 0,
                    };
                    let max = match spawn_count.end_bound() {
                        std::ops::Bound::Included(x) => *x,
                        std::ops::Bound::Excluded(x) => *x - 1,
                        std::ops::Bound::Unbounded => 1,
                    };
                    min..max
                };
                if spawn_count.start == spawn_count.end { spawn_count.end }
                else { 
                    crate::RNG.with(|r| 
                        (**r).borrow_mut().gen_range(spawn_count)
                    )
                }
            }
        } { 
            let gear_type = gear_type.clone();

            // 初速・角度の計算
            let vel = gear_type.velocity0(speed_ratio, diffuse_ratio);
            let rotation = rotation + gear_type.angle_diffuse(diffuse_ratio).unwrap_or(0.);
            let (vel, rotation) = if let Some(
                vel0
            ) = velocity {
                let vel = nalgebra::Vector2::new(
                    vel * rotation.cos(), 
                    vel * rotation.sin(), 
                ) + vel0;
                let rotation = f32::atan2(vel.y, vel.x);
                let vel = f32::sqrt(vel.x.powi(2) + vel.y.powi(2));
                (vel, rotation)
            } else {
                (vel, rotation)
            };
            self.gear.push(Gear {
                pbody: GearPhysicBody { 
                    position, 
                    velocity: [
                        vel * rotation.cos(), 
                        vel * rotation.sin()
                    ].into(), 
                    vel, 
                    rotation, 
                    render_rotation: crate::RNG.with(
                        |r| (**r).borrow_mut()
                            .gen_range(-std::f32::consts::PI..std::f32::consts::PI)
                    ), 
                    render_rotation_speed: 0., 
                }, 
                gear_type, 
            });
        }
    }

    /// 描画構造体のアップデート
    pub fn renderer_update(
        &self, 
        renderer: &mut simple2d::img_obj::ImgObjRender, 
    ) {
        self.gear.render_update(renderer)
    }
}