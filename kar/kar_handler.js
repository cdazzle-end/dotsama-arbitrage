const { ApiPromise, WsProvider } = require('@polkadot/api');
const { options } = require('@acala-network/api');
const apiHelper = require('../parachains/api_utils')
const nativeAssets = require('./native_assets.js')
const calculator = require('./calculator.js');
// const [decimals, setDecimals] = useState();

//swap formula
// Fee = 0.3%
// Bo = y - ( y * x / ( x + Ai )) = y * Ai / ( x + Ai )
// Slip = Bo / y
//decimals: ksm 12 kusd 12 rmrk 10

async function main() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    // const rmrkCurrencyId = { ForeignAssetId: 0 };
    // const ksmCurrencyId = { NativeAssetId: { Token: "KSM" } };
    // const kusdCurrencyId = { NativeAssetId: { Token: "KUSD" } };

    // const ksm = await api.query.assetRegistry.assetMetadatas(ksmCurrencyId);
    // const rmrk = await api.query.assetRegistry.assetMetadatas(rmrkCurrencyId);
    // const kusd = await api.query.assetRegistry.assetMetadatas(kusdCurrencyId);
    // console.log(ksm.toHuman())
    // console.log(rmrk.toHuman())
    // console.log(kusd.toHuman())

    // const ksmRmrkPool = await api.query.dex.liquidityPool([{ Token: "KSM" }, { ForeignAsset: 0 }]);
    // const rmrkPerKsm = (ksmRmrkPool[1].toString() / 10 ** 10) /
    //     (ksmRmrkPool[0].toString() / 10 ** 12);

    // console.log(rmrkPerKsm)

    // get_assets(api);
    // get_all_pools(api);
    // save_dex_to_file(api);
    // save_assets_to_file(api);

    //Get liquidity pool numbers
    //Rmrk <-> Kusd <-> Ksm
    const kusdRmrkPool = await api.query.dex.liquidityPool([{ Token: "KUSD" }, { ForeignAsset: 0 }]);
    // const kusdKsmPool = await api.query.dex.liquidityPool([{ Token: "KUSD" }, { Token: "KSM"}]);
    console.log("kusd/rmrk")
    console.log(kusdRmrkPool.toHuman())
    // console.log("kusd/ksm")
    // console.log(kusdKsmPool.toHuman())

    const kusdLiquidity = kusdRmrkPool[0].toString() / 10 ** 12;
    const rmrkLiquidity = kusdRmrkPool[1].toString() / 10 ** 10;
    calculator.rmrk_to_kusd(kusdLiquidity, rmrkLiquidity, 15);
    // // calculate_price_1(kusdLiquidity, rmrkLiquidity);
    // // console.log("|||||")
    // // calculate_price_2(kusdLiquidity, rmrkLiquidity);

    // const kusdL2 = kusdKsmPool[0].toString() / 10 ** 12;
    // const ksmL2 = kusdKsmPool[1].toString() / 10 ** 12

    // // calculate_ksm_to_kusd(kusdL2, ksmL2, 10);
    // // calculate_kusd_to_ksm(kusdL2, ksmL2, 300)

    // calculate_rmrk_to_ksm(kusdRmrkPool, kusdKsmPool, 100);
    // read_asset_registry()
    // read_liq_pools();
    save_dex_to_file()
    // get_assets();
    // queryAssets(api)
    // saveAssetsToFile(api)
    // read_asset_registry()
    // getNativeAssets()
    // saveAssetsToFile2()
}

async function calculateSwap() {
    // calculator.rmrk_to_kusd()
}

async function read_asset_registry() {
    let fs = require('fs');
    let asset_registry = fs.readFileSync('./asset_registry', 'utf8');
    asset_registry = JSON.parse(asset_registry);
    
    // console.log(asset_registry.assetRegistryObjects)
    asset_registry.assetRegistryObjects.forEach((asset) => {
        console.log(asset["asset"]["localId"])
        console.log(asset)
    })
}

async function read_liq_pools() {
    let fs = require('fs');
    let liqPools = fs.readFileSync('./liq_pool_registry', 'utf8');
    liqPools = JSON.parse(liqPools);
    // console.log(liqPools)
    liqPools.map((liqPool) => {
        console.log(liqPool)
        // console.log(liqPool.poolAssets)
        // console.log(liqPool.liquidityStats)
    })
}

// main().then(() => console.log("Complete"))

async function get_assets(api) {
    //get the whole map
    const assetRegistry = await api.query.assetRegistry.assetMetadatas.entries();
    // console.log(assetRegistry)
    assetRegistry.forEach(([key, exposure]) => {
        console.log('key arguments:', key.args.map((k) => k.toHuman()));
        console.log('     exposure:', exposure.toHuman());
    });
}

async function queryAssets(api) {
    let assets = await apiHelper.getAssets(api);
    console.log(assets)
    // assets.map((asset) => {
    //     console.log(asset)
    // })
}


//USE THIS ONE
async function saveAssetsToFile2() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let assets = await apiHelper.getAssets(api);
    let assetLocations = await apiHelper.getAssetLocations(api);
    // console.log(assets.length)
    // console.log(assetLocations.length)

    //GET ASSET LOCATION FOR EACH ASSET
    let assetRegistryObjects = [];
    await assets.forEach(async (asset) => {
        // console.log(asset["localId"])
        let assetIdType = Object.keys(asset["localId"]);
        if (assetIdType == "ForeignAssetId") {
            let assetIdVal = asset["localId"]["ForeignAssetId"]
            for (i in assetLocations) {
                let locationId = assetLocations[i]["ForeignAsset"];
                if (locationId == assetIdVal) {
                    // console.log("found match")
                    // console.log(assetLocations[i])
                    let assetRegistryObject = {};
                    assetRegistryObject.asset = asset;
                    assetRegistryObject.assetLocation = assetLocations[i];
                    console.log(assetRegistryObject);
                    assetRegistryObjects.push(assetRegistryObject);
                    // console.log()
                    
                }
            }
        }

        //GET NATIVE ASSETS TOO
        if (assetIdType == "NativeAssetId") {
            let assetIdKey = Object.keys(asset["localId"]["NativeAssetId"])
            let assetIdVal = asset["localId"]["NativeAssetId"][assetIdKey]
            // console.log(asset)
            let assetLocation = await nativeAssets.getNativeAsset(assetIdVal);
            let assetRegistryObject = {};
            assetRegistryObject.asset = asset;
            assetRegistryObject.assetLocation = assetLocation;
            // console.log(assetRegistryObject);
            // console.log(asset)
            assetRegistryObjects.push(assetRegistryObject);
            // console.log("as")
        }
        // console.log(assetIdType)
        if (assetIdType == "StableAssetId") {
            // console.log(assetIdType)
            let assetIdKey = Object.keys(asset["localId"]["StableAssetId"])
            let assetIdVal = asset["localId"]["StableAssetId"][assetIdKey]
            // console.log(asset)
            // console.log(assetIdKey)
            // console.log(assetIdVal)
            let assetLocation = await nativeAssets.getStableAsset(assetIdVal);
            // console.log(assetLocation)
            let assetRegistryObject = {};
            assetRegistryObject.asset = asset;
            assetRegistryObject.assetLocation = assetLocation;
            // console.log(assetRegistryObject);
            // console.log(asset)
            assetRegistryObjects.push(assetRegistryObject);
            // console.log("as")
        }
    })

    


    let assetRegistry = {};
    assetRegistry.parachainId = parachainId;
    assetRegistry.assetRegistryObjects = assetRegistryObjects;
    // console.log(assetRegistry.assetRegistryObjects.length)
    let fs = require('fs')
    fs.writeFileSync('./asset_registry', JSON.stringify(assetRegistry), 'utf8');
}

class assetLocation{
    constructor(parents, location) {
        this.parents = parents;
        this.location = location;
    }
}
async function saveAssetsToFile(api) {
    const parachainId = await (await api.query.parachainInfo.parachainId()).toHuman();
    let assets = await apiHelper.getAssets(api);
    let assetLocations = await apiHelper.getAssetLocations(api);

    console.log(assets)
    console.log(assetLocations)

    let assetRegisterObjects = assets.map((asset) => {
        let localId = asset["localId"];
        let localIdKey = Object.keys(localId);
        let localIdVal = localId[localIdKey]
        console.log(localId)
        console.log(localIdKey)
        console.log(localIdVal)
        if (localIdKey == "NativeAssetId") {
            let nativeAssetIdKey = Object.keys(localIdVal);
            let nativeAssetIdVal = localIdVal[nativeAssetIdKey];
            for (i in assetLocations) {
                let locationId = assetLocations[i]
            }
            // console.log(nativeAssetIdKey);
            // console.log(nativeAssetIdVal)
        }
    })
}

async function save_assets_to_file(api) {
    const assetRegistry = await api.query.assetRegistry.assetMetadatas.entries();
    // let k =[];
    // let e = [];
    let tokens = [];
    assetRegistry.forEach(([key, entry]) => {
        console.log("Key: ", key.args.length);
        // console.log("Entry: ", entry.valueOf());
        let k = key.args.map((k) => k.toHuman());
        let e = entry.toHuman();
        let token = {asset_key: k, asset_data: e}
        tokens.push(token)
    });

    // console.log(tokens.toString())
    // let assetsFile = {
    //     keys: k,
    //     assets: e
    // }

    const jsonAssets = JSON.stringify(tokens);
    console.log(jsonAssets)
    var fs = require('fs');

    // fs.writeFile('kar_asset_registry_2', jsonAssets, function (err) {
    //     if (err) throw err;
    //     console.log('Saved!');
    // });

}

async function get_all_pools(api) {
    //get the whole map
    const assetRegistry = await api.query.dex.liquidityPool.entries();

    assetRegistry.forEach(([key, exposure]) => {
        if (key.args.toString().includes("{\"token\":\"KSM\"}")) {
            console.log(key.args.toString())
        }

    });
}

exports.save_dex_to_file = async () => {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const fs = require('fs');
    //make sure file can be reached from exported function
    let assetRegistry = fs.readFileSync('../kar/asset_registry')
    assetRegistry = JSON.parse(assetRegistry);

    // console.log(assetRegistry)
    let assets = assetRegistry.assetRegistryObjects;

    const liqPools = await api.query.dex.liquidityPool.entries();

    // liqPools.forEach(([key, exposure]) => {
    // });
    let liqPoolObjects = [];
    await liqPools.forEach(async (liqPool) => {
        let tokens = liqPool[0].toHuman()[0];
        let liquidity = liqPool[1].toHuman();
        // console.log(tokens)
        tokens = await tokens.map((token) => {
            // console.log(token)
            // console.log(tokenIdKey)
            let idKey = Object.keys(token)
            let idVal = token[idKey]
            // console.log(idKey)
            let assetMatchId;
            let rAsset;
            switch (idKey[0]) {
                case 'Token':
                    assetMatchId = { NativeAssetId: { Token: idVal } };
                    assets.forEach((asset) => {
                        let assetIdKey = Object.keys(asset["asset"]["localId"])[0];
                        let assetIdValue = asset["asset"]["localId"][assetIdKey];
                        let assetIdValueKey = Object.keys(assetIdValue)[0]
                        let assetIdValueKeyValue = assetIdValue[assetIdValueKey];
                        if (assetIdKey === "NativeAssetId" && assetIdValueKey === "Token" && assetIdValueKeyValue === idVal) {
                            // console.log(asset)
                            rAsset = asset["asset"]["localId"]
                        }
                    })
                    // console.log("match:")
                    // console.log(assetMatchId)
                    break;
                case 'ForeignAsset':
                    assetMatchId = { ForeignAssetId: idVal }
                    // console.log(assetMatchId)
                    assets.forEach((asset) => {
                        let assetIdKey = Object.keys(asset["asset"]["localId"])[0];
                        let assetIdValue = asset["asset"]["localId"][assetIdKey];
                        if (assetIdKey === "ForeignAssetId" && assetIdValue === idVal) {
                            // console.log(asset)
                            rAsset = asset["asset"]["localId"]
                        }
                    })
                    break;
                case 'StableAssetPoolToken':
                    assetMatchId = { StableAssetId: idVal };
                    // console.log(assetMatchId)
                    assets.forEach((asset) => {
                        let assetIdKey = Object.keys(asset["asset"]["localId"])[0];
                        let assetIdValue = asset["asset"]["localId"][assetIdKey];
                        if (assetIdKey === "StableAssetId" && assetIdValue === idVal) {
                            // console.log(asset)
                            rAsset = asset["asset"]["localId"]
                        }
                    })
                    break;
                default:
                    console.log("COULD NOT MATCH")
                    // assetMatchId = "NOTHIN"
            }

            return rAsset;
        })
        // console.log(tokens)
        liquidity = liquidity.map((liq) => {
            return liq.replace(/,/g, '')
        })
        // console.log(liquidity)

        let liqPoolObj = new LiqPool("2000", "None", tokens, liquidity)
        liqPoolObjects.push(liqPoolObj)
        
    })
    console.log(liqPoolObjects)
    
    //Make sure write to the proper location from export
    fs.writeFileSync('../kar/liq_pool_registry', JSON.stringify(liqPoolObjects))
}
//TO DO
//find the best path to trade out of all token pools
function find_all_paths(tokenA, tokenB, liqPools) {

}

    class LiqPool {
        constructor(chainId, contractAddress, poolAssets, liquidityStats) {
            this.chainId = chainId;
            this.contractAddress = contractAddress;
            this.poolAssets = poolAssets; // [token 1, token 2]
            this.liquidityStats = liquidityStats; //[reserves 1, reserves 2]
        }
    }