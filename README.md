# stripe-checkout-sample-rust
An implementation, in Rust, of the [Stripe Checkout Simple Subscription sample](https://github.com/stripe-samples/checkout-single-subscription)


## How to install

1. Login to stripe (with your key and password) and get a pairing code: `stripe login`
2. Create a basic and a premium product, either using the CLI or the dashboard
  Using the CLI:
```
stripe products create --name="Basic" --description="Basic plan"
stripe products create --name="Premium" --description="Premium plan"
```
3. Create a basic and premium price, either using the CLI or the dashboard. If using the CLI:
```
stripe prices create --product=prod_XYZ --unit-amount=1200 --currency=usd -d"recurring[interval]=month"
```
   For each product call the command line above but replace prod_XYZ with the product ids you got on 2 and change the price and currency accordingly.

4. Edit change the file `.env.example` name into `.env` and change the test keys, prices' ids and webhook id.

5. Call `cargo run` and access `http://localhost:4242`. Because the sample doesn't run on SSL, I recomend using MS Edge. For security, the settings on other browsers will block some components.
