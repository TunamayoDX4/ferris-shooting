use std::collections::VecDeque;

use super::{*, enemy::EnemyArray};

pub struct EnemySpawnerArray {
    spawner: Vec<Option<EnemySpawner>>, 
    removed: VecDeque<usize>, 
}
impl EnemySpawnerArray {
    pub fn new() -> Self { Self {
        spawner: Vec::new(), 
        removed: VecDeque::new(), 
    } }

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        enemies: &mut EnemyArray, 
    ) {
        self.spawner.iter_mut()
            .enumerate()
            .filter(|(_, es)| es.is_some())
            .for_each(|(
                idx, 
                es
            )| if !es.as_mut().unwrap().update(
                cycle, 
                varea, 
                enemies
            ) {
                *es = None;
                self.removed.push_back(idx);
            })
    }

    pub fn push(
        &mut self, 
        spawner: EnemySpawner, 
    ) {
        if let Some(instance) = self.removed.pop_front()
            .map(|idx| self.spawner.get_mut(idx))
            .flatten()
        {
            *instance = Some(spawner);   
        } else {
            self.spawner.push(Some(spawner));
        }
    }
}

pub enum SpawnerType {
    Solo(enemy::EnemyType), 
}
impl SpawnerType {
    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        position: nalgebra::Point2<f32>, 
        cycle_time: f32, 
        enemies: &mut EnemyArray, 
    ) -> bool { match self {
        Self::Solo(et) => {
            enemies.spawn(
                et.clone(), 
                position, 
                -std::f32::consts::PI * 0.5, 
                true, 
            );
            false
        }
    }}
}

pub struct EnemySpawner {
    position: nalgebra::Point2<f32>, 
    cycle_time: f32, 
    spawner_type: SpawnerType, 
}
impl EnemySpawner {
    pub fn new(
        position: nalgebra::Point2<f32>, 
        spawner_type: SpawnerType, 
    ) -> Self { Self {
        position,
        cycle_time: 0.,
        spawner_type,
    }}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        enemies: &mut EnemyArray, 
    ) -> bool {
        let res = self.spawner_type.update(
            cycle, 
            varea, 
            self.position, 
            self.cycle_time, 
            enemies, 
        );
        self.cycle_time += cycle.dur;
        res
    }
}