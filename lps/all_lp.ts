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

async function updateLps() {
    bncHandler.updateLps().then(() => console.log("bnc complete"))
    hkoHandler.updateLps().then(() => console.log("hko complete"))
    karHandler.updateLps().then(() => console.log("kar complete"))

    kucoinHandler.updateLps().then(() => console.log("kucoin complete"))
    mgxHandler.updateLps().then(() => console.log("mgx complete"))
    bsxHandler.updateLps().then(() => console.log("bsx complete"))

    movrHandler.updateLps().then(() => console.log("movr complete"))
    // sdnHandler.updateLps().then(() => console.log("sdn complete"))
}

async function main() {
    updateLps()
}

main()