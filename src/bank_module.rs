use std::collections::HashMap;

pub struct Bank {
    pub map: HashMap<i32, i32>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            map: HashMap::new(),
        }
    }
    pub fn add(&mut self, id: i32, quantity: i32) -> &mut Bank {
        let current = self.map.get_mut(&id);
        if let Some(current_quantity) = current {
            *current_quantity = *current_quantity + quantity
        } else {
            self.map.insert(id, quantity);
        }
        self
    }
}

// Implement `Display` for `MinMax`.
// impl fmt::Display for Bank {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let list = self
//             .map
//             .iter()
//             .map(|(id, qty)| format!("{}x {}", qty, id))
//             .collect();
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", list)
//     }
// }
