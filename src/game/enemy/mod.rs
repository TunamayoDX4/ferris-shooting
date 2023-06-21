use super::*;

pub mod enemy;
pub mod spawn;
pub mod spawn_ctrl;

pub struct EnemyIdentMaster(u64);
impl EnemyIdentMaster {
    pub fn issue(&mut self) -> enemy::EnemyIdent {
        let r = enemy::EnemyIdent(self.0);
        self.0 = self.0.checked_add(1).unwrap_or(0);
        r
    }
}

pub struct EnemyInstances {
    pub enemy: enemy::EnemyArray, 
    spawner: spawn::EnemySpawnerArray, 
    spctrl: spawn_ctrl::SpawnerController, 
}
impl EnemyInstances {
    pub fn new() -> Self { Self {
        enemy: enemy::EnemyArray::new(), 
        spawner: spawn::EnemySpawnerArray::new(), 
        spctrl: spawn_ctrl::SpawnerController::new(), 
    } }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
    ) {
        self.spctrl.update(
            cycle, 
            varea, 
            &mut self.spawner
        );
        self.spawner.update(
            cycle, 
            varea, 
            &mut self.enemy
        );
        self.enemy.update(cycle, varea, &mut self.spawner);
    }

    pub fn push_spawner(
        &mut self, 
        spawner: spawn::EnemySpawner, 
    ) {
        self.spawner.push(spawner)
    }

    pub fn rendering(
        &self, 
        renderer: &mut crate::renderer::FSRenderer, 
    ) {
        self.enemy.rendering(renderer)
    }
}