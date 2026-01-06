pub enum PhysicsMoment {
    MomentumTick,
    AccelerationTick,
    FrictionTick,
    GravityTick,
    Iteration {
        index: u8,
        sub_iteration: Option<u8>,
    },
}
