import { Client } from 'xrpl';

/*
Account stream monitors a specific account for validated transactions.
Sends transaction messages whenever a transaction affects the monitored account.
Subscribed to the RLUSD account to receive updates.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#accounts
*/

// Connect to XRPL and stream account transactions
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for transactions affecting the monitored account
    client.on('transaction', (transaction) => {
        console.log('New Transaction:');
        console.log(JSON.stringify(transaction, null, 2));
        console.log('---');
    });

    // Subscribe to account stream
    await client.request({
        command: 'subscribe',
        accounts: ['rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De']
    });

    console.log('Listening for transactions on account: rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De');
}

main().catch(console.error);