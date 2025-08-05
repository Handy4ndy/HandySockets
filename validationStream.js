import { Client } from 'xrpl';

/*
Validations stream monitors validation messages (votes) from XRPL validators.
Sends messages whenever the server receives a validation message during consensus.

https://xrpl.org/docs/references/http-websocket-apis/public-api-methods/subscription-methods/subscribe#validations-stream
*/

// Connect to XRPL and stream validation messages
const client = new Client('wss://xrplcluster.com');

async function main() {
    // Connect to XRPL
    await client.connect();
    console.log('Connected to XRPL');

    // Listen for validation messages
    client.on('validationReceived', (validation) => {
        console.log('New Validation:');
        console.log(JSON.stringify(validation, null, 2));
        console.log('---');
    });

    // Subscribe to validations stream
    await client.request({
        command: 'subscribe',
        streams: ['validations']
    });

    console.log('Listening for validation messages...');
}

main().catch(console.error);