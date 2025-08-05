import { Client } from 'xrpl';

/*
Book changes stream monitors order book changes across all trading pairs.
Since book_changes stream requires ledger index, we subscribe to ledger stream
and request book changes for each new validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#book-changes-stream
*/

// Connect to XRPL and stream book changes
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for new validated ledgers
    client.on('ledgerClosed', async (ledger) => {
        console.log(`Ledger ${ledger.ledger_index} validated, checking for book changes...`);
        
        try {
            // Request book changes for this ledger
            const response = await client.request({
                command: 'book_changes',
                ledger_index: ledger.ledger_index
            });
            
            const bookChanges = response.result || response;
            
            if (bookChanges.changes && bookChanges.changes.length > 0) {
                console.log('Order Book Changes:');
                console.log(JSON.stringify(bookChanges, null, 2));
                console.log('---');
            }
        } catch (error) {
            console.error('Error:', error.message);
        }
    });

    // Subscribe to ledger stream
    await client.request({
        command: 'subscribe',
        streams: ['ledger']
    });

    console.log('Listening for order book changes...');
}

main().catch(console.error);