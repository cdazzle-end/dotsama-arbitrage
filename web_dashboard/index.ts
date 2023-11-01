// const express = require('express');
import express from 'express';
import { MongoClient } from 'mongodb'
import * as sqlite3 from 'sqlite3';

// const cors = require('cors');
import cors from 'cors';
import ws, { WebSocket } from 'ws';
import http from 'http';
import dotenv from 'dotenv';
import * as fs from 'fs';
import * as path from 'path';


const dotenvPath = path.join(__dirname, "/.env")
dotenv.config({ path: dotenvPath });
const app = express();
const port = 3000;
app.use(cors());  // Enable CORS for all routes
// app.use(cors({
//     origin: 'http://localhost:8080'
// }));

export const server = http.createServer(app);
export const wss = new ws.Server({ server });

// app.use(cors());  // Enable CORS for all routes


wss.on('error', (error) => {
    console.error('WebSocket Server Error:', error);
});

wss.on('connection', (ws) => {
    console.log('Client connected');

    ws.send('Test message from server');  // Send a test message

    ws.on('error', (error) => {
        console.error('Client WebSocket Error:', error);
    });

    ws.on('message', (message) => {
        const translatedMessage = message.toString().toUpperCase();
        console.log('Received message:', translatedMessage);

        wss.clients.forEach(client => {
            if (client.readyState === WebSocket.OPEN) {
                client.send("TABLE_SUCCESS");
            }
        });
    });

});

app.get('/tables', (req, res) => {
    // Check if database parameter is provided
    if (!req.query.database) {
        res.status(400).json({ error: "Database parameter is required" });
        return;
    }

    const dbName = req.query.database as string;
    const pathToDb = path.join(__dirname, '..', 'sql_database', 'dbs', dbName);

    // Check if the file exists before attempting to open it
    if (!fs.existsSync(pathToDb)) {
        res.status(404).json({ error: "Database not found" });
        return;
    }

    const db = new sqlite3.Database(pathToDb);
    db.all("SELECT name FROM sqlite_master WHERE type='table';", [], (err, tables) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        res.json(tables);
    });
    db.close();
});

app.get('/tables-with-last-value', (req, res) => {
    if (!req.query.database) {
        res.status(400).json({ error: "Database parameter is required" });
        return;
    }

    const dbName = req.query.database as string;
    const pathToDb = path.join(__dirname, '..', 'sql_database', 'dbs', dbName);
    const db = new sqlite3.Database(pathToDb);

    db.all("SELECT name FROM sqlite_master WHERE type='table';", [], (err, tables: any) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }

        // Fetch the path_value of the last row for each table
        let completedQueries = 0;
        tables.forEach((table: any) => {
            db.get(`SELECT path_value FROM ${table.name} ORDER BY ROWID DESC LIMIT 1`, [], (err, row: any) => {
                if (row) table.lastPathValue = row.path_value;
                completedQueries++;
                if (completedQueries === tables.length) {
                    res.json(tables);
                    db.close();
                }
            });
        });
    });
});

app.get('/latest-table', (req, res) => {
    console.log("Fetching latest table")
    const dbsPath = path.join(__dirname, '..', 'sql_database', 'dbs');
    fs.readdir(dbsPath, (err, files) => {
        if (err) {
            res.status(500).json({ error: err.message });
            console.log("ERROR 1");
            return;
        }
        const latestDb = files.sort().reverse()[0];
        const pathToLatestDb = path.join(dbsPath, latestDb);
        const db = new sqlite3.Database(pathToLatestDb);
        // var latestTable: any;
        db.all("SELECT name FROM sqlite_master WHERE type='table';", [], (err, tables) => {
            if (err) {
                res.status(500).json({ error: err.message });
                console.log("ERROR 2");
                return;
            }
            const latestTable:any = tables.sort().reverse()[0];
            // res.json({ database: latestDb, table: latestTable });
            db.all("SELECT * FROM " + latestTable.name, [], (err, rows) => {
                if (err) {
                    res.status(500).json({ error: err.message });
                    console.log("ERROR 3");
                    return;
                }
                res.json({ database: latestDb, table: latestTable, contents: rows });
                console.log("Table fetched successfully");
                db.close();
            });
        });
        
    });
});

app.get('/databases', (req: any, res: any) => {
    // console.log("Received request for /databases");
    const dbsPath = path.join(__dirname, '..', 'sql_database', 'dbs');
    const pathToDbs = '..\\sql_database\\dbs\\'
    fs.readdir(dbsPath, (err, files) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        const dbFiles = files.filter(file => path.extname(file) === '.db');
        res.json(dbFiles);
    });
});

app.get('/getMongodbData', async (req, res) => {
    console.log("Fetching MONDODB data...")
    if (!process.env.MONGODB_URI) {
        res.status(500).json({ error: "MONGODB_URI environment variable is not set" });
        throw new Error('MONGODB_URI environment variable is not defined!');
        // return;
    }
    // const client = new MongoClient(process.env.MONGODB_URI);
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    // const db = client.db(dbName);
    // const data = db.collection(collectionName).find({}).toArray();
    try {
        const client = new MongoClient(process.env.MONGODB_URI);
        await client.connect();  // Await connection

        const db = client.db(dbName);
        const data = await db.collection(collectionName).find({}).toArray();  // Await data fetching

        console.log("Mondodb data successfully fetched")

        res.status(200).json(data);  // Send data as response

        await client.close();  // Close the connection
    } catch (error) {
        console.log("Error fetching mongodb")
        console.log("ERROR: " + error);
        res.status(500).json({ error: error });
    }
})


app.get('/getMongoLatest', async (req, res) => {
    const uri = process.env.MONGODB_URI;
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    if (!uri) {
        console.error('MONGODB_URI is not defined in your environment.');
        return;
    }

    const client = new MongoClient(uri);

    try {
        await client.connect();

        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const latestDateEntry = await collection.find().sort({ 'date': -1 }).limit(1).toArray();
        const latestDate = latestDateEntry[0].date;

        const latestRecord = await collection.find({ 'date': latestDate }).sort({ 'time': -1 }).limit(1).toArray();

        res.status(200).json(latestRecord[0]);  // Send data as response

        await client.close();  // Close the connection

    } catch (error) {
        console.error('An error occurred while retrieving the most recent record:', error);
    } finally {
        // Close the connection to the MongoDB cluster
        await client.close();
    }
})
app.get('/getMongoDates', async(req, res) => {
    const uri = process.env.MONGODB_URI;
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    if (!uri) {
        console.error('MONGODB_URI is not defined in your environment.');
        return;
    }

    const client = new MongoClient(uri);

    try {
        await client.connect();

        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const uniqueDates = await collection.distinct('date'); // Make sure 'date' is the field name in your documents

        console.log(uniqueDates);
        res.status(200).json(uniqueDates);
        await client.close()
    } catch (error) {
        console.error('An error occurred while retrieving unique dates:', error);
    } finally {
        await client.close();
    }
})
app.get('/table-content', (req, res) => {
    if (!req.query.database || !req.query.table) {
        res.status(400).json({ error: "Database and table parameters are required" });
        return;
    }
    console.log("recieved request for table data")

    const dbName = req.query.database as string;
    const tableName = req.query.table;
    const pathToDb = path.join(__dirname, '..', 'sql_database', 'dbs', dbName);

    const db = new sqlite3.Database(pathToDb);
    db.all(`SELECT * FROM ${tableName}`, [], (err, rows) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        // console.log(rows)
        res.json(rows);
    });
    db.close();
});
app.get('/getMongoDate', async (req, res) => {
    const uri = process.env.MONGODB_URI;
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    const date = req.query.date as string;

    if (!uri) {
        console.error('MONGODB_URI is not defined in your environment.');
        return;
    }
    const client = new MongoClient(uri);
    try {
        await client.connect();
        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const recordsForDate = await collection.find({ 'date': date }).toArray();

        console.log(recordsForDate);
        let timeStamps = recordsForDate.map((record) => {
            return record.time;
        })
        res.status(200).json(timeStamps);
        return recordsForDate; // Since limit is 1, we can take the first element of the array

    } catch (error) {
        console.error('An error occurred while retrieving the most recent record:', error);
    } finally {
        await client.close();
    }
});
app.get('/getMongoDateTime', async (req, res) => {
    const uri = process.env.MONGODB_URI;
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    const date = req.query.date as string;
    const time = req.query.time as string;

    if (!uri) {
        console.error('MONGODB_URI is not defined in your environment.');
        return;
    }
    const client = new MongoClient(uri);
    try {
        await client.connect();
        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const table = await collection.find({ 'date': date, 'time': time }).toArray();

        res.status(200).json(table);
        return table; // Since limit is 1, we can take the first element of the array

    } catch (error) {
        console.error('An error occurred while retrieving the most recent record:', error);
    } finally {
        await client.close();
    }
});
async function getRecordsForDate(date: string) {
    // Load environment variables
    const uri = process.env.MONGODB_URI;
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    if (!uri) {
        console.error('MONGODB_URI is not defined in your environment.');
        return;
    }

    const client = new MongoClient(uri);

    try {
        await client.connect();

        const database = client.db(dbName);
        const collection = database.collection(collectionName);

        const recordsForDate = await collection.find({ 'date': date }).toArray();
        return recordsForDate; // Since limit is 1, we can take the first element of the array
    } catch (error) {
        console.error('An error occurred while retrieving the most recent record:', error);
    } finally {
        await client.close();
    }
}
