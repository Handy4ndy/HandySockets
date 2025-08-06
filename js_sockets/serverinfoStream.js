import { Client } from 'xrpl';

/*
Server info stream monitors server information for each validated ledger.
We subscribe to ledger stream and request server_info for each new validated ledger.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/server-info-methods/server_info/
*/

// Connect to XRPL and stream server info
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for new validated ledgers
    client.on('ledgerClosed', async (ledger) => {
        console.log(`Ledger ${ledger.ledger_index} validated, requesting server info...`);
        
        try {
            // Request server info
            const response = await client.request({
                command: 'server_info'
            });
            
            const serverInfo = response.result || response;
            
            console.log('Server Info:');
            console.log(JSON.stringify(serverInfo, null, 2));
            console.log('---');
        } catch (error) {
            console.error('Error:', error.message);
        }
    });

    // Subscribe to ledger stream
    await client.request({
        command: 'subscribe',
        streams: ['ledger']
    });

    console.log('Listening for server info updates...');
}

main().catch(console.error);