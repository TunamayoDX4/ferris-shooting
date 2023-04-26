use super::*;

pub mod enemy_type;

pub mod instance;

pub struct Enemy {
    pub ident: u64, 
    pub health: u32, 
    position: nalgebra::Point2<f32>, 
    rotation: f32, 
    render_rotation: f32, 
    render_rot_speed: f32, 
    vel: f32, 
    velocity: nalgebra::Vector2<f32>, 
    enemy_type: enemy_type::EnemyType, 
}
impl Enemy {

    /// 移動処理
    fn moving(
        &mut self, 
        cycle: &CycleMeasure, 
    ) {
        let prev_position = self.position;
        self.position += nalgebra::Vector2::new(
            self.vel * self.rotation.cos(), 
            self.vel * self.rotation.sin(), 
        ) * cycle.dur;
        self.velocity = (self.position - prev_position) * cycle.cps;
    }

    /// 描画オブジェの更新
    fn render_obj_update(
        &mut self, 
        cycle: &CycleMeasure, 
    ) {
        self.render_rotation += self.render_rot_speed * cycle.dur;
    }

    /// 生存チェック処理
    fn alive(
        &self, 
        varea: &VisibleField, 
    ) -> bool {
        varea.visible_area()[0].y <= self.position.y + self.enemy_type.size()[1] * 0.5
        && self.health != 0
    }

    /// 更新処理
    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
    ) -> bool {
        self.render_obj_update(cycle);
        self.moving(cycle);

        self.alive(varea)
    }
}

impl InstanceGen<ImgObjInstance> for Enemy {
    fn generate(&self) -> ImgObjInstance {
        ImgObjInstance { 
            position: self.position.into(), 
            size: self.enemy_type.size().into(), 
            rotation: self.render_rotation, 
            tex_coord: self.enemy_type.tex_coord(), 
            tex_size: self.enemy_type.tex_size(), 
            tex_rev: [false, false], 
        }
    }
}
impl physic_body::PhysicBody for Enemy {
    fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    fn size(&self) -> nalgebra::Vector2<f32> {
        self.enemy_type.size()
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }

    fn velocity(&self) -> nalgebra::Vector2<f32> {
        self.velocity
    }
}