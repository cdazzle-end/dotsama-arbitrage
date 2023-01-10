const { ApiPromise, WsProvider } = require('@polkadot/api');
const fs = require('fs');

//GET ASSET REGISTRY

//GET ZENLINK LIQ POOL DATA
async function main(){
    readLiqPools();
    // rewriteLiqPools()
    // readAssetRegistry();
}

class LiqPool {
    constructor(chainId, contractAddress, poolAssets, liquidityStats) {
        this.chainId = chainId;
        this.contractAddress = contractAddress;
        this.poolAssets = poolAssets; // [token 1, token 2]
        this.liquidityStats = liquidityStats; //[reserves 1, reserves 2]
    }
}

main()

async function readAssetRegistry() {
    let assetRegistry = fs.readFileSync('./asset_registry', 'utf8');
    assetRegistry = JSON.parse(assetRegistry);
    // console.log(assetRegistry["assetRegistryObjects"]);
    assetRegistry["assetRegistryObjects"].map((assetObj) => {
        console.log(assetObj["asset"]["localId"])
        console.log(assetObj)
    })
}

async function readLiqPools() {
    let liqPools = fs.readFileSync('./liq_pool_registry_new', 'utf8');
    liqPools = JSON.parse(liqPools);
    liqPools.map((liqPool) => {
        // console.log(liqPool.poolAssets)
        console.log(liqPool)
    })
    // console.log(liqPools)
}

async function rewriteLiqPools(){
    let liqPools = fs.readFileSync('./liq_pool_registry', 'utf8');
    liqPools = JSON.parse(liqPools);
    let newPools = [];
    liqPools.map((liqPool) => {
        console.log(liqPool.poolAssets)
        console.log(liqPool.liquidityStats)
        let pool = new LiqPool("2001", "None", liqPool.poolAssets, liqPool.liquidityStats);
        newPools.push(pool)
    })

    newPools.forEach((pool) => {
        console.log(pool)
    })
    fs.writeFileSync("./liq_pool_registry_new", JSON.stringify(newPools))
    // console.log(liqPools)
}