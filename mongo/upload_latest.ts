import path from "path";
import dotenv from 'dotenv';
import axios from "axios";
const dotenvPath = path.join(__dirname, "/.env")
dotenv.config({path: dotenvPath});
import fs from "fs";

const { MongoClient } = require("mongodb");
// const dotenv = require("dotenv");


const password = process.env.DB_PASSWORD
const uri = `mongodb+srv://dazzlec123:${password}@cluster0.i2n4fzh.mongodb.net/?retryWrites=true&w=majority`;


function getCurrentDate() {
    const date = new Date();  // Get the current date and time

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');  // Months are 0-indexed in JavaScript
    const day = String(date.getDate()).padStart(2, '0');

    return `${year}-${month}-${day}`;
}

// When running arb, after each run, this function will be called to update the database
async function uploadLatestRecord() {
    const [latestRecord, fileTime, date] = await getLatestFile();
    const formattedRecord = {
        date: date,
        time: fileTime,
        record: latestRecord
    }
    
    //If no record with date:time exists yet, insert into database
    let exists = await doesRecordExist(date, fileTime)
    if (!exists) {
        const client = new MongoClient(uri);
        const dbName = "DailyRecordsTest1";
        const collectionName = "Records";

        const db = client.db(dbName);
        const collection = db.collection(collectionName);
    
        try {
            await collection.insertOne(formattedRecord);
            console.log(`Success: ${date} : ${fileTime} added`)
        } catch (err) {
            console.error("Error inserting latest record into database")
            console.error(err)
        }
        await client.close()
        
        try {
            const response = await axios.post("https://arb-server-a7fa597e65ca.herokuapp.com/notify-update", {
                message: 'Data uploaded to mongo',
            });
            console.log('Heroku server notified', response.data);
        } catch (error) {
            console.error('Error notifying heroku server', error);
        }

    } else {
        console.log("Record already exists in database")
    }
    
   
}
uploadLatestRecord()

//Check database for record with same date and time
async function doesRecordExist(date: string, time: string) {
    // const[latestRecord, fileTime, date] = await getLatestFile();

    const client = new MongoClient(uri);
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";
    await client.connect()
    const db = client.db(dbName);
    const collection = db.collection(collectionName);

    let findRecord = await collection.findOne({ date: date, time: time });
    client.close()

    if (findRecord) {
        console.log(`Document already exists: ${date} : ${time}`);
        return true;
    } else {
        return false;
    }
    
}

async function getLatestFile() {
    const currentDate = getCurrentDate();
    const dayFolderPath = path.join(__dirname, '\..\\arb_handler\\result_log_data\\', currentDate)
    // const folderPath = '\..\\arb_handler\\result_log_data\\' + currentDate + '\\';  // replace with your folder path
    try {
        // Read the directory
        const files = await fs.promises.readdir(dayFolderPath);

        // Get file details with stat
        const fileDetails = await Promise.all(files.map(async (file: string) => {
            const filePath = path.join(dayFolderPath, file);
            const stat = await fs.promises.stat(filePath);
            return { file, mtime: stat.mtime };
        }));

        // Sort files by modification time in descending order
        fileDetails.sort((a: any, b: any) => b.mtime - a.mtime);

        // Return the most recently modified file
        // return fileDetails[0].file;
        let fileTime = fileDetails[0].file.split("_")[1].split(".")[0];
        const filePath = path.join(dayFolderPath, fileDetails[0].file);
        const latestFile = JSON.parse(fs.readFileSync(filePath, 'utf8'));
        // console.log(latestFile)
        return [latestFile,fileTime,currentDate];

    } catch (err) {
        console.error("Error fetching the latest file:", err);
        throw err;
    }
}
async function run2() {

    const client = new MongoClient(uri);

    const testFilePath = path.join(__dirname, "..//arb_handler//result_log_data//2023-10-29//Kusama_00-16-40.json");
    // const testFilePath = "C://Users//dazzl//CodingProjects//substrate//test2//arb-dot-2//arb_handler//result_log_data//2023-10-29//Kusama_00-16-40.json"

    const testFile = await fs.readFileSync(testFilePath, 'utf8');
    const records = JSON.parse(testFile);
    console.log(records)

    const date = "2023-10-29";
    const time = "00-16-40"

    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    // Create references to the database and collection in order to run
    // operations on them.
    const database = client.db(dbName);
    const collection = database.collection(collectionName);

    const modifiedRecord = {
        date: date,
        time: time,
        records: records
    }

    // console.log(modifiedRecord)
    try {
        const insertMany = await collection.insertOne(modifiedRecord);
        console.log(`${insertMany} documents successfully inserted.\n`);
    } catch (err) {
        console.error(`Something went wrong trying to insert the new documents: ${err}\n`);
    }
    await client.close();
}
async function queryDates() {
    const client = new MongoClient(uri);
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";

    const db = client.db(dbName);
    const collection = db.collection(collectionName);

    try {
        const distinctDates = await collection.distinct('date');
        return distinctDates;
    } catch (err) {
        console.error("Error retrieving distinct dates:", err);
        throw err;
    } finally {
        await client.close();
    }
    
}
// queryDates()
//     .then(dates => {
//         console.log(dates);
//     })
//     .catch(err => {
//         console.error(err);
//     });
// Inserts all records, batched daily, asynchronously
async function insertAllRecordsAsync() {
    const client = new MongoClient(uri);
    const dbName = "DailyRecordsTest1";
    const collectionName = "Records";
    const dailyRecordsDirectory = path.join(__dirname, "..//arb_handler//result_log_data//");
    // const testFilePath = "C://Users//dazzl//CodingProjects//substrate//test2//arb-dot-2//arb_handler//result_log_data//2023-10-29//Kusama_00-16-40.json"
    let days = await fs.readdirSync(dailyRecordsDirectory, { withFileTypes: true})
        .filter(dirEntry => dirEntry.isDirectory())
        .map(dirEntry => dirEntry.name)
    
    // Make sure all the directories are in the correct format
    const datePattern = /^\d{4}-\d{2}-\d{2}$/;
    days = days.filter((day: string) => datePattern.test(day));

    console.log(days)



    let formattedDailyRecords =await Promise.all(days.map(async (day: string) => {
        const dayDirectory = path.join(dailyRecordsDirectory, day);
        const timeFiles = await fs.readdirSync(dayDirectory, { withFileTypes: true})
            .filter(dirEntry => dirEntry.isFile())
            .map(dirEntry => {
                // console.log(dirEntry.name)
                return dirEntry.name
            })
        
        let fileNameAndTime: [string, string][] = [];
        const filePattern = /^Kusama_(\d{2}-\d{2}-\d{2})$/;
        timeFiles.forEach((fileName: string) => {
            let timeStamp = fileName.split("_")[1].split(".")[0];
            fileNameAndTime.push([fileName, timeStamp])
        })
        // console.log(fileNameAndTime)
        
        // Now use the current day folder and the file names to get the records
        // Upload records to MongoDB with the date and time, and record as a subdocument
        let formattedRecordsForDay = await Promise.all(fileNameAndTime.map(async ([fileName, timestamp]) => {
            const formattedRecord = {
                date: day,
                time: timestamp,
                record: JSON.parse(fs.readFileSync(path.join(dayDirectory, fileName), 'utf8'))
            }
            return formattedRecord
            // console.log(formattedRecord)
        }))
        // console.log(formattedRecordsForDay)
        return formattedRecordsForDay
    }))

    console.log(formattedDailyRecords)

    // Upload records, batched for each day
    await client.connect();
    const db = client.db(dbName);
    const collection = db.collection(collectionName);
    
    let uploadPromises: Promise<any>[] = [];

    for (let x in formattedDailyRecords) {
        const dailyRecord = formattedDailyRecords[x];

        // push each insertMany operation into promises array
        uploadPromises.push(
            collection.insertMany(dailyRecord)
                .then((result: any) => {
                    console.log(`Inserted ${result.insertedCount} records for ${dailyRecord[0].date}`);
                })
                .catch((err: any) => {
                    console.error(`Failed to insert records for ${dailyRecord[0].date}: ${err}`);
                })
        );
    }

    await Promise.all(uploadPromises);

    await client.close();
}


// run2().catch(console.dir);

async function run() {
    // TODO:
    // Replace the placeholder connection string below with your
    // Altas cluster specifics. Be sure it includes
    // a valid username and password! Note that in a production environment,
    // you do not want to store your password in plain-text here.


    // The MongoClient is the object that references the connection to our
    // datastore (Atlas, for example)
    const client = new MongoClient(uri);

    // The connect() method does not attempt a connection; instead it instructs
    // the driver to connect using the settings provided when a connection
    // is required.
    await client.connect();

    // Provide the name of the database and collection you want to use.
    // If the database and/or collection do not exist, the driver and Atlas
    // will create them automatically when you first write data.
    const dbName = "Daily Records";
    const collectionName = "Records";

    // Create references to the database and collection in order to run
    // operations on them.
    const database = client.db(dbName);
    const collection = database.collection(collectionName);

    // const testFilePath = path.join(__dirname, "test.txt");
    // const testFilePath = "C://Users//dazzl//CodingProjects//substrate//test2//arb-dot-2//arb_handler//result_log_data//2023-10-29//Kusama_00-16-40.json"
    // const testFile = fs.readFileSync(testFilePath);
    // const records = JSON.parse(testFile);

    /*
     *  *** INSERT DOCUMENTS ***
     *
     * You can insert individual documents using collection.insert().
     * In this example, we're going to create four documents and then
     * insert them all in one call with collection.insertMany().
     */

    const recipes = [
        {
            name: "elotes",
            ingredients: [
                "corn",
                "mayonnaise",
                "cotija cheese",
                "sour cream",
                "lime",
            ],
            prepTimeInMinutes: 35,
        },
        {
            name: "loco moco",
            ingredients: [
                "ground beef",
                "butter",
                "onion",
                "egg",
                "bread bun",
                "mushrooms",
            ],
            prepTimeInMinutes: 54,
        },
        {
            name: "patatas bravas",
            ingredients: [
                "potato",
                "tomato",
                "olive oil",
                "onion",
                "garlic",
                "paprika",
            ],
            prepTimeInMinutes: 80,
        },
        {
            name: "fried rice",
            ingredients: [
                "rice",
                "soy sauce",
                "egg",
                "onion",
                "pea",
                "carrot",
                "sesame oil",
            ],
            prepTimeInMinutes: 40,
        },
    ];

    try {
        const insertManyResult = await collection.insertMany(recipes);
        console.log(`${insertManyResult.insertedCount} documents successfully inserted.\n`);
    } catch (err) {
        console.error(`Something went wrong trying to insert the new documents: ${err}\n`);
    }

    /*
     * *** FIND DOCUMENTS ***
     *
     * Now that we have data in Atlas, we can read it. To retrieve all of
     * the data in a collection, we call Find() with an empty filter.
     * The Builders class is very helpful when building complex
     * filters, and is used here to show its most basic use.
     */

    const findQuery = { prepTimeInMinutes: { $lt: 45 } };

    try {
        const cursor = await collection.find(findQuery).sort({ name: 1 });
        await cursor.forEach((recipe: any) => {
            console.log(`${recipe.name} has ${recipe.ingredients.length} ingredients and takes ${recipe.prepTimeInMinutes} minutes to make.`);
        });
        // add a linebreak
        console.log();
    } catch (err) {
        console.error(`Something went wrong trying to find the documents: ${err}\n`);
    }

    // We can also find a single document. Let's find the first document
    // that has the string "potato" in the ingredients list.
    const findOneQuery = { ingredients: "potato" };

    try {
        const findOneResult = await collection.findOne(findOneQuery);
        if (findOneResult === null) {
            console.log("Couldn't find any recipes that contain 'potato' as an ingredient.\n");
        } else {
            console.log(`Found a recipe with 'potato' as an ingredient:\n${JSON.stringify(findOneResult)}\n`);
        }
    } catch (err) {
        console.error(`Something went wrong trying to find one document: ${err}\n`);
    }

    /*
     * *** UPDATE A DOCUMENT ***
     *
     * You can update a single document or multiple documents in a single call.
     *
     * Here we update the PrepTimeInMinutes value on the document we
     * just found.
     */
    const updateDoc = { $set: { prepTimeInMinutes: 72 } };

    // The following updateOptions document specifies that we want the *updated*
    // document to be returned. By default, we get the document as it was *before*
    // the update.
    const updateOptions = { returnOriginal: false };

    try {
        const updateResult = await collection.findOneAndUpdate(
            findOneQuery,
            updateDoc,
            updateOptions,
        );
        console.log(`Here is the updated document:\n${JSON.stringify(updateResult.value)}\n`);
    } catch (err) {
        console.error(`Something went wrong trying to update one document: ${err}\n`);
    }

    /*      *** DELETE DOCUMENTS ***
     *
     *      As with other CRUD methods, you can delete a single document
     *      or all documents that match a specified filter. To delete all
     *      of the documents in a collection, pass an empty filter to
     *      the DeleteMany() method. In this example, we'll delete two of
     *      the recipes.
     */


    const deleteQuery = { name: { $in: ["elotes", "fried rice"] } };
    try {
        const deleteResult = await collection.deleteMany(deleteQuery);
        console.log(`Deleted ${deleteResult.deletedCount} documents\n`);
    } catch (err) {
        console.error(`Something went wrong trying to delete documents: ${err}\n`);
    }

    // Make sure to call close() on your client to perform cleanup operations
    await client.close();
}
// insertAllRecordsAsync().catch(console.dir);
