use std::io;

struct VendingMachine {
    balance: u32,
    items: Vec<Item>,
}

impl VendingMachine {
    fn new() -> Self {
        VendingMachine {
            balance: 0,
            items: vec![
                Item {
                    name: "Coca Cola".to_string(),
                    price: 150,
                    quantity: 10,
                },
                Item {
                    name: "Sprite".to_string(),
                    price: 120,
                    quantity: 5,
                },
                Item {
                    name: "Water".to_string(),
                    price: 100,
                    quantity: 7,
                },
            ],
        }
    }

    fn display_items(&self) {
        println!("Items Available:");
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {} ({}) - {} cents", i + 1, item.name, item.quantity, item.price);
        }
    }

    fn select_item(&mut self, item_index: usize) -> Result<(), String> {
        let item = match self.items.get_mut(item_index - 1) {
            Some(item) => item,
            None => return Err("Invalid item selection.".to_string()),
        };

        if item.price > self.balance {
            return Err("Not enough balance to buy item.".to_string());
        }

        if item.quantity == 0 {
            return Err("Item out of stock.".to_string());
        }

        self.balance -= item.price;
        item.quantity -= 1;

        println!("Enjoy your {}!", item.name);

        Ok(())
    }

    fn insert_money(&mut self, amount: u32) {
        self.balance += amount;
        println!("Balance: {} cents", self.balance);
    }
}

struct Item {
    name: String,
    price: u32,
    quantity: u32,
}

fn main() {
    let mut vending_machine = VendingMachine::new();

    println!("Welcome to the Vending Machine!");

    loop {
        println!("");

        vending_machine.display_items();

        println!("");

        println!("Please insert coins (in cents):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let amount: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        vending_machine.insert_money(amount);

        println!("Enter item number (or 0 to exit):");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let item_index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if item_index == 0 {
            break;
        }

        match vending_machine.select_item(item_index) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        }
    }

    println!("Thank you for using the Vending Machine!");
}
