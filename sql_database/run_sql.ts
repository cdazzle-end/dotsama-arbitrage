const fsPromise = require('fs').promises;
// const fs = require('fs');
import * as fs from 'fs';
// const path = require('path');
import * as path from 'path';
// const sqlite3 = require('sqlite3').verbose();
import * as sqlite3 from 'sqlite3';
import WebSocket from 'ws';
import {wss} from '../web_dashboard/index';

// Try run_sql.ts as server
// import ws, { Server } from 'ws';

// const wss = new Server({ port: 4000 });


async function getLatestFile(dirPath: any) {
    try {
        // Read the directory
        const files = await fsPromise.readdir(dirPath);

        // Get file details with stat
        const fileDetails = await Promise.all(files.map(async (file: string) => {
            const filePath = path.join(dirPath, file);
            const stat = await fsPromise.stat(filePath);
            return { file, mtime: stat.mtime };
        }));

        // Sort files by modification time in descending order
        fileDetails.sort((a, b) => b.mtime - a.mtime);

        // Return the most recently modified file
        return fileDetails[0].file;
    } catch (err) {
        console.error("Error fetching the latest file:", err);
        throw err;
    }
}

function getCurrentDate() {
    const date = new Date();  // Get the current date and time

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');  // Months are 0-indexed in JavaScript
    const day = String(date.getDate()).padStart(2, '0');

    return `${year}-${month}-${day}`;
}

async function main() {
    const folderPath = '\..\\arb_handler\\result_log_data\\' + getCurrentDate() + '\\';  // replace with your folder path
    // const dayFolder = '' + getCurrentDate() + '\\';
    const latestFile = await getLatestFile(folderPath);
    // console.log("Folder path:", folderPath);
    // console.log("Latest file:", latestFile);
    

    var jsonData = await fsPromise.readFile(folderPath + latestFile, 'utf8');
    jsonData = JSON.parse(jsonData);
    // console.log(jsonData);

    //Removing special characters from the file name
    const dbName = getCurrentDate().replace(/-/g, '_') + '.db';
    const newTable = latestFile.replace(/.json/g, '').replace(/-/g, '_');

    // console.log(dbName);
    // console.log(newTable);
    const dbFolderPath = '\.\\dbs\\'
    // Check if the directory exists
    if (!fs.existsSync(dbFolderPath)) {
        // If not, create the directory
        fs.mkdirSync(dbFolderPath, { recursive: true });
    }

    // Create a new database, or connect to existing one
    const db = new sqlite3.Database(dbFolderPath + dbName, (err: any) => {
        if (err) {
            console.error('Error opening database:', err.message);
        } else {
            console.log('Connected to the SQLite database.');
        }
    });

    db.serialize(() => {
        db.run("CREATE TABLE IF NOT EXISTS " + newTable + "(node_key TEXT, asset_name TEXT, path_value REAL)");

        // Prepare an INSERT statement
        const stmt = db.prepare("INSERT INTO " + newTable + "(node_key, asset_name, path_value) VALUES (?, ?, ?)");

        // Iterate through each JSON object and insert it as a row in the table
        jsonData.forEach((item: any) => {
            stmt.run(item.node_key, item.asset_name, item.path_value);
        });

        stmt.finalize();

        console.log(`database:${dbName};table:${newTable}`);

        
        
 
    });
    db.close();

    // const ws = new WebSocket('ws://localhost:3000');
    // ws.on('error', console.error);
    // ws.on('open', function open() {
    //     ws.send('NEW_TABLE_CREATED');
    // });

    // ws.on('message', function message(data) {
    //     const message = data.toString();
    //     // console.log('received: %s', data.toString());
    //     if (message == "TABLE_SUCCESS") {
    //         // console.log("Closing database and websocket connection")
    //         // db.close();
    //         ws.close();
    //         // console.log("Closed database connection")

    //     }
    // });
    // console.log("Closing database connection")

}


main()

