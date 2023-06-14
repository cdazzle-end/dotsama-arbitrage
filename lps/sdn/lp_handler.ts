import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
import { options } from '@astar-network/astar-api';
import { ethers } from 'ethers';
import * as fs from 'fs';
import { MyLp } from '../lp_types';
declare const fetch: any;

// 2. Define network configurations
const providerRPC = {
    sdn: {
        name: 'sdn',
        rpc: 'https://shiden.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 592, // 0x504 in hex,
    },
};

// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.sdn.rpc,
    {
        chainId: providerRPC.sdn.chainId,
        name: providerRPC.sdn.name,
    }
);

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

const testContract = "0x1Ba530cf929ea5bc7f1Af241495C97331Ddb4f70"
const wsdn = "0x0f933Dc137D21cA519ae4C7E93f87a4C8EF365Ef"

export async function updateLps() {
    const lpAddresses = JSON.parse(fs.readFileSync('./sdn/all_lp_addresses.json', 'utf8'))
    // const lpContracts = JSON.parse(fs.readFileSync('./lp_contracts', 'utf8'))
    const lps = await Promise.all(lpAddresses.map(async (lpContract: any) => {
        const pool = await new ethers.Contract(lpContract, altDexContractAbi, provider);
        let reserves = await pool.getReserves();
        const token0 = await pool.token0();
        const token1 = await pool.token1();
        let reserve_0 = await hexToDec(reserves[0]["_hex"]);
        let reserve_1 = await hexToDec(reserves[1]["_hex"]);
        let newliquidityStats = [reserve_0, reserve_1];
        // let newPool = new LiqPool("2023", poolAddress, pool["poolAssets"], newliquidityStats);
        const newPool: MyLp = {
            chainId: 2007,
            contractAddress: lpContract,
            poolAssets: [token0, token1],
            liquidityStats: newliquidityStats
        }
        return newPool;
    }))
    // console.log(lps)
    // console.log(lps.length)
    fs.writeFileSync('./sdn/lps.json', JSON.stringify(lps, null, 2))
}

async function saveLps() {
    const provider = new WsProvider('wss://shiden.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    console.log((await api.rpc.system.properties()).toHuman());
    let count = 0;


    getSdnLiqPools()
}

// Save evm lps
async function saveEvmLps() {
    const lpAddresses = JSON.parse(fs.readFileSync('all_lp_addresses.json', 'utf8'))
    // const lpContracts = JSON.parse(fs.readFileSync('./lp_contracts', 'utf8'))
    const lps = await Promise.all(lpAddresses.map(async (lpContract: any) => {
        const pool = await new ethers.Contract(lpContract, altDexContractAbi, provider);
        let reserves = await pool.getReserves();
        const token0 = await pool.token0();
        const token1 = await pool.token1();
        let reserve_0 = await hexToDec(reserves[0]["_hex"]);
        let reserve_1 = await hexToDec(reserves[1]["_hex"]);
        let newliquidityStats = [reserve_0, reserve_1];
        // let newPool = new LiqPool("2023", poolAddress, pool["poolAssets"], newliquidityStats);
        const newPool: MyLp = {
            chainId: 2007,
            contractAddress: lpContract,
            poolAssets: [token0, token1],
            liquidityStats: newliquidityStats
        }
        return newPool;
    }))
    console.log(lps)
    console.log(lps.length)
    fs.writeFileSync('./lps.json', JSON.stringify(lps, null, 2))
}

//From list of assets, get token holders and filter for the dexs
// async function getAllLps() {
//     let allAssets: any[] = JSON.parse(fs.readFileSync('evm_tokens.json', 'utf8'))
//     let allLps: any[] = [];
//     await Promise.all(allAssets.map(async (asset: any) => {
//         let tokenContract = new ethers.Contract(asset, tokenContractAbi, provider);
//         let tokenName = await tokenContract.name();
//         let tokenSymbol = await tokenContract.symbol();
//         let tokenDecimals = await tokenContract.decimals();
//         let tokenHolders = await tokenContract.queryFilter(tokenContract.filters.Transfer(null, null, null))
//         let tokenHoldersAddresses = tokenHolders.map((holder: any) => holder.args[1])
//         let tokenHoldersAddressesUnique = [...new Set(tokenHoldersAddresses)]
//         let tokenHoldersAddressesUniqueLowerCase = tokenHoldersAddressesUnique.map((holder: any) => holder.toLowerCase())
//         let tokenHoldersAddressesUniqueLowerCaseFiltered = tokenHoldersAddressesUniqueLowerCase.filter((holder: any) => holder != wsdn)
//         let tokenHoldersAddressesUniqueLowerCaseFilteredDexs = tokenHoldersAddressesUniqueLowerCaseFiltered.filter((holder: any) => isDex(holder))
//         console.log(tokenHoldersAddressesUniqueLowerCaseFilteredDexs)
//         allLps.push({
//             name: tokenName,
//             symbol: tokenSymbol,
//             decimals: tokenDecimals,
//             address: asset,
//             holders: tokenHoldersAddressesUniqueLowerCaseFilteredDexs
//         })
//     }))
//     console.log(allLps)
//     console.log(allLps.length)
//     fs.writeFileSync('evm_lps.json', JSON.stringify(allLps, null, 2))
// }

// Get list of assets, get holders for each asset, 
async function getAllLps2() {
    let allAssets: any[] = JSON.parse(fs.readFileSync('evm_tokens.json', 'utf8'))
    let currentTokenHolders = JSON.parse(fs.readFileSync('formatted_all_token_holders.json', 'utf8'))
    const currentAssets = currentTokenHolders.map((tokenHolderObject: any) => {
        return tokenHolderObject.asset
    })
    let currentSearchAssets: any[] = [];
    let searchCounter = 0;
    while (searchCounter < 5) {
        const newAssetToSearch = allAssets.find((asset: any) => {
            if (!currentAssets.includes(asset) && !currentSearchAssets.includes(asset)) {
                searchCounter++;
                return true
            }
        })
        if (newAssetToSearch == undefined) {
            console.log("no more new assets")
            searchCounter++
        }
        currentSearchAssets.push(newAssetToSearch)
    }

    // let testAssets = allAssets.slice(0, 5)
    let allLps: any[] = [];
    // let uri = `https://blockscout.com/shiden/api?module=token&action=getTokenHolders&contractaddress=${wsdn}&page=${page}&offset=${offset}`;
    let newTokenHolders = await Promise.all(currentSearchAssets.map(async (asset) => {
        let uris: any[] = [];
        for (var i = 1; i < 5; i++) {
            let page = i;
            let offset = 50;
            let uri = `https://blockscout.com/shiden/api?module=token&action=getTokenHolders&contractaddress=${asset}&page=${page}&offset=${offset}`;
            uris.push(uri);
        }
        const promises: any[] = uris.map(async (uri) => {
            return fetch(uri)
        })
        const results = await Promise.all(promises)
            .then(responses => {
                return Promise.all(responses.map(async response => {
                    const res = await response.json()
                    console.log(res)
                    return res
                }))
            })
        let tokenHolders: any[] = [];
        results.map(async (result: any) => {
            result.result.map((holder: any) => {
                tokenHolders.push(holder.address)
                allLps.push(holder.address)
            })
        })
        let tokenHolderObject = {
            asset: asset,
            holders: tokenHolders
        }
        return tokenHolderObject
    }))
    newTokenHolders.map((tokenHolderObject: any) => {
        console.log(tokenHolderObject.holders.length)
        currentTokenHolders.push(tokenHolderObject)
    })
    // console.log(allLps.length)
    fs.writeFileSync('formatted_all_token_holders.json', JSON.stringify(currentTokenHolders, null, 2))
}

// filter for dexs, get new assets, repeat
async function getDexFromAllAssetHolders() {
    let tokenHolderAddresses: any[] = [];
    JSON.parse(fs.readFileSync('formatted_all_token_holders.json', 'utf8')).map((tokenHolderObject: any) => {
        tokenHolderObject.holders.map((holder: any) => {
            tokenHolderAddresses.push(holder)    
        })
    })
    let allLps = await Promise.all(tokenHolderAddresses.map(async (holder: any) => {
        if (await isDex(holder)) {
            return holder
        }
    }))
    allLps = allLps.filter((lp: any) => lp != undefined)
    console.log(allLps)
    console.log(allLps.length)
    fs.writeFileSync('all_lp_addresses.json', JSON.stringify(allLps, null, 2))
}

async function combineSdnAndAll() {
    let wsdn = JSON.parse(fs.readFileSync('wsdn_lp_addresses.json', 'utf8'))
    let rest = JSON.parse(fs.readFileSync('all_lp_addresses.json', 'utf8'))
    console.log(wsdn.length)
    console.log(rest.length)
    wsdn.forEach((lp: any) => {
        if (!rest.includes(lp)) {
            rest.push(lp)
        }
    })
    console.log(rest.length)
    fs.writeFileSync('all_lp_addresses.json', JSON.stringify(rest, null, 2))
}

//Get assets from list of wsdn lps
async function getAllAssets() {
    let allAssets: any[] = [];
    const sdnLps = JSON.parse(fs.readFileSync('wsdn_lp_addresses.json', 'utf8'))
    await Promise.all(sdnLps.map(async (lp: any) => {
        let dexContract;
        try {
            dexContract = new ethers.Contract(lp, altDexContractAbi, provider);
        } catch (err) {
            dexContract = new ethers.Contract(lp, dexContractAbi, provider);
            console.log("Other dex contract")
        }
        let token0 = await dexContract.token0();
        let token1 = await dexContract.token1();
        if (!allAssets.includes(token0)) {
            allAssets.push(token0)
        }
        if (!allAssets.includes(token1)) {
            allAssets.push(token1)
        }
    }))
    console.log(allAssets)
    console.log(allAssets.length)
    fs.writeFileSync('evm_tokens.json', JSON.stringify(allAssets, null, 2))
}

//From list of top wsdn holders, get list of dexs
async function getSdnLiqPools() {
    let data = JSON.parse(fs.readFileSync('wsdn_holders.txt', 'utf8'))
    const tokenHolders = data.map((holder: any) => holder.address)

    console.log(tokenHolders.includes(testContract.toLowerCase()))
    console.log(await isDex(testContract.toLowerCase()))

    let dexs: any = [];
    let nonDexs: any = [];
    let found = await Promise.all(tokenHolders.map(async (address: any) => {
        console.log(address)
        if (await isDex(address)) {
            console.log("addresss is a dex")
            dexs.push(address)
            return address
        } else {
            // console.log("address is not a dex")
            nonDexs.push(address)
        }
    }))
    
    console.log("dexs: " + dexs.length)
    console.log("nonDexs: " + nonDexs.length)

    fs.writeFileSync('wsdn_lp_addresses.json', JSON.stringify(dexs, null, 2));
}

//Get top holders of wsdn
async function getSdnTokenHoldersBlockscout() {
    let tokenHolders: any[] = [];

    // let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=1&offset=50`;
    // const res = await fetch(uri)
    // const answer = await res.json()
    // console.log(answer)

    let uris: any[] = [];
    // uris.push(uri)
    for (var i = 1; i < 20; i++) {
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/shiden/api?module=token&action=getTokenHolders&contractaddress=${wsdn}&page=${page}&offset=${offset}`;
        uris.push(uri);
    }
    const promises: any[] = uris.map(async (uri) => {
        return fetch(uri)
    })

    const results = await Promise.all(promises)
        .then(responses => {
            return Promise.all(responses.map(async response => {
                const res = await response.json()
                console.log(res)
                return res
            }))
        })
    results.map((result: any) => {
        result.result.map((holder: any) => {
            tokenHolders.push(holder)
        })
    })

    fs.writeFileSync('wsdn_holders.txt', JSON.stringify(tokenHolders, null, 2));
}

async function isDex(dexAddress: any): Promise<boolean> {
    const code = await provider.getCode(dexAddress);
    // console.log(code)
    if (code == "0x") {
        // console.log("Not a contract")
        return false;
    } else {
        console.log("YES contract")
        try {

            const potentialContract = new ethers.Contract(dexAddress, altDexContractAbi, provider);
            const name = await potentialContract.name();
            const token_0 = await potentialContract.token0();
            const token_1 = await potentialContract.token1();
            console.log("True")
            console.log(`lp Name: ${name}`)
            console.log(`1: ${token_0} - 2: ${token_1}`)
            return true;
        } catch {
            try {
                const potentialContract = new ethers.Contract(dexAddress, dexContractAbi, provider);
                const name = await potentialContract.name();
                const token_0 = await potentialContract.token0();
                const token_1 = await potentialContract.token1();
                console.log("True")
                console.log(`lp Name: ${name}`)
                console.log(`1: ${token_0} - 2: ${token_1}`)
                return true;
            } catch {
                return false;
            }
        }
    }
}

async function main() {
    // saveLps()
    // getSdnTokenHoldersBlockscout()
    // getSdnLiqPools()
    // getAllAssets()
    // getAllLps()
    // getAllLps2()
    // getDexFromAllAssetHolders()
    // getDexFromAllAssetHolders()
    // combineSdnAndAll()
    saveEvmLps()
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