use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: u64,
    quantity: f32,
    unit: String
}

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>
}

fn main() -> Result<(), io::Error> {
    let input_path = env::args().nth(1).unwrap();
    let output_path = env::args().nth(2).unwrap();

    let mut sales_and_products: SalesAndProducts = {
        let data = fs::read_to_string(input_path).expect("error reading file");

        serde_json::from_str(&data).unwrap()
    };

    sales_and_products.sales[1].quantity += 1.5;

    fs::write(output_path, serde_json::to_string_pretty(&sales_and_products).unwrap())?;

    Ok(())
}
