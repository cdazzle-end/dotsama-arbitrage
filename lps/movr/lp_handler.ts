import { ethers } from 'ethers'
import * as fs from 'fs';
// import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { parse } from 'path'
// import { formatUnits } from 'ethers/lib/utils';
// import {hexToDec2, decToHex2} from '../../parachains/hex'
import { ApiPromise, WsProvider } from '@polkadot/api';
// import { } from '@moonbeam-network/api-augment/moonriver/interfaces';
// const apiHelper = require('../parachains/api_utils')
// import apiHelper from '../parachains/api_utils'
import Keyring from '@polkadot/keyring';
import { u8aToHex, stringToHex, numberToHex } from '@polkadot/util';
import { mnemonicToLegacySeed, hdEthereum } from '@polkadot/util-crypto';
import { MyLp } from '../lp_types';
const rpc1 = 'wss://wss.moonriver.moonbeam.network';
const rpc2 = 'wss://moonriver.public.blastapi.io';
const rpc3 = 'wss://moonriver.api.onfinality.io/public-ws';
const rpc4 = 'wss://moonriver.unitedbloc.com'

const providerRPC = {
    moonriver: {
        name: 'moonriver',
        rpc: rpc2, // Insert your RPC URL here
        chainId: 1285, // 0x505 in hex,
    },
};
// const provider = new ethers.JsonRpcProvider(
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



export async function updateLps() {
    
    const lps = JSON.parse(fs.readFileSync('./movr/lps_base.json', 'utf8'))
    const asseRegistry = JSON.parse(fs.readFileSync('../assets/movr/asset_registry.json', 'utf8'))
    lps.map((lp: any) => {
        
        const token0 = asseRegistry.find((asset: any) => asset.tokenData.contractAddress.toLowerCase() == lp.poolAssets[0].toLowerCase() )
        const token1 = asseRegistry.find((asset: any) => asset.tokenData.contractAddress.toLowerCase() == lp.poolAssets[1].toLowerCase() )
        lp.poolAssets = [token0? token0.tokenData.localId : lp.poolAssets[0], token1? token1.tokenData.localId : lp.poolAssets[1]]
    })

    const updatedLps = (await Promise.all(lps.map(async (lp: any) => {
        
        const pool = await new ethers.Contract(lp.contractAddress, altDexContractAbi, provider);
        let reserves = await pool.getReserves();
        // console.log(lp)
        // console.log(reserves)
        // let reserve_0 = await hexToDec(reserves[0]["_hex"]);
        // let reserve_1 = await hexToDec(reserves[1]["_hex"]);
        let reserve_0 = removeLastChar(reserves[0].toString());
        let reserve_1 = removeLastChar(reserves[1].toString());
        // console.log(reserve_0, reserve_1)
        // const newPool: MyLp = {
        //     chainId: 2023,
        //     contractAddress: lp.contractAddress,
        //     poolAssets: lp.poolAssets,
        //     liquidityStats: [reserve_0, reserve_1]
        // }
        // // console.log(newPool)
        // return newPool;
        if (reserve_0 !== "" && reserve_1 !== "") {
            const newPool: MyLp = {
                chainId: 2023,
                contractAddress: lp.contractAddress,
                poolAssets: lp.poolAssets,
                liquidityStats: [reserve_0, reserve_1]
            };
            return newPool;
        }
    }))).filter(pool => pool != null); // Filter out null entries
    fs.writeFileSync('./movr/lps.json', JSON.stringify(updatedLps, null, 2))
    provider.destroy()

}

async function saveLps() {
    const lpContracts = JSON.parse(fs.readFileSync('./lp_contracts', 'utf8'))
    const lps = await Promise.all(lpContracts.map(async (lpContract: any) => {
        const pool = await new ethers.Contract(lpContract, altDexContractAbi, provider);
        let reserves = await pool.getReserves();
        const token0 = await pool.token0();
        const token1 = await pool.token1();
        let reserve_0 = await hexToDec(reserves[0]["_hex"]);
        let reserve_1 = await hexToDec(reserves[1]["_hex"]);
        let newliquidityStats = [reserve_0, reserve_1];
        // let newPool = new LiqPool("2023", poolAddress, pool["poolAssets"], newliquidityStats);
        const newPool: MyLp = {
            chainId: 2023,
            contractAddress: lpContract,
            poolAssets: [token0, token1],
            liquidityStats: newliquidityStats
        }
        return newPool;
    }))
    console.log(lps)
    fs.writeFileSync('./lps_base.json', JSON.stringify(lps, null, 2))
}



async function lpList() {
    const lps = JSON.parse(fs.readFileSync('./liq_pool_registry', 'utf8')).map((lp: any) => {
        return lp.contractAddress
    })
    fs.writeFileSync('./lp_contracts', JSON.stringify(lps, null, 2))
}

async function main() {
    await updateLps()
}

// main()

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

async function decToHex(decStr: any) {
    var hex = convertBase(decStr, 10, 16);
    return hex ? '0x' + hex : null;
}

async function hexToDec(hexStr: any) {
    if (hexStr.substring(0, 2) === '0x') hexStr = hexStr.substring(2);
    hexStr = hexStr.toLowerCase();
    return convertBase(hexStr, 16, 10);
}

function removeLastChar(str: string) {
    return str.slice(0, -1);
}