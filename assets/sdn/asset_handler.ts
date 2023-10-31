import * as fs from 'fs';
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
import { options } from '@astar-network/astar-api'
import { AssetId, AssignmentId  } from '@polkadot/types/interfaces';
import { MyAsset, MyAssetRegistryObject, MyMultiLocation, MyJunction } from '../asset_types';

export async function saveAssets() {
    const assets = await queryAssets();
    const assetLocations = await queryLocations();
    let assetRegistry = assets.map((asset, index) => {
        const matchedLocation = assetLocations.find((location) => asset.localId === location[1]);
        console.log(matchedLocation)
        const newAssetRegistryObject: MyAssetRegistryObject = matchedLocation ? {
            tokenData: asset,
            hasLocation: true,
            tokenLocation: matchedLocation[0]
        } : {
            tokenData: asset,
            hasLocation: false,
        }
        return newAssetRegistryObject;
    });
    // assetRegistry.forEach((asset) => console.log(asset))
    fs.writeFileSync('../sdn/asset_registry.json', JSON.stringify(assetRegistry, null, 2));
}

export async function getAssets() {
    return JSON.parse(fs.readFileSync('./sdn/asset_registry.json', 'utf8'));
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
    const provider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    let locations = await api.query.xcAssetConfig.assetIdToLocation.entries();
    let assetLocations = locations.map(([assetId, location]) => {
        let locationData = (location.toHuman() as any)['V3']['interior']
        locationData = removeCommasFromAllValues(locationData)
        const currencyId = (assetId.args[0].toHuman() as string).replace(/,/g, '')
        const junction = Object.keys(locationData)[0]

        // let genKey = await findValueByKey(locationData, "generalKey")

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

    })
    const sdnLocation = {
        X1: { Parachain: "2007" }
    }
    assetLocations.push([sdnLocation, "SDN"])

    assetLocations.forEach((location) => console.log(JSON.stringify(location)))

    return assetLocations
}

async function queryAssets() {
    const provider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number
    let assets = (await api.query.assets.metadata.entries()).map(([assetId, asset]) => {
        const formattedAssetId = (assetId.args[0].toHuman() as string).replace(/,/g, '')
        let hex = decToHex(formattedAssetId)
        hex = hex!.slice(2)
        while (hex.length < 32) {
            hex = "0" + hex    
        }
        let evmContractAddress = "0xFFFFFFFF" + hex;
        const assetData = asset.toHuman() as any
        const newAsset: MyAsset = {
            network: "kusama",
            chain: parachainId,
            localId: formattedAssetId,
            name: assetData.name,
            symbol: assetData.symbol,
            decimals: assetData.decimals,
            deposit: assetData.deposit.replace(/,/g, ''),
            isFrozen: assetData.isFrozen,
            contractAddress: evmContractAddress,
        }
        console.log(newAsset)
        return newAsset
    });
    const sdnAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: "SDN",
        name: "SDN",
        symbol: "SDN",
        decimals: "18",
        contractAddress: "0x0f933Dc137D21cA519ae4C7E93f87a4C8EF365Ef"
    }
    assets.push(sdnAsset)
    return assets
}

async function main() {

    await saveAssets()
    // getAssets()
    // await queryAssets()
    // await queryLocations()

    process.exit(0);
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
function add(x: any, y: any, base: any) {
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
function multiplyByNumber(num: any, x: any, base: any) {
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

function decToHex(decStr: any) {
    var hex = convertBase(decStr, 10, 16);
    return hex ? '0x' + hex : null;
}

function hexToDec(hexStr: any) {
    if (hexStr.substring(0, 2) === '0x') hexStr = hexStr.substring(2);
    hexStr = hexStr.toLowerCase();
    return convertBase(hexStr, 16, 10);
}