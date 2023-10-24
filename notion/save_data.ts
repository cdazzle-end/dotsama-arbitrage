import dotenv from 'dotenv';
import { Client } from "@notionhq/client"
import fs from 'fs';
dotenv.config();

// const notion = process.env.NOTION_KEY;
// const databaseUrl = process.env.NOTION_DATABASE_ID;

const notion = new Client({ auth: process.env.NOTION_KEY })

const databaseId = process.env.NOTION_DATABASE_ID as string;
const entry1Page = '02c30282b60b4963995f7994f037482d'
const entry1Database = '27979054c6cf4380b310c6fcaafa3bff'
const mainPageId = '80a624cbc2d84d23848b2ee3e0945d3a'
const tablePageId = '79b684c18351484e960b41a8af811acd'
const allEntriesPageId = '64cddad549d4432fa3662fdf0ef52609'

const entry1results1 = {

    node_key: {
        type: "title",
        title: [
            {
                type: "rich_text",
                rich_text: {
                    "content": "node_key_1"
                }
            }
        ]
    },
    // asset_name: {
    //     "type": "rich_text",
    //     "rich_text": { "content": "asset_name_1" }
    // },
    // path_value: {
    //     "type": "rich_text",
    //     "rich_text": { "content": "path_value_1" }
    // },
}
async function addItem(text: string) {
    try {
        const response = await notion.pages.create({
            parent: { database_id: '1906fbdab99845469b251ddfa9013ee8' },
            properties: {
                title: {
                    title: [
                        {
                            "text": {
                                "content": text
                            }
                        }
                    ]
                }
            },
        })
        console.log(response)
        console.log("Success! Entry added.")
    } catch (error: any) {
        console.error(error.body)
    }
}
async function addItem2() {
    try {
        const response = await notion.pages.create({
            parent: { database_id: '1906fbdab99845469b251ddfa9013ee8' },
            properties: {
                "node_key": {
                    title: [
                        {
                            "text": {
                                "content": "test key"
                            }
                        }
                    ]
                },
                "asset_name": {
                    rich_text: [
                        {
                            "text": {
                                "content": "test asset"
                            }
                        }
                    ]
                },
                "path_value": {
                    number: 1
                },
            },
            // position: 2,
        })
        console.log(response)
        console.log("Success! Entry added.")
    } catch (error: any) {
        console.error(error.body)
    }
}

async function getDatabase() {
    try {
        const response = await notion.databases.query({
            database_id: databaseId,
        })
        console.log(response)
    } catch (error: any) {
        console.error(error.body)
    }
}

// Add the json data from the result of arb to the database for the day it was run
async function addResultToDay(properties: any) {
    try {
        const response = await notion.pages.create({
            parent: { database_id: databaseId },
            properties: properties,
        });
        console.log("Success! Entry added.");
        console.log(response);
    } catch (error: any) {
        console.error(error.body);
    }
}

async function addRowToEntryDatabase() {
    // try {
    //     const response = await notion.pages.create({
    //         parent: { database_id: entry1Database },
    //         properties: entry1results1
    //     })
    //     console.log(response)
    //     console.log("Success! Entry added.")
    // } catch (error: any) {
    //     console.error(error.body);
    // }
}

async function addPageToPage() {
    try {
        const response = await notion.pages.create({
            parent: {
                page_id: '80a624cbc2d84d23848b2ee3e0945d3a',
            },
            properties: {
                title: {
                    type: 'title',
                    title: [
                        {
                            type: 'text',
                            text: {
                                content: 'A note from your pals at Notion',
                            },
                        },
                    ],
                },
            },
            children: [
                {
                    object: 'block',
                    type: 'paragraph',
                    paragraph: {
                        rich_text: [
                            {
                                type: 'text',
                                text: {
                                    content: 'You made this page using the Notion API. Pretty cool, huh? We hope you enjoy building with us.',
                                },
                            },
                        ],
                    },
                },
            ],
        });
        console.log(response);
    } catch (error: any) {
        console.error(error.body);
    }
}

//Adds table with 2 columns to main page
async function addTableToPage() {
    try {
        const response = await notion.pages.create({
        // const response = await notion.blocks.children.append({
            parent: {
                page_id: mainPageId,
            },
            // block_id: databaseId,
            properties: {
                title: [
                    {
                        text: {
                            content: "Table Page",
                        },
                    },
                ],
            },
            children: [
                {
                    object: "block",
                    type: "table",
                    table: {
                        table_width: 2,
                        has_column_header: true,
                        has_row_header: true,
                        children: [
                            {
                                object: "block",
                                type: "table_row",
                                table_row: {
                                    cells: [
                                        [
                                        {
                                            type: "text",
                                            text: {
                                                content: "column 1 content",
                                                link: null,
                                            },
                                        },
                                        ],
                                        [
                                            {
                                                type: "text",
                                                text: {
                                                    content: "column 2 content",
                                                    link: null,
                                                },
                                            },
                                        ]
                                        
                                        
                                    ]
                                }
                            },
                            {
                                object: "block",
                                type: "table_row",
                                table_row: {
                                    cells: [
                                        [
                                            {
                                                
                                                type: "text",
                                                text: {
                                                    content: "column 1 content",
                                                    link: null,
                                                },
                                            },
                                        ],
                                        [
                                            {
                                                type: "text",
                                                text: {
                                                    content: "column 2 content",
                                                    link: null,
                                                },
                                            },
                                        ]
                                    ]
                                }
                            }
                        ]
                    },
                },
            ],
        });
        console.log("Success! Table added.");
        console.log(response);
    } catch (error: any) {
        console.error(error.body);
    }
}

//Add a database to the tablepage, which contains a table for the day
async function addFullPageDatabaseToPage() {
    try {
        const response = await notion.databases.create({
            parent: {
                page_id: tablePageId,
            },
            title: [
                {
                    text: {
                        content: "Insert Title",
                    },
                },
            ],
            properties: {
                "day": {
                    type: 'title',
                    title: {}
                },
                // "day": {
                //     type: 'title',
                //     title: {}
                // },
                "current_date": {
                    type: "date",
                    date: {}
                },

            },
            // children: [
            //     {
            //         object: "block",
            //         type: "child_database",
            //         collection_id: databaseId,
            //     },
            // ],
        });

        console.log("Success! Database added.");
        console.log(response);
    } catch (error: any) {
        console.error(error.body);
    }
}

//Add an entry to a day table, an entry contains a table of json data
async function addEntryToDayTable() {

}

//Create an entry for a day, on the all entries page. contains JSON Data
async function createEntry() {
    //All Entries page ID
    const testEntryData = await getTestJSON();

    //Create table rows from json data
    const tableRows = testEntryData.map((entry: any) => {
        return {
            object: "block",
            type: "table_row",
            table_row: {
                cells: [
                    [
                        {
                            type: "text",
                            text: {
                                content: entry["name"],
                                link: null,
                            },
                        },
                    ],
                    [
                        {
                            type: "text",
                            text: {
                                content: entry["value"],
                                link: null,
                            },
                        },
                    ]
                ]
            }
        }
    })

    try {
        const response = await notion.databases.create({
            parent: {
                page_id: allEntriesPageId,
            },
            title: [
                {
                    text: {
                        content: "Test Entry Title",
                    },
                },
            ],
            properties: {
                "node_key": {
                    type: 'title',
                    title: {}
                },
                "asset_name": {
                    type: "rich_text",
                    rich_text: {}
                },
                "path_value": {
                    type: "number",
                    number: {}
                },
            },
        });

        console.log("Success! Database added.");
        console.log(response);

        const newDatabaseId = response.id;
        for (const rowData of testEntryData) {
            // const data = rowData.node_key;
            // console.log(data)
            console.log(rowData)
            const newRow = await notion.pages.create({
                parent: {
                    database_id: newDatabaseId,
                },
                properties: {
                    "node_key": {
                        type: 'title',
                        title: [
                            {
                                text: {
                                    content: rowData["node_key"],
                                },
                            },
                        ],
                    },
                    "asset_name": {
                        type: "rich_text",
                        rich_text: [
                            {
                                text: {
                                    content: rowData["asset_name"],
                                },
                            },
                        ],
                    },
                    "path_value": {
                        type: "number",
                        number: rowData["path_value"]
                    },
                },
            });
        }


    } catch (error: any) {
        console.error(error.body);
    }
}

async function getTestJSON() {
    const testFile = "\..\\arb_handler\\result_log_data\\2023-07-19\\Kusama_15-30-59.json"
    try {
        const jsonData = fs.readFileSync(testFile, 'utf-8');
        const parsedData = JSON.parse(jsonData);
        parsedData.forEach((element: any) => {
            console.log(element)
            console.log(element.node_key)
            console.log(element.asset_name)
            console.log(element.path_value)
        });
        // console.log(parsedData);
        return parsedData;
    } catch (error) {
        console.error(`Error reading JSON file: ${error}`);
        return null;
    }
}

async function main() {
    // console.log(apiKey, databaseUrl)
    // addItem("Yurts in Big Sur, California")
    // addPageToPage()
    // addTableToPage()
    // addFullPageDatabaseToPage()
    // getTestJSON()
    // createEntry()
    addItem2()
}

main();