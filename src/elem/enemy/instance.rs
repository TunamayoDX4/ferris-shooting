use rand::Rng;
use super::*;

pub struct EnemyIdentGen(u64);
impl EnemyIdentGen {
    pub fn new() -> Self { Self(0) }
    pub fn gen(&mut self) -> u64 {
        let r = self.0;
        self.0 += 1;
        r
    }
}

pub struct EnemyInstance {
    enemy_ident: EnemyIdentGen, 
    pub enemies: EntityArray<Enemy>, 
    enemy_form_spawner: Vec<Option<enemy_type::formation::EnemyFormation>>, 
}
impl EnemyInstance {
    pub fn new() -> Self {     
        Self {
            enemy_ident: EnemyIdentGen::new(), 
            enemies: EntityArray::new([]), 
            enemy_form_spawner: Vec::new(), 
        }
    }

    pub fn update(
        &mut self, 
        cycle: &CycleMeasure, 
        varea: &VisibleField, 
    ) {
        let (chance, spawn_range) = crate::RNG.with(|r| {
            let mut r = (**r).borrow_mut();
            let chance = r.gen_range(0..100);
            let spawn_range = r.gen_range(0.0..1.0);
            (chance, spawn_range)
        });
        if chance < 2 {
            let enemy_form = enemy_type::formation::EnemyFormation::new(
                {
                    let area = varea.visible_area();
                    let spawn_x = area[0].x + (area[1].x - area[0].x) * spawn_range;
                    nalgebra::Point2::new(spawn_x, area[1].y)
                }, 
                cycle.dur, 
                None, 
            );
            
            if let Some(v) = self.enemy_form_spawner.iter_mut()
                .filter(|v| v.is_none())
                .next()
            {
                *v = Some(enemy_form);
            } else {
                self.enemy_form_spawner.push(Some(enemy_form));
            }
        }

        for e in self.enemy_form_spawner.iter_mut()
            .filter(|v| v.is_some())
        {
            if !e.as_mut().unwrap().update(
                cycle, 
                varea, 
                &mut self.enemy_ident, 
                &mut self.enemies
            ) {
                *e = None
            }
        }

        self.enemies.retain(|_, e| {
            e.update(cycle, varea)
        });
    }

    pub fn render_update(
        &self, 
        renderer: &mut simple2d::img_obj::ImgObjRender, 
    ) {
        self.enemies.render_update(renderer);
    }
}