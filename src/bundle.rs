use crate::*;
use std::fmt::Debug;

/// Creates a bundle of systems for the game_features crate.
/// ## Generics
/// - S: Stats Id
/// - E: Effectors Id
/// - I: Items Id
/// - K: Skill
/// - ST: Inventory SlotTypes. Defaults to `()`.
/// - U: User ItemInstance Data. Defaults to `()`.
pub struct GameFeaturesSystemBundle<S, E, I, K, ST = (), U = ()> {
    _phantom: std::marker::PhantomData<(S, E, I, K, ST, U)>,
}

impl<
        S: Eq + Hash + Send + Sync + 'static + Clone,
        E: Eq + Hash + Send + Sync + 'static + Clone,
        I: Eq + Hash + Send + Sync + 'static + Clone + Debug,
        K: Eq + Hash + Send + Sync + 'static + Debug,
        ST: Eq + Hash + Send + Sync + 'static + SlotType,
        U: Eq + Hash + Send + Sync + 'static + Clone + Debug + Default,
    > Bundle for GameFeaturesSystemBundle<S, E, I, K, ST, U>
{
    fn systems() -> Vec<System> {
        vec![
            apply_effector_system::<S, E>.system(),
            exec_skill_system::<K, E, S, I>.system(),
            remove_outdated_effector_system::<E>.system(),
            skill_cooldown_system::<K>.system(),
            trigger_passive_skill_system::<K, E, S, I, ST, U>.system(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    pub enum Stats {}
    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    pub enum Effectors {}
    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    pub enum Skills {}
    #[derive(Eq, PartialEq, Hash, Clone, Debug)]
    pub enum Items {}

    #[test]
    fn can_generate_systems() {
        let _ = GameFeaturesSystemBundle::<Stats, Effectors, Skills, Items>::systems();
    }
}
