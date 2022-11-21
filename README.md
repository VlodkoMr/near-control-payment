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
- DB_CONNECTION: database prisma connection.

4. Update server/index.js script to handle new transactions and save in your database structure.
5. Upload server script into your server.
