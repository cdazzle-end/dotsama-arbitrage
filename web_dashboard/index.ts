// const express = require('express');
import express from 'express';
// const sqlite3 = require('sqlite3');
import * as sqlite3 from 'sqlite3';
// const cors = require('cors');
import cors from 'cors';
import ws, { WebSocket } from 'ws';
import http from 'http';

import * as fs from 'fs';
import * as path from 'path';


const app = express();
const port = 3000;
export const server = http.createServer(app);
export const wss = new ws.Server({ server });

// const wsClient: WSWebSocket = new ws('ws://localhost:4000');
// wsClient.on('open', () => {
//     console.log('Connected to run_sql WebSocket server');
// });

// wsClient.on('message', (message:string) => {
//     console.log('Received message:', message);
//     if (message === 'NEW_TABLE_CREATED') {
//         // Fetch the latest table or update the UI
//     }
// });

app.use(cors());  // Enable CORS for all routes


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

        // Send "TABLE_SUCCESS" to all connected clients(sql and web dashboard)
        wss.clients.forEach(client => {
            if (client.readyState === WebSocket.OPEN) {
                client.send("TABLE_SUCCESS");
            }
        });
    });

    // existing code...
});



// server.listen(3000, () => {
//     console.log('Server is listening on port 3000');
// });

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




// Endpoint to get a list of all .db files in the /dbs/ folder
app.get('/databases', (req: any, res: any) => {
    console.log("Received request for /databases");
    const dbsPath = path.join(__dirname, '..', 'sql_database', 'dbs');
    // const pathToDbs = '..\\sql_database\\dbs\\'
    fs.readdir(dbsPath, (err, files) => {
        if (err) {
            res.status(500).json({ error: err.message });
            return;
        }
        const dbFiles = files.filter(file => path.extname(file) === '.db');
        res.json(dbFiles);
    });
});

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

// Additional endpoints (e.g., fetch table contents) can be added here

// app.listen(port, () => {
//     console.log(`Server running at http://localhost:${port}/`);
// });

async function main() {

}

main()

// module.exports.wss = wss;  // Export the WebSocket Server instance
