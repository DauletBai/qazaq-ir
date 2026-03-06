use qazaq_ir::RootEntity;
use std::collections::HashMap;

/// The In-Memory State Machine for Orda Node.
/// Stores the global balance map corresponding to each immutable `RootEntity`.
#[derive(Default)]
pub struct State {
    balances: HashMap<RootEntity, u64>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieve the current balance for a deterministic entity.
    pub fn get_balance(&self, entity: &RootEntity) -> u64 {
        *self.balances.get(entity).unwrap_or(&0)
    }

    /// Add amount to the specified deterministic entity's balance.
    pub fn add_balance(&mut self, entity: RootEntity, amount: u64) {
        let current = self.get_balance(&entity);
        self.balances.insert(entity, current.saturating_add(amount));
    }

    /// Subtract amount from the specified deterministic entity's balance.
    /// Returns `Err` if there are insufficient funds.
    pub fn sub_balance(&mut self, entity: &RootEntity, amount: u64) -> Result<(), String> {
        let current = self.get_balance(entity);
        if current < amount {
            return Err(format!(
                "Insufficient funds: expected {}, got {}",
                amount, current
            ));
        }
        self.balances.insert(entity.clone(), current - amount);
        Ok(())
    }
}
