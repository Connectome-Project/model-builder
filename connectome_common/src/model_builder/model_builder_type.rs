pub enum ModelBuilderType {
    Builder,
    Stereotype,
    StereotypeEvolution,
    Adoption,
    GarbageCollector,
}

impl Default for ModelBuilderType {
    fn default() -> Self {
        Self::Builder
    }
}
