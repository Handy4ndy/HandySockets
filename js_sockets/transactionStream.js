import { Client } from 'xrpl';

/*
Transaction stream monitors all validated transactions on the XRPL network.
Sends transaction messages whenever any transaction is included in a validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#transaction-streams
*/

// Connect to XRPL and stream all transactions
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for all validated transactions
    client.on('transaction', (transaction) => {
        console.log('New Transaction:');
        console.log(JSON.stringify(transaction, null, 2));
        console.log('---');
    });

    // Subscribe to transactions stream
    await client.request({
        command: 'subscribe',
        streams: ['transactions']
    });

    console.log('Listening for all validated transactions...');
}

main().catch(console.error);