const { ApiPromise, WsProvider } = require('@polkadot/api');

var apiHelper = require('./api_utils')
// const zenGetter = require('./zenlink.ts')
const zenG = require('./bifrost/zen')
let axios = require('axios')

class LiqPool {
    constructor(chainId, contractAddress, poolAssets, liquidityStats) {
        this.chainId = chainId;
        this.contractAddress = contractAddress;
        this.poolAssets = poolAssets; // [token 1, token 2]
        this.liquidityStats = liquidityStats; //[reserves 1, reserves 2]
    }
}

async function bifrost_kusama() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let assets = await apiHelper.getAssets(api);
    let assetLocations = await apiHelper.getAssetLocations(api);

    // console.log(assetLocations)
    console.log(parachainId)

    console.log(assetLocations)
    console.log("TE")
    let xcAssets = []
    for (index in assetLocations) {
        let xcAsset = {};
        xcAsset.locationData = assetLocations[index];
        // console.log(xcAsset.locationData)
        const tokenId = assetLocations[index]
        const tokenIdKey = Object.keys(assetLocations[index])[0];
        const tokenIdValue = assetLocations[index][tokenIdKey];

        for (i in assets) {
            const assetId = assets[i]["localId"]["NativeAssetId"];
            const assetIdKey = Object.keys(assetId);
            const assetIdValue = assetId[assetIdKey];
            // console.log(`${assetIdKey} :: ${assetIdValue}`)
            if (tokenIdKey == assetIdKey && tokenIdValue == assetIdValue) {
                xcAsset.tokenData = assets[i];
            }
        }
        xcAssets.push(xcAsset)
    }
    // console.log(xcAssets)
    // for (index in xcAssets) {
    //     console.log(xcAssets[index])
    // }
    console.log(xcAssets)
    let xcAssetData = {};
    xcAssetData.parachainId = parachainId;
    xcAssetData.xcAssets = xcAssets;
    console.log(parachainId)
    // let fs = require('fs');
    // fs.writeFileSync('./x_token_data/bnc_k_x_tokens', JSON.stringify(xcAssetData), 'utf8')

}

async function saveAssetsToFile() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let assets = await apiHelper.getAssets(api);
    let assetLocations = await apiHelper.getAssetLocations(api);
    console.log(assets)
    console.log(assetLocations)

    //GET ASSET LOCATION FOR EACH ASSET
    let assetRegistryObjects = assets.map((asset) => {
        // console.log(asset["localId"]["NativeAssetId"])
        let assetIdKey = Object.keys(asset["localId"]["NativeAssetId"])
        let assetIdVal = asset["localId"]["NativeAssetId"][assetIdKey]
        // console.log(assetIdKey)
        // console.log(assetIdVal)
        for (i in assetLocations) {
            let locationId = assetLocations[i][assetIdKey];
            if (locationId == assetIdVal) {
                // console.log("found match")
                // console.log(assetLocations[i])
                let assetRegisterObject = {};
                assetRegisterObject.asset = asset;
                assetRegisterObject.assetLocation = assetLocations[i];
                // console.log(assetRegisterObject);
                return assetRegisterObject;
            }
        }
    })

    let assetRegistry = {};
    assetRegistry.parachainId = parachainId;
    assetRegistry.assetRegistryObjects = assetRegistryObjects;
    // console.log(assetRegistry)
    let fs = require('fs')
    fs.writeFileSync('../bnc/asset_registry', JSON.stringify(assetRegistry), 'utf8');
}

async function zen_assets() {
    let tokens = await zenG.getTokenList();
    console.log(tokens)
}

async function zen_liqPools() {
    let liqPools = await zenG.getLiqPools();
    // console.log(liqPools)
    console.log("TEST")
    let i = 0;
    console.log("POOL LENGTH: " + liqPools.length)
    // while (i < liqPools.length) {
    //     // console.log("Token 0 Reserves: " + standardPools[i].reserve0)
    //     console.log(`RETURNED: Token 0: ${liqPools[i].token0.name} ${liqPools[i].token0.symbol} - Token 1: ${liqPools[i].token1.name} ${liqPools[i].token1.symbol}`)
    //     console.log(`RETURNED: ${liqPools[i].reserve0.numerator.toString()} - ${liqPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
    // console.log("TEST 2")
    liqPools.forEach((pool) => {
        pool.forEach((value) => {
            console.log(`RETURNED: Token 0: ${value.token0.name} ${value.token0.symbol} - Token 1: ${value.token1.name} ${value.token1.symbol}`)
            console.log(`RETURNED: ${value.reserve0.numerator.toString()} - ${value.reserve1.numerator.toString()}`)
        })
    })
}

//LINK THE BNC ASSETS WITH THEIR CORRESPONDING ZENLINK ASSET

async function mergeAssets() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let bncAssets = await apiHelper.getAssets(api);

    // let bifrostAssets = apiHelper.getAssets();
    for (i in bncAssets) {
        console.log(bncAssets[i])
    }
    let mergedAssets = [];
    let zenTokens = await zenG.getTokenList();

    mergedAssets = zenTokens.map((token) => {
        // let mergedAsset;
        if (token["symbol"].toLowerCase() == "ausd".toLowerCase()) {
            console.log("FOUND ausd")
            for (j in bncAssets) {
                let asset = bncAssets[j];
                // console.log(asset["localId"]);
                let assetSymbol = asset["symbol"];
                if (assetSymbol.toLowerCase() == "kusd") {
                    let mergedAsset = new MergedAsset(asset, token);
                    console.log(mergedAsset)
                    // mergedAssets.push(mergedAsset);
                    return mergedAsset
                }
                // assetKey = Object.keys(assetKey);
                // console.log(assetKey)
                // if (asset["localId"] )
            }
        } else if (token["symbol"].toLowerCase() == "vsksm".toLocaleLowerCase()) {
            console.log("FOUND VSKSM")
            for (j in bncAssets) {
                let asset = bncAssets[j];
                // console.log(asset["localId"]);
                let assetKey = asset["localId"]["NativeAssetId"];
                assetKey = Object.keys(assetKey);
                if (assetKey == 'VSToken') {
                    let mergedAsset = new MergedAsset(asset, token);
                    // mergedAssets.push(mergedAsset);
                    console.log(mergedAsset["bncAsset"]["localId"])
                    console.log(mergedAsset)
                    return mergedAsset
                }
            }
        } else if (token["symbol"].toLowerCase() == "vksm".toLocaleLowerCase()) {
            console.log("vksm")
            return
        } else {
            for (j in bncAssets) {
                let asset = bncAssets[j];
                // console.log(asset)
                let assetSymbol = asset["symbol"];
                let tokenSymbol = token["symbol"];
                if (assetSymbol.toLowerCase() === tokenSymbol.toLowerCase()) {
                    let mergedAsset = new MergedAsset(asset, token)
                    // mergedAssets.push(mergedAsset)
                    console.log(mergedAsset)
                    return mergedAsset
                }
                // console.log(asset["localId"]);
                // let assetKey = asset["localId"]["NativeAssetId"];
                // assetKey = Object.keys(assetKey);
                // console.log(assetKey)
            }
        }
        
    })

    
    mergedAssets = mergedAssets.splice(0, 6)
    console.log(mergedAssets.length)
    for (i in mergedAssets) {
        let asset = mergedAssets[i];
        console.log("Test")
        console.log(asset["bncAsset"]["localId"])
    }

    let fs = require('fs');
    // fs.writeFileSync('../bnc/merged_assets', JSON.stringify(mergedAssets), 'utf8')
}

class MergedAsset{
    constructor(bncAsset, zlkAsset) {
        this.bncAsset = bncAsset;
        this.zlkAsset = zlkAsset;
    }
}

exports.saveZenLiqPools = async () => {
    let liqPoolObjs = [];
    //Get zenlink pools
    let liqPools = await zenG.getLiqPools();
    // console.log(liqPools)
    await liqPools.forEach(async (liqPoolResponse) => {
        // let liqPoolObjs = [];
        // console.log(liqPoolResponse)
        await liqPoolResponse.forEach(async (pool) => {
            // console.log(pool)
            console.log(`RETURNED: Token 0: ${pool.token0.name} ${pool.token0.symbol} - Token 1: ${pool.token1.name} ${pool.token1.symbol}`)
            console.log(`RETURNED: ${pool.reserve0.numerator.toString()} - ${pool.reserve1.numerator.toString()}`)

            let token0 = await zenToBnc(pool.token0);
            let token1 = await zenToBnc(pool.token1);
            let reserve0 = pool.reserve0.numerator.toString();
            let reserve1 = pool.reserve1.numerator.toString();
            if (token0 != undefined && token1 != undefined) {
                let tokens = [token0["bncAsset"]["localId"], token1["bncAsset"]["localId"]]
                let liquidity = [reserve0, reserve1]
                let liqPool = new LiqPool("2001", "None", tokens, liquidity)
                liqPoolObjs.push(liqPool);
                console.log("pool")
            }
        })
        console.log(await liqPoolObjs)
        // console.log(liqPoolObjs)
        let fs = require('fs');
        fs.writeFileSync('../bnc/liq_pool_registry', JSON.stringify(liqPoolObjs), 'utf8')
    })
    
    // let i = 0;
    // let liqPoolsInner = liqPools[0];
    // console.log(liqPoolsInner);
    // liqPoolsInner.forEach(async (value) => {
    //     console.log(`RETURNED: Token 0: ${value.token0.name} ${value.token0.symbol} - Token 1: ${value.token1.name} ${value.token1.symbol}`)
    //     console.log(`RETURNED: ${value.reserve0.numerator.toString()} - ${value.reserve1.numerator.toString()}`)

    //     let token0 = await zenToBnc(value.token0);
    //     let token1 = await zenToBnc(value.token1);
    //     let reserve0 = value.reserve0.numerator.toString();
    //     let reserve1 = value.reserve1.numerator.toString();
    //     if (token0 != undefined && token1 != undefined) {
    //         let tokens = [token0["bncAsset"]["localId"], token1["bncAsset"]["localId"]]
    //         let liquidity = [reserve0, reserve1]
    //         let liqPool = new LiqPool("2001", "None", tokens, liquidity)
    //         liqPoolObjs.push(liqPool);
    //         console.log("pool")
    //     }
    // });

    //Match the zenlink assets to the merged ZLK/BNC assets and create liqPool objects
    // await liqPools.forEach(async (pool) => {
    //     // console.log("TEST")
    //     await pool.forEach(async (value) => {
    //         console.log(`RETURNED: Token 0: ${value.token0.name} ${value.token0.symbol} - Token 1: ${value.token1.name} ${value.token1.symbol}`)
    //         console.log(`RETURNED: ${value.reserve0.numerator.toString()} - ${value.reserve1.numerator.toString()}`)

    //         let token0 = await zenToBnc(value.token0);
    //         let token1 = await zenToBnc(value.token1);
    //         let reserve0 = value.reserve0.numerator.toString();
    //         let reserve1 = value.reserve1.numerator.toString();
    //         if (token0 != undefined && token1 != undefined) {
    //             let tokens = [token0["bncAsset"]["localId"], token1["bncAsset"]["localId"]]
    //             let liquidity = [reserve0, reserve1]
    //             let liqPool = new LiqPool("2001", "None", tokens, liquidity)
    //             liqPoolObjs.push(liqPool);
    //             console.log("pool")
    //         }
    //     })
    //     console.log(liqPoolObjs)
    // })
    
    
    
    // liqPoolObjs.forEach((pool) => {
    //     console.log(pool)
    //     console.log("test2")
    // })
    // let fs = require('fs');  
    // fs.writeFileSync('../bnc/liq_pool_registry', JSON.stringify(liqPoolObjs), 'utf8')
}

//Find the corresponding merged BNC/ZLK asset
async function zenToBnc(token) {
    let fs = require('fs');
    let mergeAssets = fs.readFileSync('./bifrost/merged_assets', 'utf8');
    mergeAssets = JSON.parse(mergeAssets);
    // console.log(mergeAssets)
    // console.log("Token: ")
    // console.log(token)
    for (i in mergeAssets){
        let mergeA = mergeAssets[i].zlkAsset;
        if (token.meta.assetType == mergeA.meta.assetType && token.meta.assetIndex == mergeA.meta.assetIndex) {
            // console.log("Found match")
            // console.log(mergeAssets[i])
            return mergeAssets[i]
        }

    }
}

async function printMergeAssets() {
    let fs = require('fs');
    let mergeAssets = fs.readFileSync('../bnc/merged_assets', 'utf8');
    mergeAssets = JSON.parse(mergeAssets);
    // console.log(mergeAssets)
    for (i in mergeAssets) {
        let m = mergeAssets[i];
        console.log("bnc");
        console.log(m.bncAsset["localId"])
        console.log(m.bncAsset)
        console.log("zlk")
        console.log(m.zlkAsset.meta)
        console.log(m.zlkAsset)
    }
}
async function main() {
    // zen()
    // zen_liqPools();
    // bifrost_kusama();
    // mergeAssets()
    saveZenLiqPools()
    // saveAssetsToFile();
    // printMergeAssets()
    // getResponse()

}
// main().then(() => console.log("Complete"))

async function getResponse() {
    const API_URL = 'https://api.zenlink.pro/rpc';  

    const response = await axios.post(API_URL, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "asset.get",
        "params": []
    });

    const allTokens = response.data.result || [];
    // const sdkTokens = allTokens
    //     .filter((token) => token.isNative === Native.P || token.isNative === Native.N)
    //     .map((token) => {
    //         let networkId = null;
    //         const { chainId, assetType, assetIndex, symbol, decimals, name, account } = token;
    //         networkId = CHAINID_TO_NETWORKID(chainId)

    //         return {
    //             networkId,
    //             address: account,
    //             chainId,
    //             assetType,
    //             assetIndex,
    //             symbol,
    //             decimals,
    //             name,
    //         };
    //     });
    
    console.log(allTokens)
}

// curl -H "Content-Type: application/json" http://localhost:11111 -d '{"jsonrpc": "2.0","id": 1,"method": "zenlinkProtocol_getSovereignsInfo","params": [{ "chain_id": 200, "asset_type": 0, "asset_index": 0 }]}'

// curl -H 'Content-Type: application/json' http://localhost:11111 -d "{"jsonrpc": '2.0','id': 1,'method': 'zenlinkProtocol_getSovereignsInfo','params': [{ 'chain_id': 200, 'asset_type': 0, 'asset_index': 0 }]}"