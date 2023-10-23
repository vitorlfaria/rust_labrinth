use crate::state_machine::{base_state_trair::BaseStateTrait, state_manager_trait::StateManagerTrait};

pub struct EnemyPatrolState {
    
}

impl BaseStateTrait for EnemyPatrolState {
    fn enter_state(&self, state_manage: Box<&dyn StateManagerTrait>) {
        todo!()
    }

    fn update_state(&self, state_manage: Box<&dyn StateManagerTrait>) {
        todo!()
    }

    fn on_collision_enter(&self, state_manage: Box<&dyn StateManagerTrait>) {
        todo!()
    }
}