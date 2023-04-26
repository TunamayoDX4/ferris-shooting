use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    UndefinedBehavior, 
    NullPointer, 
    DataRace, 
    DanglingPointer, 
}
impl EnemyType {
    pub fn spawn() -> Self { crate::RNG.with(
        |r| match (**r).borrow_mut().gen_range(0..128) {
            v @ _ if 120 <= v => Self::DanglingPointer, 
            v @ _ if 105 <= v => Self::DataRace, 
            v @ _ if 80 <= v => Self::NullPointer, 
            _ => Self::UndefinedBehavior, 
        }
    ) }

    pub fn size(&self) -> f32 { match self {
        Self::UndefinedBehavior => 64., 
        Self::NullPointer => 64., 
        Self::DataRace => 64., 
        Self::DanglingPointer => 64., 
    }}

    pub fn tex_coord(&self) -> [f32; 2] { match self {
        Self::UndefinedBehavior => [0., 0.], 
        Self::NullPointer => [64., 0.], 
        Self::DataRace => [128., 0.], 
        Self::DanglingPointer => [192., 0.], 
    }}

    pub fn tex_size(&self) -> [f32; 2] { match self {
        _ => [64., 64.], 
    }}

    pub fn default_speed(&self, speed_ratio: f32) -> f32 {
        speed_ratio * match self {
            EnemyType::UndefinedBehavior => 160.,
            EnemyType::NullPointer => 240.,
            EnemyType::DataRace => 240.,
            EnemyType::DanglingPointer => 120.,
        }
    }

    pub fn default_render_rot_speed(&self) -> f32 { crate::RNG.with(
        |r| if let Some((def_rot, range)) = match self {
            EnemyType::UndefinedBehavior => None,
            EnemyType::NullPointer => None,
            EnemyType::DataRace => Some((360., 120.)),
            EnemyType::DanglingPointer => Some((120., 0.5)),
        } {
            let rot: f32 = (**r).borrow_mut().gen_range(-range..range);
            rot.signum() * def_rot + rot
        } else {
            0.
        } * (std::f32::consts::PI / 180.)
    )}
}