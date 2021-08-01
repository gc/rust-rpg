use loot_table_module::LootTable;
mod bank_module;
mod loot_table_module;

fn main() {
    let mut table = LootTable {
        total_weight: 0,
        items: Vec::new(),
    };

    // Add 2 IDs to the table
    table.add(123, 1).add(312, 3);

    let result = table.roll_many(100_000);

    // Log how many of each ID were rolled
    for (id, qty) in result.map.iter() {
        println!("Item ID {} had {} drops", id, qty);
    }
}
