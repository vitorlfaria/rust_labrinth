use crate::state_machine::{state_manager_trait::StateManagerTrait, base_state_trair::BaseStateTrait};
use super::Enemy;

pub struct EnemyStateManager {
    pub current_state: Box<dyn BaseStateTrait>,
    
}

impl StateManagerTrait for Enemy {
    fn start(&mut self) {
        println!("Enemy start");
    }

    fn update(&mut self) {
        println!("Enemy update");
    }
}