import { Client } from 'xrpl';

/*
The ledger stream only sends ledgerClosed messages when the consensus process declares a new
validated ledger. The message identifies the ledger and provides some information about its
contents. 

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#ledger-stream
*/

// Connect to XRPL and stream validated ledgers
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for new validated ledgers
    client.on('ledgerClosed', (ledger) => {
        console.log('New Validated Ledger:');
        console.log(JSON.stringify(ledger, null, 2));
        console.log('---');
    });

    // Subscribe to ledger stream
    await client.request({
        command: 'subscribe',
        streams: ['ledger']
    });

    console.log('Listening for validated ledgers...');
}

main().catch(console.error);