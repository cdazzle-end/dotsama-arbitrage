import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../assets/asset_types';
import { MyLp } from './lp_types';
import * as bncHandler from './bnc/lp_handler'
import * as hkoHandler from './hko/lp_handler'
import * as karHandler from './kar/lp_handler'
import * as movrHandler from './movr/lp_handler'
import * as sdnHandler from './sdn/lp_handler'
import * as kucoinHandler from './kucoin/lp_handler'
import * as mgxHandler from './mgx/lp_handler'
import * as bsxHandler from './bsx/lp_handler'

const dateTimeOptions: Intl.DateTimeFormatOptions = {
    timeZone: 'America/New_York',
    hour12: false,
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
};

// const options: Intl.DateTimeFormatOptions = {
//     timeZone: 'America/New_York',
//     hour12: false,
//     year: "numeric",
//     month: "2-digit",
//     day: "2-digit",
//     hour: "2-digit",
//     minute: "2-digit",
//     second: "2-digit"
// };

// const formatter = new Intl.DateTimeFormat('en-US', options);
// const estTime = formatter.format(date);


// export async function updateLpsChop() {
//     await Promise.all([
//         bncHandler.updateLps().then(() => console.log("bnc complete")),
//         hkoHandler.updateLps().then(() => console.log("hko complete")),
//         karHandler.updateLps().then(() => console.log("kar complete")),
//         // kucoinHandler.updateLps().then(() => console.log("kucoin complete")),
//         mgxHandler.updateLps().then(() => console.log("mgx complete")),
//         bsxHandler.updateLps().then(() => console.log("bsx complete")),
//         movrHandler.updateLps().then(() => console.log("movr complete")),
//         sdnHandler.updateLps().then(() => console.log("sdn complete"))
//     ]);
// }


async function startTimer() {
    console.log("startTimer")
    const date = new Date();
    const startTime = date.toLocaleString('en-US', dateTimeOptions);
    fs.appendFileSync("lp_timestamps.txt", "LPs started at: " + startTime + "\n");
}
async function updateLpTimeStamp() {

    console.log("updateLpTimeStamp")
    const date = new Date();
    const startTime = date.toLocaleString('en-US', dateTimeOptions);
    fs.appendFileSync("lp_timestamps.txt", "LPs updated at: " + startTime + "\n");
}

async function updateLps(chopsticks: boolean) {
    await Promise.all([
        bncHandler.updateLps(chopsticks).then(() => console.log("bnc complete")),
        hkoHandler.updateLps(chopsticks).then(() => console.log("hko complete")),
        karHandler.updateLps(chopsticks).then(() => console.log("kar complete")),
        // kucoinHandler.updateLps().then(() => console.log("kucoin complete")),
        mgxHandler.updateLps(chopsticks).then(() => console.log("mgx complete")),
        bsxHandler.updateLps(chopsticks).then(() => console.log("bsx complete")),
        movrHandler.updateLps().then(() => console.log("movr complete")),
        sdnHandler.updateLps().then(() => console.log("sdn complete"))
    ]);
}

async function main() {

    let chopsticks = await process.argv.includes("true")

    console.log("chopsticks: " + chopsticks)

    startTimer()
    await updateLps(chopsticks)
    updateLpTimeStamp()
}

main()