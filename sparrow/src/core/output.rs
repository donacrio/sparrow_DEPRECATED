use crate::core::egg::Egg;

/// Output send from the engine through the output consumer.
pub type EngineOutput = Message<Option<Egg>>;
