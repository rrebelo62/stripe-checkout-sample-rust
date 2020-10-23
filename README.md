# stripe sample checkout-single-subscription, Rust version


Usage
----

1. Login to stripe and get a pairing code: stripe login

2. Create a basic and a premium product, either using the CLI or the dashboard
	Using CLI:	stripe products create --name="Basic" --description="Basic plan"
				stripe products create --name="Premium" --description="Premium plan"

3. Create a basic and premium price, either using the CLI or the dashboard
	Using CLI: ..\stripe prices create --product=prod_XYZ --unit-amount=1200 --currency=usd -d"recurring[interval]=month"
	For each product call the command line above but replace prod_XYZ with the product ids you got on 2 and change the price and currency accordingly.

4. Edit .env and change the test keys, prices' ids and webhook id.



