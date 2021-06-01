use crate::*;

/// Modifies the stats of entities depending on the effectors applied through them
/// (using the `EffectorSet` component.)
pub fn apply_effector_system<K: Hash + Eq, E: Hash + Eq>(
    effector_defs: &EffectorDefinitions<K, E>,
    effectors: &Components<EffectorSet<E>>,
    time: &Time,
    stats: &mut Components<StatSet<K>>,
) -> SystemResult {
    for (stat, effector) in join!(&mut stats && &effectors) {
        let stat = stat.unwrap();
        let effector = effector.unwrap();
        effector.apply_to(effector_defs, stat, time.delta_time().as_secs_f32());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    enum Stats {
        S,
    }
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    enum Effectors {
        E,
    }
    #[test]
    fn apply_effector() {
        let mut world = World::default();
        let mut sys = apply_effector_system::<Stats, Effectors>.system();
        sys.initialize(&mut world);

        world.get_mut_or_default::<EffectorDefinitions<Stats, Effectors>>()
            .defs.insert(Effectors::E, EffectorDefinition::new(Effectors::E, None, vec![(Stats::S, EffectorType::AdditiveMultiplier(2.0))]));

        let entity = world.get_mut_or_default::<Entities>().create();
        world.get_mut_or_default::<Components<EffectorSet<Effectors>>>().insert(entity, EffectorSet::new(vec![EffectorInstance::new(Effectors::E, None)]));
        let mut statset = std::collections::HashMap::new();
        statset.insert(Stats::S, StatInstance::new(Stats::S, 1.0));
        world.get_mut_or_default::<Components<StatSet<Stats>>>().insert(entity, StatSet::new(statset));

        sys.run(&mut world).unwrap();
        assert_eq!(world.get_mut_or_default::<Components<StatSet<Stats>>>().get(entity).unwrap().stats.get(&Stats::S).unwrap().value, 1.0);
        assert_eq!(world.get_mut_or_default::<Components<StatSet<Stats>>>().get(entity).unwrap().stats.get(&Stats::S).unwrap().value_with_effectors, 2.0);
    }
}
