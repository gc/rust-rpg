use crate::bank::Bank;
use rand::Rng;
use rayon::prelude::*;

pub struct LootTableItem {
    pub id: i32,
    pub weight: i32,
}

pub struct LootTable {
    pub total_weight: i32,
    pub items: Vec<LootTableItem>,
}

impl LootTable {
    pub fn add(&mut self, id: i32, weight: i32) -> &mut LootTable {
        self.total_weight += weight;
        self.items.push(LootTableItem { id, weight });
        self
    }
    pub fn roll_many(&self, quantity: i32) -> Bank {
        let mut bank = Bank::new();

        let rolled_items = (0..quantity)
            .into_par_iter()
            .filter_map(|_| self.roll_single())
            .collect::<Vec<_>>();

        for rolled_item in rolled_items {
            bank.add(rolled_item.id, 1);
        }

        bank
    }
    pub fn roll_single(&self) -> Option<&LootTableItem> {
        let random_weight = rand::thread_rng().gen_range(1..=self.total_weight);

        let mut weight = 0;
        for i in self.items.iter() {
            if random_weight <= weight + i.weight {
                return Some(i);
            }
            weight += i.weight;
        }
        None
    }
}
