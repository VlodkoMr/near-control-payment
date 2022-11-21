# NEAR Control Payments

Smart-contract and server-side script that you can use to sync
NEAR blockchain payments with your database.

## Install

1. Install requirements:

``` 
npm install
```

2. Deploy smart-contract:

``` 
npm run deploy 
```

3. Copy .env.sample into .env.local and/or .env.production files and set variables:

- NODE_ENV: testnet or mainnet environment.
- ACCOUNT_PRIVATE_KEY: Private key for deployed smart-contract (without ed25519:).
- DB_CONNECTION: database connection URL.

4. Update server/index.js script to handle new transactions and save in your database structure.
5. Upload server script and run.

## Contract Methods

#### Read all temporary order list

``` 
get_tmp_list()
```

Return list (array) of orders: Example: [{order_id: u32, payment: U128}]

#### Add new order to list

Method used to get payment from customer and store this information in temporary storage.

``` 
send_order_payment(order_list: Vec<TmpOrder>, to_account: AccountId)
```

order_list: JSON array of orders with fields. Example: [{order_id: u32, payment: U128}]
to_account: seller NEAR Wallet address to transfer tokens.

#### Cleanup orders

Executed by server script to cleanup orders and move payment information into database

``` 
cleanup_tmp_payments(orders: Vec<u32>)
```

orders - list of order id to cleanup.