use std::fs::File;
use std::io::{ BufReader, BufRead, Result };
use serde_json::{ Value, json };
use chrono::prelude::*;

// Struct to represent a credit card
struct Card {
    number: String,
    exp_date: String,
    cvv: String,
}

fn main() -> Result<()> {
    // Open the JSON file and read the contents into a string
    let file = File::open("credit_cards.json")?;
    let reader = BufReader::new(file);
    let contents: String = reader
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Parse the JSON string into a Vec<Card> of credit card objects
    let cards = parse_json(&contents);

    // Validate each credit card and create a new Vec<Value> of validated credit cards
    let validated_cards: Vec<Value> = cards.iter().filter_map(|card| {
        let is_valid = validate_card(&card.number, &card.exp_date, &card.cvv);
        let brand = get_brand(&card.number);
        if is_valid {
            Some(json!({
                "card_number": card.number,
                "exp_date": card.exp_date,
                "cvv": card.cvv,
                "brand": brand,
                "valid": is_valid
            }))
        } else {
            None
        }
    }).collect();

    // Write the validated credit cards to a new JSON file
    let result: Value = json!({
        "validated_credit_cards": validated_cards,
    });
    let result_string: String = serde_json::to_string_pretty(&result)?;
    std::fs::write("validated_credit_cards.json", result_string)?;

    Ok(())
}

// Parses the JSON string and returns a vector of Card objects
fn parse_json(json_str: &str) -> Vec<Card> {
    let cards: Vec<Card> = match serde_json::from_str::<Value>(json_str) {
        Ok(json) => {            
            let card_node: &Vec<Value> = json.get("credit_cards").unwrap().as_array().unwrap();
            card_node.iter().filter_map(|card| {                
                let card_obj: &serde_json::Map<String, Value> = card.as_object().unwrap();
                Some(Card {
                    number: card_obj.get("CreditCardNumber").unwrap().as_str().unwrap().to_string(),
                    exp_date: card_obj.get("ExpDate").unwrap().as_str().unwrap().to_string(),
                    cvv: card_obj.get("CVV").unwrap().as_str().unwrap().to_string()
                })               
            }).collect()
        },
        Err(_) => Vec::new(),
    };
    cards
}

// Function to validate a credit card number, expiration date, and CVV code
fn validate_card(card_number: &str, expiration_date: &str, cvv: &str) -> bool {
    // Validate the credit card number using the Luhn algorithm
    let mut digits: Vec<u32> = card_number
        .chars()
        .filter_map(|d| d.to_digit(10))
        .collect();
    if digits.len() > 16 || digits.len() < 15 {
        return false;
    }

    let check_digit: u32 = digits.pop().unwrap();   
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, &d)| {
            let d2: u32 = if i % 2 == 0 { d * 2 } else { d };
            sum + d2 / 10 + (d2 % 10)
        });
    if (sum * 9) % 10 != check_digit {
        return false;
    }

    // Validate the expiration date in MM/yy format
    let current_year: i32 = Local::now().year();
    let current_month: u32 = Local::now().month();
    let date_parts: Vec<&str> = expiration_date.split("/").collect();
    if date_parts.len() != 2 {
        return false;
    }
    let month: u32 = date_parts[0].parse::<u32>().unwrap();
    let year: String = format!("20{}", date_parts[1]); // Add 20 to the year to convert it to a 4-digit year
    if
        year.parse::<i32>().unwrap() < current_year ||
        (year.parse::<i32>().unwrap() == current_year && month < current_month)
    {
        return false;
    }

    // Validate the CVV
    if cvv.len() != 3 {
        return false;
    }

    // If the CC brand is Unknown return false
    if get_brand(card_number).eq("Unknown"){
        return false;
    }

    true
}

// Function to determine the brand of a credit card
fn get_brand(card_number: &str) -> &str {
    if card_number.starts_with("4") {
        "Visa"
    } else if card_number.starts_with("5") {
        "Mastercard"
    } else if card_number.starts_with("34") || card_number.starts_with("37") {
        "American Express"
    } else if card_number.starts_with("67") {
        "Maestro"
    } else if card_number.starts_with("65") || card_number.starts_with("64") {
        "Discover"
    } else {
        "Unknown"
    }
}
