use std::thread;

use rand::Rng;

use crate::bank_module::Bank;

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
    pub fn roll_many(&'static mut self, quantity: i32) -> Bank {
        let quantity_per_thread = quantity / num_cpus::get() as i32;
        // let leftover = quantity % quantity_per_thread;

        let mut bank = Bank::new();
        // bank.add

        for _ in 1..=quantity_per_thread {
            let handle = thread::spawn(move || {
                for _ in 1..quantity_per_thread {
                    let loot = &self.roll_single();
                    if let Some(rolled_item) = loot {
                        bank.add(rolled_item.id, 1);
                    }
                }
            });
            handle.join().unwrap();
        }

        bank
    }
    pub fn roll_single(&mut self) -> Option<&LootTableItem> {
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
