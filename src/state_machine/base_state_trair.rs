use super::state_manager_trait::StateManagerTrait;

pub trait BaseStateTrait {
    fn enter_state(&self, state_manage: Box<&dyn StateManagerTrait>);
    fn update_state(&self, state_manage: Box<&dyn StateManagerTrait>);
    fn on_collision_enter(&self, state_manage: Box<&dyn StateManagerTrait>);
}