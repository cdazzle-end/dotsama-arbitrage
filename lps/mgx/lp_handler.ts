import { Mangata } from '@mangata-finance/sdk';
import { MyLp } from '../lp_types';
import * as fs from 'fs';
const localRpc = "ws://172.26.130.75:8011"
const liveRpc = 'wss://kusama-rpc.mangata.online'
export async function updateLps(chopsticks: boolean) {
    let rpc = chopsticks ? localRpc : liveRpc
    const MAINNET_1 = 'wss://mangata-x.api.onfinality.io/public-ws'
    const MAINNET_2 = 'wss://prod-kusama-collator-01.mangatafinance.cloud'
    const MAINNET_3 = 'wss://mangatax.api.onfinality.io/public-ws'
    const MAINNET_4 = 'wss://prod-kusama-collator-02.mangatafinance.cloud'
    const MAINNET_5 = 'wss://kusama-rpc.mangata.online'
    // const mangata = Mangata.getInstance([MAINNET_1, MAINNET_2])
    const mangata = Mangata.getInstance([rpc])

    // Retrieve the chainName, nodeName & nodeVersion information
    const [chain, nodeName, nodeVersion] = await Promise.all([
        mangata.getChain(),
        mangata.getNodeName(),
        mangata.getNodeVersion()
    ]);

    // console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    let api = await mangata.getApi();
    // await (await api).isReady;
    await api.isReady;

    let parachainId = await api.query.parachainInfo?.parachainId();
    let chainId = parachainId.toHuman() as any;
    let chainIdNum = chainId.replace(/,/g, "");
    // console.log("ChainId: ", chainIdNum)
    let poolAssets = await api.query.xyk.liquidityPools.entries();
    let poolAssetIds = await poolAssets.map(([key, assetIds]) => {
        // console.log(key.toHuman());
        // console.log(assetIds.toHuman());
        return assetIds
    })
    let lps = await Promise.all(poolAssetIds.map(async (assetIds) => {
        let assetIdsFormat = assetIds.toHuman() as any;
        let poolLiquidity = await api.query.xyk.pools([assetIdsFormat[0], assetIdsFormat[1]]);
        let poolFormat = poolLiquidity.toHuman() as any;
        // console.log(poolFormat)
        let poolLiq = poolFormat.map((liq: any) => {
            return liq.replace(/,/g, "");
        })
        // console.log(poolLiq)
        // console.log("AssetIds: ", assetIds.toHuman());
        // console.log("Pool Liquidity: ", poolLiquidity.toHuman());
        let lp: MyLp = {
            chainId: parseInt(chainIdNum),
            poolAssets: assetIdsFormat,
            liquidityStats: poolLiq
        }
        // console.log(lp)
        return lp
    }))

    // console.log(lps)
    fs.writeFileSync('./mgx/lps.json', JSON.stringify(lps, null, 2));
    api.disconnect()
}

async function saveLps() {
    const MAINNET_1 = 'wss://mangata-x.api.onfinality.io/public-ws'
    const MAINNET_2 = 'wss://prod-kusama-collator-01.mangatafinance.cloud'
    const mangata = Mangata.getInstance([MAINNET_1, MAINNET_2])

    // Retrieve the chainName, nodeName & nodeVersion information
    // const [chain, nodeName, nodeVersion] = await Promise.all([
    //     mangata.getChain(),
    //     mangata.getNodeName(),
    //     mangata.getNodeVersion()
    // ]);

    // console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    let api = await mangata.getApi();
    // await (await api).isReady;
    await api.isReady;

    let parachainId = await api.query.parachainInfo?.parachainId();
    let chainId = parachainId.toHuman() as any;
    let chainIdNum = chainId.replace(/,/g, "");
    // console.log("ChainId: ", chainIdNum)
    let poolAssets = await api.query.xyk.liquidityPools.entries();
    let poolAssetIds = await poolAssets.map(([key, assetIds]) => {
        // console.log(key.toHuman());
        // console.log(assetIds.toHuman());
        return assetIds
    })
    let lps = await Promise.all(poolAssetIds.map(async (assetIds) => {
        let assetIdsFormat = assetIds.toHuman() as any;
        let poolLiquidity = await api.query.xyk.pools([assetIdsFormat[0], assetIdsFormat[1]]);
        let poolFormat = poolLiquidity.toHuman() as any;
        // console.log(poolFormat)
        let poolLiq = poolFormat.map((liq: any) => {
            return liq.replace(/,/g, "");
        })
        // console.log(poolLiq)
        // console.log("AssetIds: ", assetIds.toHuman());
        // console.log("Pool Liquidity: ", poolLiquidity.toHuman());
        let lp: MyLp = {
            chainId: parseInt(chainIdNum),
            poolAssets: assetIdsFormat,
            liquidityStats: poolLiq
        }
        // console.log(lp)
        return lp
    }))

    // console.log(lps)
    fs.writeFileSync('lps.json', JSON.stringify(lps, null, 2));
    api.disconnect()
}

async function main() {
    // Connect to the node
    const MAINNET_1 = 'wss://mangata-x.api.onfinality.io/public-ws'
    const MAINNET_2 = 'wss://prod-kusama-collator-01.mangatafinance.cloud'
    const mangata = Mangata.getInstance([MAINNET_1, MAINNET_2])

    // Retrieve the chainName, nodeName & nodeVersion information
    const [chain, nodeName, nodeVersion] = await Promise.all([
        mangata.getChain(),
        mangata.getNodeName(),
        mangata.getNodeVersion()
    ]);

    console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);
    let api = await mangata.getApi();
    // await (await api).isReady;
    await api.isReady;

    let parachainId = await api.query.parachainInfo?.parachainId();
    let chainId = parachainId.toHuman() as any;
    let chainIdNum = chainId.replace(/,/g, "");
    console.log("ChainId: ", chainIdNum)
    let poolAssets = await api.query.xyk.liquidityPools.entries();
    let poolAssetIds = await poolAssets.map(([key, assetIds]) => {
        // console.log(key.toHuman());
        // console.log(assetIds.toHuman());
        return assetIds
    })
    let lps = await Promise.all(poolAssetIds.map(async (assetIds) => {
        let assetIdsFormat = assetIds.toHuman() as any;
        let poolLiquidity = await api.query.xyk.pools([assetIdsFormat[0], assetIdsFormat[1]]);
        let poolFormat = poolLiquidity.toHuman() as any;
        console.log(poolFormat)
        let poolLiq = poolFormat.map((liq: any) => {
            return liq.replace(/,/g, "");
        })
        console.log(poolLiq)
        console.log("AssetIds: ", assetIds.toHuman());
        console.log("Pool Liquidity: ", poolLiquidity.toHuman());
        let lp: MyLp = {
            chainId: chainIdNum,
            poolAssets: assetIdsFormat,
            liquidityStats: poolLiq
        }
        // console.log(lp)
        return lp
    }))

    console.log(lps)
    fs.writeFileSync('lps.json', JSON.stringify(lps, null, 2));
}

// main().catch(console.error).finally(() => process.exit());