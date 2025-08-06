import { Client } from 'xrpl';

/*
Consensus stream monitors consensus phase changes during the XRPL consensus process.
Sends messages when the server changes phase in the consensus cycle.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#consensus-stream
*/

// Connect to XRPL and stream consensus phase changes
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for consensus phase changes
    client.on('consensusPhase', (consensus) => {
        console.log('Consensus Phase Change:');
        console.log(JSON.stringify(consensus, null, 2));
        console.log('---');
    });

    // Subscribe to consensus stream
    await client.request({
        command: 'subscribe',
        streams: ['consensus']
    });

    console.log('Listening for consensus phase changes...');
}

main().catch(console.error);