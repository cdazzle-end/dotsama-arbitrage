import { ethers } from 'ethers'
import path from 'path'
import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { parse } from 'path'
import { formatUnits } from 'ethers/lib/utils';
// import {hexToDec2, decToHex2} from '../../parachains/hex'
import { ApiPromise, WsProvider } from '@polkadot/api';
import {  } from '@moonbeam-network/api-augment/moonriver/interfaces';
// const apiHelper = require('../parachains/api_utils')
// import apiHelper from '../parachains/api_utils'
import Keyring from '@polkadot/keyring';
import { u8aToHex, stringToHex , numberToHex} from '@polkadot/util';
import { mnemonicToLegacySeed, hdEthereum } from '@polkadot/util-crypto';
const rpc1 = 'wss://wss.moonriver.moonbeam.network';
const rpc2 = 'wss://moonriver.public.blastapi.io';
const rpc3 = 'wss://moonriver.api.onfinality.io/public-ws';
const rpc4 = 'wss://moonriver.unitedbloc.com'

// // hex.d.ts
// declare module '../../parachains/hex' {
//     export function hexToDec2(hex: string): Uint8Array;
//     export function decToHex2(u8a: Uint8Array): string;
//     // ... add any other exports from the hex.js module here
// }

// const providerRPC = {
//     moonriver: {
//         name: 'moonriver',
//         rpc: 'https://moonriver.api.onfinality.io/public', // Insert your RPC URL here
//         chainId: 1285, // 0x505 in hex,
//     },
// };
// const provider = new ethers.providers.StaticJsonRpcProvider(
//     providerRPC.moonriver.rpc,
//     {
//         chainId: providerRPC.moonriver.chainId,
//         name: providerRPC.moonriver.name,
//     }
// );
const provider = new ethers.WebSocketProvider(rpc1);
const dexContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function getReserves() view returns (uint, uint, uint)",
    "function token0() view returns (address)",
    "function token1() view returns (address)"
]

//getReserves returns 2 uint instead of 3
const altDexContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function getReserves() view returns (uint, uint)",
    "function token0() view returns (address)",
    "function token1() view returns (address)"
]

const tokenContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function decimals() view returns (uint8)",
    "event Transfer(address indexed src, address indexed dst, uint val)"
]

export async function saveAssets() {
    const assets = await queryAssets();
    const locations = await queryLocations();
    let assetRegistry = assets.map((asset) => {
        const matchedLocation = locations.find((location) => location[1] == asset.localId)
        // console.log(asset.localId + " " + matchedLocation)
        const newAssetRegistryObject: MyAssetRegistryObject = matchedLocation ? {
            tokenData: asset,
            hasLocation: true,
            tokenLocation: matchedLocation[0]
        } : {
            tokenData: asset,
            hasLocation: false,
        }
        return newAssetRegistryObject
    })
    const filePath = path.join(__dirname, '/asset_registry.json')
    await fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2))
    process.exit(0)
}

export async function getAssets(): Promise<MyAssetRegistryObject[]> {
    const assetRegistry = JSON.parse(fs.readFileSync('../assets/movr/asset_registry.json', 'utf8'));
    return assetRegistry
}

async function findValueByKey(obj: any, targetKey: any): Promise<any> {
    if (typeof obj !== 'object' || obj === null) {
        return null;
    }
    for (let key in obj) {
        if (key === targetKey) {
            return obj[key];
        }

        let foundValue: any = await findValueByKey(obj[key], targetKey);
        if (foundValue !== null) {
            return foundValue;
        }
    }

    return null;
}
function removeCommasFromAllValues(obj: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    for (let key in obj) {
        if (typeof obj[key] === 'string') {
            // Remove commas if the value is a string
            obj[key] = obj[key].replace(/,/g, '');
        } else {
            // Recursively remove commas from nested objects
            obj[key] = removeCommasFromAllValues(obj[key]);
        }
    }

    return obj;
}

async function queryLocations() {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const locationEntries = await api.query.assetManager.assetIdType.entries();
    // console.log(locationEntries.length + " " + loc2.length)
    let assetLocations = await Promise.all(locationEntries.map(async ([key, value]) => {
        const currencyId = (key.toHuman() as any)[0].replace(/,/g, "");
        let locationData = (value.toHuman() as any)['Xcm']['interior'];
        locationData = removeCommasFromAllValues(locationData);
        const junction = Object.keys(locationData)[0]

        let genKey = await findValueByKey(locationData, "generalKey")

        let junctionList: MyJunction[] = []
        if (locationData == "Here") {
            const newLocation = "here"
            return [newLocation, currencyId]
        } else {
            const junctionData = locationData[junction]

            if (!Array.isArray(junctionData)) { // If junction is X1
                let newLocation: MyMultiLocation;
                let newJunction: MyJunction = junctionData;
                newLocation = {
                    [junction]: newJunction
                }
                return [newLocation, currencyId]

            } else {
                const junctions = locationData[junction];
                junctions.forEach((junction: any) => {
                    const newJunction: MyJunction = junction;
                    junctionList.push(newJunction)
                })
                let newLocation: MyMultiLocation = {
                    [junction]: junctionList
                }
                return [newLocation, currencyId]
            }
        }
    }))



    const movrLocation = {
        X2: [{Parachain: "2023"},{PalletInstance: "10"}]
    }
    const zlkOldGeneralKey = "0x0207";
    // Remove '0x' prefix and calculate the length
    const keyWithoutPrefix = zlkOldGeneralKey.slice(2);
    const length = keyWithoutPrefix.length / 2;

    // Right-pad the key with zeros to 64 characters (32 bytes)
    const paddedData = keyWithoutPrefix.padEnd(64, '0');
    const newGeneralKey = {
        length: JSON.stringify(length),
        data: '0x' + paddedData
    }

    const zlkLocation = {
        X2: [{ Parachain: "2001" }, { GeneralKey: newGeneralKey }]
    }
    assetLocations.push([movrLocation, "MOVR"])
    assetLocations.push([zlkLocation, "ZLK"])

    assetLocations.forEach(([location, id]) => {
        console.log(id)
        console.log(JSON.stringify(location))
    })

    return assetLocations;
}

async function queryAssets(): Promise<MyAsset[]> {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number
    const assetEntries = await api.query.assets.metadata.entries();
    let assets = await Promise.all(assetEntries.map(async ([assetId, asset]) => {
        const assetData = asset.toHuman() as any;
        const id = (assetId.toHuman() as any)[0].replace(/,/g, "");
        const hexContractAddress = numberToHex(id);
        let hex = await decToHex(id)
        hex = hex!.slice(2)
        //LEADING ZERO GETS REMOVED, SO ADD IT BACK IN
        if (hex.length === 31) {
            hex = "0" + hex
        }
        let evmContractAddress = "0xFFFFFFFF" + hex;


        const newAsset: MyAsset = {
            network: "kusama",
            chain: parachainId,
            localId: (assetId.toHuman() as any)[0].replace(/,/g, ""),
            name: assetData.name,
            symbol: assetData.symbol,
            decimals: assetData.decimals,
            deposit: assetData.deposit,
            isFrozen: assetData.isFrozen,
            contractAddress: evmContractAddress,
        }
        console.log(newAsset)
        return newAsset
    }))
    const movrAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: "MOVR",
        name: "MOVR",
        symbol: "MOVR",
        decimals: "18",
        contractAddress: "0x98878B06940aE243284CA214f92Bb71a2b032B8A"
    }
    const zlkAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: "ZLK",
        name: "ZLK",
        symbol: "ZLK",
        decimals: "18",
        contractAddress: "0x0f47ba9d9Bde3442b42175e51d6A367928A1173B"
    }
    assets.push(movrAsset)
    assets.push(zlkAsset)
    console.log(assets.length)
    return assets
}

async function main() {
    // queryAssets()
    await saveAssets()
    // await queryLocations()
    process.exit(0)
}

// main()

/**
 * A function for converting hex <-> dec w/o loss of precision.
 *
 * The problem is that parseInt("0x12345...") isn't precise enough to convert
 * 64-bit integers correctly.
 *
 * Internally, this uses arrays to encode decimal digits starting with the least
 * significant:
 * 8 = [8]
 * 16 = [6, 1]
 * 1024 = [4, 2, 0, 1]
 */

// Adds two arrays for the given base (10 or 16), returning the result.
// This turns out to be the only "primitive" operation we need.
function add(x:any, y:any, base:any) {
    var z: any = [];
    var n = Math.max(x.length, y.length);
    var carry = 0;
    var i = 0;
    while (i < n || carry) {
        var xi = i < x.length ? x[i] : 0;
        var yi = i < y.length ? y[i] : 0;
        var zi = carry + xi + yi;
        z.push(zi % base);
        carry = Math.floor(zi / base);
        i++;
    }
    return z;
}

// Returns a*x, where x is an array of decimal digits and a is an ordinary
// JavaScript number. base is the number base of the array x.
function multiplyByNumber(num:any, x:any, base: any) {
    if (num < 0) return null;
    if (num == 0) return [];

    var result = [];
    var power = x;
    while (true) {
        if (num & 1) {
            result = add(result, power, base);
        }
        num = num >> 1;
        if (num === 0) break;
        power = add(power, power, base);
    }

    return result;
}

function parseToDigitsArray(str: any, base: any) {
    var digits = str.split('');
    var ary: any = [];
    for (var i = digits.length - 1; i >= 0; i--) {
        var n = parseInt(digits[i], base);
        if (isNaN(n)) return null;
        ary.push(n);
    }
    return ary;
}

function convertBase(str: any, fromBase: any, toBase: any) {
    var digits = parseToDigitsArray(str, fromBase);
    if (digits === null) return null;

    var outArray: any = [];
    var power: any = [1];
    for (var i = 0; i < digits.length; i++) {
        // invariant: at this point, fromBase^i = power
        if (digits[i]) {
            outArray = add(outArray, multiplyByNumber(digits[i], power, toBase), toBase);
        }
        power = multiplyByNumber(fromBase, power, toBase);
    }

    var out: any = '';
    for (var i = outArray.length - 1; i >= 0; i--) {
        out += outArray[i].toString(toBase);
    }
    return out;
}

async function decToHex(decStr: any){
    var hex = convertBase(decStr, 10, 16);
    return hex ? '0x' + hex : null;
}

async function hexToDec(hexStr: any) {
    if (hexStr.substring(0, 2) === '0x') hexStr = hexStr.substring(2);
    hexStr = hexStr.toLowerCase();
    return convertBase(hexStr, 16, 10);
}