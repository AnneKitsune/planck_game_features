use crate::*;

/// An event where a skill is activated by a specific entity.
#[derive(Debug, Clone)]
pub struct SkillTriggerEvent<K>(pub Entity, pub K);
