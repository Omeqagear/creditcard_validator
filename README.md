## Credit Card Validation and JSON Write

This program reads credit card information from a JSON file and writes validated credit card information to another JSON file. The validation includes the Luhn algorithm for card numbers, verification of expiration dates, and checking that CVV codes are three digits. The program also determines the brand of the credit card using its number.

### Requirements

- Rust programming language
- `serde_json` library
- `chrono` library

### How to Use

1. Create a file named `credit_cards.json` in the root directory of the project.
2. Add credit card information to `credit_cards.json` in the following format:

   ```
   {
       "credit_cards": [
           {
               "CreditCardNumber": "[CARD_NUMBER]",
               "ExpDate": "[EXPIRATION_DATE]",
               "CVV": "[CVV_CODE]"
           },
           {
               "CreditCardNumber": "[CARD_NUMBER]",
               "ExpDate": "[EXPIRATION_DATE]",
               "CVV": "[CVV_CODE]"
           },
           ...
       ]
   }
   ```
   
   Replace `[CARD_NUMBER]` with the credit card number (no spaces or dashes), `[EXPIRATION_DATE]` with the expiration date in MM/yy format, and `[CVV_CODE]` with the three-digit CVV code on the back of the card.
3. Run `cargo run`.
4. The program will create a new file named `validated_credit_cards.json` in the root directory that contains the validated credit card information in the following format:

   ```
   {
       "validated_credit_cards": [
           {
               "card_number": "[CARD_NUMBER]",
               "exp_date": "[EXPIRATION_DATE]",
               "cvv": "[CVV_CODE]",
               "brand": "[CARD_BRAND]",
               "valid": true
           },
           {
               "card_number": "[CARD_NUMBER]",
               "exp_date": "[EXPIRATION_DATE]",
               "cvv": "[CVV_CODE]",
               \"brand": "[CARD_BRAND]",
               \"valid": true
           },
           ...
       ]
   }
   ```
   
   Replace `[CARD_NUMBER]` with the credit card number (with spaces every four digits), `[EXPIRATION_DATE]` with the expiration date in MM/yy format, `[CVV_CODE]` with the three-digit CVV code on the back of the card, and `[CARD_BRAND]` with the determined brand of the credit card. The `valid` field will be `true` if the credit card is valid and `false` otherwise.