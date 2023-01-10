// 1. Import ethers
const ethers = require('ethers');
const { parse } = require('path');
const { formatUnits } = require('ethers/lib/utils');
const hexConversion = require('../parachains/hex.js')
const { ApiPromise, WsProvider } = require('@polkadot/api');
const apiHelper = require('../parachains/api_utils')

class LiqPool {
    constructor(chainId, contractAddress, poolAssets, liquidityStats) {
        this.chainId = chainId;
        this.contractAddress = contractAddress;
        this.poolAssets = poolAssets; // [token 1, token 2]
        this.liquidityStats = liquidityStats; //[reserves 1, reserves 2]
    }
}

// 2. Define network configurations
const providerRPC = {
    moonriver: {
        name: 'moonriver',
        rpc: 'https://moonriver.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 1285, // 0x505 in hex,
    },
};
// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.moonriver.rpc,
    {
        chainId: providerRPC.moonriver.chainId,
        name: providerRPC.moonriver.name,
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

async function getMovrTokenHoldersBlockscout() {
    let tokenHolders = [];
    for (i = 1; i < 20; i++) {
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        const res = await fetch(uri)
        const answer = await res.json()
        console.log(answer.result.length)
        for (j = 0; j < answer.result.length; j++) {
            let holderAddress = answer.result[j].address;
            tokenHolders.push(holderAddress)
            // console.log(holderAddress)
        }
    }
    console.log(tokenHolders)
    console.log(tokenHolders.length)

    fs.writeFileSync('wmovr.txt', tokenHolders.toString(), 'utf8');

    let tokenUris = [];
    // let tokenPromises =[]
    for (i = 1; i < 20; i++){
        let page = i;
        let offset = 50;
        
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        tokenUris.push(uri);
        // const res = await fetch(uri)
        // const answer = await res.json()
        // console.log(answer.result.length)
    }
    let tokenRequests = tokenUris.map(uri => {
        return fetch(uri).then((val) => val)
    })
}

async function getPromises() {
    let promises = await testPromises();
    console.log(promises)
}

async function testWithoutPromises() {
    let tokenUris = [];
    // let tokenPromises =[]
    for (i = 1; i < 10; i++) {
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98&page=${page}&offset=${offset}`;
        // let uri = `https://blockscout.moonriver.moonbeam.network/api?module=token&action=getTokenHolders&contractaddress=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98&page=${page}&offset=${offset}`;
        // tokenUris.push(uri);
        const res = await fetch(uri)
        const answer = await res.json()
        console.log(answer.result)

    }
}

async function testPromises() {
    let tokenUris = [];
    // let tokenPromises =[]
    for (i = 1; i < 10; i++) {
        let page = i;
        let offset = 50;
        // let uri = `https://blockscout.moonriver.moonbeam.network/api?module=token&action=getTokenHolders&contractaddress=0x5D9ab5522c64E1F6ef5e3627ECCc093f56167818&page=${page}&offset=${offset}`;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98&page=${page}&offset=${offset}`;
        tokenUris.push(uri);
    }
    let tokenRequests = tokenUris.map(uri => {
        console.log("fetching uri: " + uri)
        let response = fetch(uri);
        return response.then(val => {
            console.log("fetch uri complete")
            let js = val.json().then(val => {
                // console.log(val)
                return val;
            });
            // console.log(js)
            return js
        });

    });

    return Promise.all(tokenRequests).then(results => {
        results.map((res) => {
            // console.log(res)
        });
        return results
    })
    // console.log(tokenRequests)

    // return Promise.all(tokenRequests).then(result => {
    //     return tokenUris.filter((_, i) => result[i])
    // })
    // console.log("req: " + tokenRequests)
    // let promiseRes = Promise.all(tokenRequests).then(val => {
    //     let jarray = [];
    //     for (i = 0; i < val.length; i++){
    //         jarray.push(val[i])
    //         console.log(val[i])
    //     }
        // console.log(jarray)
        // return Promise.all(jarray).then((val => {
        //     // console.log(val[0])
        //     for (i = 0; i < val.length; i++) {
        //         console.log(val[i])
        //     }
        //     console.log(val.length)
        // }));
    // })
    // console.log("PR: " + promiseRes)
}

async function readTokenRegistry() {
    let fs = require('fs');
    let assetRegistry = fs.readFileSync('token_registry.txt', 'utf8');
    assetRegistry = JSON.parse(assetRegistry);
    console.log(assetRegistry);
}
async function readAssetRegistry() {
    let fs = require('fs');
    let assetRegistry = fs.readFileSync('asset_registry', 'utf8');
    assetRegistry = JSON.parse(assetRegistry);
    assetRegistry.assetRegistryObjects.forEach((asset) => {
        console.log(asset.assetLocation.location)
        console.log(asset)
    })
    // console.log(assetRegistry.assetRegistryObjects);
}

async function readLiqPools() {
    let fs = require('fs');
    let liqPools = fs.readFileSync('liq_pool_registry', 'utf8');
    liqPools = JSON.parse(liqPools);
    console.log(liqPools);
}

//Go through all LiqPools, query the blockchain with the pool contract address and save the current liquidity
exports.updateLiqPools = async () => {
    let fs = require('fs');
    let liqPools = fs.readFileSync('../movr/liq_pool_registry', 'utf8');
    liqPools = JSON.parse(liqPools);
    // let newPools2 = [];
    // let newLiqPools = [];

    let newLiqPools = await Promise.all(liqPools.map(async(pool) => {
        let poolAddress = pool["contractAddress"];
        try {
            const poolContract = await new ethers.Contract(poolAddress, altDexContractAbi, provider);
            let reserves = await poolContract.getReserves();
            let reserve_0 = hexConversion.hexToDec(reserves[0]["_hex"]);
            let reserve_1 = hexConversion.hexToDec(reserves[1]["_hex"]);
            let newliquidityStats = [reserve_0, reserve_1];
            let newPool = new LiqPool("2023", poolAddress, pool["poolAssets"], newliquidityStats);
            return newPool;
        } catch (err) {
            console.log("Try alternate contract")
            try {
                const poolContract = await new ethers.Contract(poolAddress, dexexContractAbi, provider);
                let reserves = await poolContract.getReserves();
                let reserve_0 = hexConversion.hexToDec(reserves[0]["_hex"]);
                let reserve_1 = hexConversion.hexToDec(reserves[1]["_hex"]);
                let newliquidityStats = [reserve_0, reserve_1];
                let newPool = new LiqPool("2023", poolAddress, pool["poolAssets"], newliquidityStats);
                return newPool;
            } catch (err) {
                console.log("Could not parse contract")
            }
        }
    }))


    console.log(newLiqPools)
    fs.writeFileSync("../movr/liq_pool_registry", JSON.stringify(newLiqPools), 'utf8')
}

async function main() {
    // readAssetRegistry()
    // readTokenRegistry()
    readLiqPools()
    // updateLiqPools();
    // rewriteLiqpools()
    // saveAssetsToFile()
    // assetToToken()
    
}

async function rewriteLiqpools() {
    let fs = require('fs');
    let liqPools = fs.readFileSync('liq_pool_registry', 'utf8');
    liqPools = JSON.parse(liqPools);
    let chainId = "2023";
    let newLiqPools = [];
    liqPools.forEach((pool) => {
        let contractAddress = pool["address"];
        let token0 = pool["token_1"]["address"];
        let token1 = pool["token_2"]["address"]
        let hex0 = pool["reserves"][0]["hex"]
        let hex1 = pool["reserves"][1]["hex"]
        hex0 = hexConversion.hexToDec(hex0)
        hex1 = hexConversion.hexToDec(hex1)
        // console.log(`${token0} -- ${token1}`)
        // console.log(`${hex0} -- ${hex1}`)
        let newPool = new LiqPool(chainId, contractAddress, [token0, token1], [hex0, hex1])
        newLiqPools.push(newPool);
        // console.log(pool);
    })
    console.log(newLiqPools);
    fs.writeFileSync('liq_pool_registry_new', JSON.stringify(newLiqPools))
}

async function movr_addresses() {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    let xcAssets = await api.query.assets.asset.entries();
    xcAssets.map(([key, val]) => {
        console.log(key.toHuman())
        // console.log(val.toHuman())
        let num = key.toHuman()[0].replace(/,/g, '')
        console.log(num)
        console.log(parseInt(num))
        // let hex = decToHex(parseInt(num))
        let hex2 = hexConversion.decToHex(num);

        // console.log(hex)
        console.log(hex2)
        hex2 = hex2.slice(2)
        console.log(hex2)
        // console.log("0xFFFFFFFF" + hex)
        console.log("0xFFFFFFFF" + hex2)
    })

    //0xFFFFFFFFF075423BE54811ECB478E911F22DDE7D

}

// async function 

async function saveAssetsToFile() {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let assets = await apiHelper.getAssets2(api);
    let assetLocations = await apiHelper.getAssetLocations2(api);
    // console.log(assets.length)
    // console.log(assetLocations.length)
    assetLocations.forEach((location) => {
        // console.log(location)
    })

    let assetRegistryObjects = [];
    assets.forEach(async (asset) => {
        // console.log(asset.name)
        // console.log(asset.localId)
        let assetId = asset.localId
        assetLocations.forEach(async (assetLocation) => {
            locationId = assetLocation.id;
            // console.log(locationId)
            if (assetId === locationId) {
                console.log(locationId)
                let assetRegisteryObject = {};
                assetRegisteryObject.asset = asset;

                let num = assetId.replace(/,/g, '')
                let hex = hexConversion.decToHex(num);
                hex = hex.slice(2)

                //LEADING ZERO GETS REMOVED, SO ADD IT BACK IN
                if (hex.length === 31) {
                    hex = "0" + hex
                }
                

                let evmAddress = "0xFFFFFFFF" + hex;
                assetRegisteryObject.asset.evmAddress = evmAddress;

                assetRegisteryObject.assetLocation = assetLocation;
                // console.log(assetRegisteryObject)
                assetRegistryObjects.push(assetRegisteryObject);
            }
        })
        

    })
    console.log(assetRegistryObjects)
    let assetRegistry = {};
    assetRegistry.parachainId = parachainId;
    assetRegistry.assetRegistryObjects = assetRegistryObjects;
    // console.log(assetRegistry)
    // let fs = require('fs')
    // fs.writeFileSync('asset_registry', JSON.stringify(assetRegistry), 'utf8')
}

async function assetToToken() {
    let fs = require('fs');
    let assets = fs.readFileSync('asset_registry', 'utf8')
    assets = JSON.parse(assets);
    const rmrkAdd = "0xFfffffFF893264794d9d57E1E0E21E0042aF5A0A".toLocaleLowerCase();

    assets.assetRegistryObjects.forEach(async(asset) => {
        console.log(asset)
        let evmAddress = asset.asset.evmAddress;
        try {
            const contract = await new ethers.Contract(evmAddress.toLocaleLowerCase(), tokenContractAbi, provider);
            console.log(await contract.name())
            console.log(await contract.symbol())
        } catch (err) {
            console.log("Couldnt make contract")
            console.log(err)
        }
    })
}

// main()

