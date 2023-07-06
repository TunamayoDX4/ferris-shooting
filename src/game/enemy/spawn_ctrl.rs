use rand::Rng;

use super::{*, spawn::{EnemySpawner, SpawnerType}, enemy::EnemyType};

pub struct SpawnerController {
}

impl SpawnerController {
    pub fn new() -> Self { Self {}}

    pub fn update(
        &mut self, 
        cycle: &cycle_measure::CycleMeasure, 
        varea: &simple2d::types::VisibleField, 
        spawner: &mut super::spawn::EnemySpawnerArray, 
    ) {
        if let Some((
            enemy, pos
        )) = crate::RNG.with(|r| {
            let mut r = (**r).borrow_mut(); 
            let (
                head_pos, 
                line
            ) = {
                let ve = varea.visible_edge();

                (ve[2], ve[3] - ve[2])
            };
            let pos = head_pos + r.gen_range(0.0..1.0) * line;
            let chance = r.gen_range(0..100);
            if !r.gen_bool(1. / 8.) { return None }
            let enemy = if chance < 3 {
                EnemyType::DangPtr
            } else if chance < 10 {
                EnemyType::DataRace
            } else if chance < 25 {
                EnemyType::NullPtr
            } else {
                EnemyType::UndefBeh
            };
            Some((enemy, pos))
        }) {
            spawner.push(EnemySpawner::new(
                pos, 
                SpawnerType::Solo(enemy), 
            ))
        }
    }
}