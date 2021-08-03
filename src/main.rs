use std::time::Instant;

use loot_table::LootTable;
mod bank;
mod loot_table;

fn main() {
    let now = Instant::now();

    let mut table = LootTable {
        total_weight: 0,
        items: Vec::new(),
    };

    table
        .add(123, 1)
        .add(312, 3)
        .add(33, 3)
        .add(234, 3)
        .add(125, 3)
        .add(35, 3);

    let result = table.roll_many(500_000_000);
    println!("{}ms", now.elapsed().as_millis());
    println!("{}", result);
}
