use super::*;

pub mod solo;
pub mod swarm;

/// 敵の編隊
pub struct EnemyFormation {
    position: nalgebra::Point2<f32>,
    cycle_dur: f32, 
    form_type: EnemyFormationType, 
}
impl EnemyFormation {
    pub fn new(
        position: nalgebra::Point2<f32>, 
        cycle_dur: f32, 
        form_type: Option<EnemyFormationType>, 
    ) -> Self { Self {
        position,
        cycle_dur,
        form_type: form_type.unwrap_or(EnemyFormationType::new(None)),
    } }

    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        ident: &mut EnemyIdentGen, 
        enemies: &mut EntityArray<Enemy>, 
    ) -> bool {
        let r = crate::RNG.with(|r| {
            self.form_type.spawn(
                cycle, varea, &mut *(**r).borrow_mut(), 
                ident, 
                enemies, 
                self.position, 
                self.cycle_dur, 
            )
        });
        self.cycle_dur += cycle.dur;
        r
    }
}

pub enum EnemyFormationType {
    Solo(solo::Solo), 
    Swarm(swarm::Swarm), 
}
impl Default for EnemyFormationType {
    fn default() -> Self {
        match crate::RNG.with(
            |r| (**r).borrow_mut().gen_range(0..100)
        ) {
            i @ _ if i < 10 => Self::Swarm(swarm::Swarm::new(None)), 
            _ => Self::Solo(solo::Solo::new(None)), 
        }
    }
}
impl Formation for EnemyFormationType {
    fn spawn(
        &self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        rng: &mut impl Rng, 
        ident: &mut EnemyIdentGen, 
        enemies: &mut EntityArray<Enemy>, 
        position: nalgebra::Point2<f32>, 
        cycle_dur: f32, 
    ) -> bool { match self {
        EnemyFormationType::Solo(s) => s.spawn(
            cycle, varea, rng, ident, 
            enemies, position, cycle_dur
        ),
        EnemyFormationType::Swarm(s) => s.spawn(
            cycle, varea, rng, ident, 
            enemies, position, cycle_dur,  
        ), 
    }}
}

trait Formation: Default {
    fn new(default: Option<Self>) -> Self {
        default.unwrap_or(Self::default())
    }
    fn spawn(
        &self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
        rng: &mut impl Rng, 
        ident: &mut EnemyIdentGen, 
        enemies: &mut EntityArray<Enemy>, 
        position: nalgebra::Point2<f32>, 
        cycle_dur: f32, 
    ) -> bool;
}