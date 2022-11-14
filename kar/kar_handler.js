const { ApiPromise, WsProvider } = require('@polkadot/api');
const { options } = require('@acala-network/api');
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

    const rmrkCurrencyId = { ForeignAssetId: 0 };
    const ksmCurrencyId = { NativeAssetId: { Token: "KSM" } };
    const kusdCurrencyId = { NativeAssetId: { Token: "KUSD" } };

    const ksm = await api.query.assetRegistry.assetMetadatas(ksmCurrencyId);
    const rmrk = await api.query.assetRegistry.assetMetadatas(rmrkCurrencyId);
    const kusd = await api.query.assetRegistry.assetMetadatas(kusdCurrencyId);
    console.log(ksm.toHuman())
    console.log(rmrk.toHuman())
    console.log(kusd.toHuman())

    const ksmRmrkPool = await api.query.dex.liquidityPool([{ Token: "KSM" }, { ForeignAsset: 0 }]);
    const rmrkPerKsm = (ksmRmrkPool[1].toString() / 10 ** 10) /
        (ksmRmrkPool[0].toString() / 10 ** 12);

    console.log(rmrkPerKsm)

    // get_assets(api);
    // get_all_pools(api);
    save_dex_to_file(api);
    // save_assets_to_file(api);

    //Get liquidity pool numbers
    //Rmrk <-> Kusd <-> Ksm
    // const kusdRmrkPool = await api.query.dex.liquidityPool([{ Token: "KUSD" }, { ForeignAsset: 0 }]);
    // const kusdKsmPool = await api.query.dex.liquidityPool([{ Token: "KUSD" }, { Token: "KSM"}]);
    // console.log("kusd/rmrk")
    // console.log(kusdRmrkPool.toHuman())
    // console.log("kusd/ksm")
    // console.log(kusdKsmPool.toHuman())

    // // const kusdLiquidity = kusdRmrkPool[0].toString() / 10 ** 12;
    // // const rmrkLiquidity = kusdRmrkPool[1].toString() / 10 ** 10;
    // // calculate_price_1(kusdLiquidity, rmrkLiquidity);
    // // console.log("|||||")
    // // calculate_price_2(kusdLiquidity, rmrkLiquidity);

    // const kusdL2 = kusdKsmPool[0].toString() / 10 ** 12;
    // const ksmL2 = kusdKsmPool[1].toString() / 10 ** 12

    // // calculate_ksm_to_kusd(kusdL2, ksmL2, 10);
    // // calculate_kusd_to_ksm(kusdL2, ksmL2, 300)

    // calculate_rmrk_to_ksm(kusdRmrkPool, kusdKsmPool, 100);

}

main().then(() => console.log("Complete"))

//15 rmrk -> _ kusd
function calculate_price_1(kusdL, rmrkL) {
    const rmrkSupply = 100;
    // Bo = y - ( y * x / ( x + Ai )) = y * Ai / ( x + Ai )
    const kusdOutput = kusdL * rmrkSupply / (rmrkL + rmrkSupply);
    const slippage = (kusdOutput / kusdL) * kusdOutput;
    const total = kusdOutput - slippage;
    console.log("out kusd: ", kusdOutput)
    console.log("slip: ", slippage)
    console.log("total: ", total);
}

//calculate the slippage in small increments
//use this for accuracy until i figure out calculus
function rmrk_to_kusd(kusdL, rmrkL, rmrkSupply) {
    // const rmrkSupply = 100;
    const increments = rmrkSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let rmrkChangingLiq = rmrkL;
    let totalSlip = 0;
    let totalKusd = 0;
    const rmrkInput = increments - (increments * 0.003)
    while (i < (rmrkSupply / increments)) {
        let kusdOut = kusdChangingLiq * rmrkInput / (rmrkChangingLiq + rmrkInput);
        let slip = (kusdOut / kusdChangingLiq) * kusdOut;
        totalKusd += kusdOut - slip;
        kusdChangingLiq -= kusdOut - slip;
        rmrkChangingLiq += rmrkInput;
        totalSlip += slip;

        i++;
    }
    console.log("out kusd: ", totalKusd)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKusd;
}

function calculate_ksm_to_kusd(kusdL, ksmL, ksmSupply) {
    const increments = ksmSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let ksmChangingLiq = ksmL;
    let totalSlip = 0;
    let totalKusd = 0;
    const ksmInput = increments - (increments * 0.003)
    while (i < (ksmSupply / increments)) {
        let kusdOut = kusdChangingLiq * ksmInput / (ksmChangingLiq + ksmInput);
        let slip = (kusdOut / kusdChangingLiq) * kusdOut;
        totalKusd += kusdOut - slip;
        kusdChangingLiq -= kusdOut - slip;
        ksmChangingLiq += ksmInput;
        totalSlip += slip;
        i++;
    }
    console.log("%d KSM to kusd: ", ksmSupply, totalKusd)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKusd;
}

function calculate_kusd_to_ksm(kusdL, ksmL, kusdSupply) {
    const increments = kusdSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let ksmChangingLiq = ksmL;
    let totalSlip = 0;
    let totalKsm = 0;
    const kusdInput = increments - (increments * 0.003)
    while (i < (kusdSupply / increments)) {
        let ksmOut = ksmChangingLiq * kusdInput / (kusdChangingLiq + kusdInput);
        let slip = (ksmOut / ksmChangingLiq) * ksmOut;
        totalKsm += ksmOut - slip;
        ksmChangingLiq -= ksmOut - slip;
        kusdChangingLiq += kusdInput;
        totalSlip += slip;
        i++;
    }
    console.log("%d KUSD to KSM: ", kusdSupply, totalKsm)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKsm;
}

//rmrk -> kusd -> ksm
function calculate_rmrk_to_ksm(rmrkPool, ksmPool, rmrkSupply) {
    // const kusdLiquidity = rmrkPool[0].toString() / 10 ** 12;
    // const rmrkLiquidity = rmrkPool[1].toString() / 10 ** 10;
    const kusdMiddle = rmrk_to_kusd(rmrkPool[0].toString() / 10 ** 12, rmrkPool[1].toString() / 10 ** 10, rmrkSupply);
    const outKsm = calculate_kusd_to_ksm(ksmPool[0].toString() / 10 ** 12, ksmPool[1].toString() / 10 ** 12, kusdMiddle);
    console.log("%d RMRK -> %d KUSD -> %d KSM", rmrkSupply, kusdMiddle, outKsm);
}

async function get_assets(api) {
    //get the whole map
    const assetRegistry = await api.query.assetRegistry.assetMetadatas.entries();
    // console.log(assetRegistry)
    assetRegistry.forEach(([key, exposure]) => {
        console.log('key arguments:', key.args.map((k) => k.toHuman()));
        console.log('     exposure:', exposure.toHuman());
    });
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

    fs.writeFile('kar_asset_registry_2', jsonAssets, function (err) {
        if (err) throw err;
        console.log('Saved!');
    });

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

async function save_dex_to_file(api) {
    const liqPools = await api.query.dex.liquidityPool.entries();

    let dexPools = [];
    // let liquidityStats = [];
    // let dexAssets = []
    liqPools.forEach(([key, exposure]) => {
        // dexPools.push(key.args.toString())
        // liquidityStats.push(exposure.toHuman())
        let dexPool = {
            poolAssets : key.args.toString(),
            liquidityStats : exposure.toHuman()
        }
        dexPools.push(dexPool);
    });
    // const pools = {
    //     pools: dexPools,
    //     liquidity: liquidityStats
    // };
    const jsonObj = JSON.stringify(dexPools);
    console.log(jsonObj)
    var fs = require('fs');

    fs.writeFile('liqpools.txt', jsonObj, function (err) {
        if (err) throw err;
        console.log('Saved!');
    });
}
//TO DO
//find the best path to trade out of all token pools
function find_all_paths(tokenA, tokenB, liqPools) {

}