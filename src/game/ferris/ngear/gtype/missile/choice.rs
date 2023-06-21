//! ミサイルの追尾関数の実装
use super::{
    *, 
    super::super::GPhysWrap, 
};

/// 単純な最も近い対象を選ぶ関数
pub fn choice_simple_neerest(
    phys: &GPhysWrap, 
    enemies: &enemy::enemy::EnemyArray, 
    mut filter: impl FnMut(
        &GPhysWrap,
        &enemy::enemy::Enemy
    ) -> bool, 
) -> Option<enemy::enemy::EnemyRef> {
    enemies.enemies.iter()
        .filter(|e| {
            filter(
                phys, 
                e.entity, 
            )
        })
        .map(|e| {
            let dist = e.entity.position - phys.position();
            let dist = dist.x.powi(2) + dist.y.powi(2);
            let enemyref = EnemyRef {
                ident: e.entity.ident.clone(), 
                idx: e.idx, 
            };
            (enemyref, dist.sqrt().abs())
        })
        .fold(
            None::<(EnemyRef, f32)>, 
            |
                init, 
                (
                    enemyref, 
                    dist
                )
            | match init {
                None => Some((enemyref, dist)), 
                Some((_, ed)) if dist <= ed => Some((enemyref, dist)), 
                Some(_) => init, 
            }
        )
        .map(|e| e.0)
}