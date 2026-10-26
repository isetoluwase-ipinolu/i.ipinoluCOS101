struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u64) -> u64 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    let quantity_bought = 3;

    let hp_cost = hp.calculate_cost(quantity_bought);
    let ibm_cost = ibm.calculate_cost(quantity_bought);
    let toshiba_cost = toshiba.calculate_cost(quantity_bought);
    let dell_cost = dell.calculate_cost(quantity_bought);
    let total_cost = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("The cost of 3 {} laptops is N{}",hp.brand, hp_cost);
    println!("The cost of 3 {} laptops is N{}",ibm.brand, ibm_cost);
    println!("The cost of 3 {} laptops is N{}",toshiba.brand, toshiba_cost);
    println!("The cost of 3 {} laptops is N{}",dell.brand, dell_cost);
    println!("The total cost of laptops across the brands is N{}", total_cost);
}
