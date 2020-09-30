use std::io;
use std::process;
use std::{thread, time};

struct Product {
    index: u32,
    product_name: String,
    price: u32,
}

fn make_selection(message: String) -> bool {
    eprint!("{} [y/n] ", message);
    loop {
        let mut user_raw_input = String::new();
        io::stdin()
            .read_line(&mut user_raw_input)
            .expect("Failed to read from stdin");
        match user_raw_input.trim().as_ref() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => {
                eprint!("Please answer Yes(y) or no (n) ");
            }
        }
    }
}

fn validate_coin(coin: u32) -> bool {
    match coin {
        1 | 5 | 10 => true,
        _ => false,
    }
}

fn validate_index(products: &Vec<Product>, index: u32) -> bool {
    for product in products.clone() {
        if product.index == index {
            return true;
        }
    }
    return false;
}

fn print_products(products: &Vec<Product>) {
    println!("\n========= | Vendig Machine |=========\n");
    println!("Index\tProduct\t\tPrice");
    println!("-------------------------------------");
    for product in products.clone() {
        println!(
            "{}\t{}\t\t{}:-",
            product.index, product.product_name, product.price
        );
    }
    println!("\n=====================================\n");
}

fn main() {
    let continue_purchase = true;

    let mut machine_products: Vec<Product> = Vec::new();
    machine_products.push(Product {
        index: 0,
        product_name: String::from("Cola"),
        price: 10,
    });
    machine_products.push(Product {
        index: 1,
        product_name: String::from("Redbull"),
        price: 25,
    });

    print_products(&machine_products);

    let mut coins = 0;
    while continue_purchase {
        let mut user_raw_input = String::new();
        eprintln!("Insert coins into machine (current balance {}:-)", coins);
        eprint!("You may choose between 1,5 and 10 kr coins: ");
        io::stdin()
            .read_line(&mut user_raw_input)
            .expect("Failed to read from stdin");

        let coin = match user_raw_input.trim().parse::<u32>() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("That is not a coint");
                0
            }
        };

        if coin > 0 && validate_coin(coin) {
            coins += coin;

            println!("You have now: {}", coins);

            user_raw_input = String::new();
            if make_selection(String::from("You want to make a selection?")) {
                print_products(&machine_products);
                eprint!("Pleace select from index: ");
                io::stdin()
                    .read_line(&mut user_raw_input)
                    .expect("Failed to read from stdin");

                let index = match user_raw_input.trim().parse::<i32>() {
                    Ok(i) => i,
                    Err(_) => {
                        eprintln!("That is not an index");
                        -1
                    }
                };

                if index >= 0 {
                    if validate_index(&machine_products, index as u32) {
                        for product in &machine_products {
                            if product.index == (index as u32) {
                                if product.price <= coins {
                                    for _i in 0..3 {
                                        eprintln!("Bonk...");
                                        thread::sleep(time::Duration::from_millis(1000));
                                    }
                                    eprintln!("Enjoy your {} sir", product.product_name);
                                    process::exit(0x00);
                                } else {
                                    eprintln!("You dont have enough money sir, please insert more to purchase {}", product.product_name);
                                }
                            }
                        }
                    } else {
                        eprintln!("{} is an invalid index", index);
                    }
                }
            }
        } else {
            eprintln!("Invalid coin, give me something else");
        }
        eprintln!("");
    }
}
