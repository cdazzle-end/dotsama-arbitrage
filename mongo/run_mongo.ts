import dotenv from 'dotenv';
dotenv.config();
import { MongoClient, ServerApiVersion } from 'mongodb';
import axios from 'axios';
// const { MongoClient, ServerApiVersion } = require('mongodb');
const password = process.env.DB_PASSWORD
const uri = `mongodb+srv://dazzlec123:${password}@cluster0.i2n4fzh.mongodb.net/?retryWrites=true&w=majority`;

// Create a MongoClient with a MongoClientOptions object to set the Stable API version
const client = new MongoClient(uri, {
    serverApi: {
        version: ServerApiVersion.v1,
        strict: true,
        deprecationErrors: true,
    }
});

async function run() {
    try {
        // Connect the client to the server	(optional starting in v4.7)
        await client.connect();
        // Send a ping to confirm a successful connection
        await client.db("admin").command({ ping: 1 });
        console.log("Pinged your deployment. You successfully connected to MongoDB!");
    } finally {
        // Ensures that the client will close when you finish/error
        await client.close();
    }
}
// run().catch(console.dir);

async function testUpdate() {
    try {
        const response = await axios.post("https://arb-server-a7fa597e65ca.herokuapp.com/notify-update", {
            message: 'Data uploaded to mongo',
        });
        console.log('Heroku server notified', response.data);
    } catch (error) {
        console.error('Error notifying heroku server', error);
    }
}
testUpdate().catch(console.dir);