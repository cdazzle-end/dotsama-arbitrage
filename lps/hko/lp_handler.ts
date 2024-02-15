import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../../assets/asset_types';
import {MyLp} from '../lp_types';
// import { Keyring, ApiPromise, WsProvider, } from '@polkadot/api';
import { options } from '@parallel-finance/api';
import { CurrencyId, Pool, Balance } from '@parallel-finance/types/interfaces';
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';
// import { BigNumber } from 'ethers';
import {BN} from '@polkadot/util';

const localRpc = "ws://172.26.130.75:8012"
const liveRpc = 'wss://heiko-rpc.parallel.fi'

export async function updateLps(chopsticks: boolean) {
    let rpc = chopsticks ? localRpc : liveRpc
    const provider = new WsProvider(rpc);
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number;
    const lpEntries = await api.query.amm.pools.entries();
    let assets: MyAsset[] = JSON.parse(fs.readFileSync('../assets/hko/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    })
    let lps = lpEntries.map(([assetData, lpData]) => {
        const poolAssetIdData = assetData.args
        const lp = lpData.toJSON()
        const pool: Pool = api.createType('Pool', lp)
        const [baseAmount, quoteAmount] = [pool.baseAmount.toString(), pool.quoteAmount.toString()]
        const poolAssetIds = [(poolAssetIdData[0].toHuman() as any).replace(/,/g, ""), (poolAssetIdData[1].toHuman() as any).replace(/,/g, "")]
        const [baseAsset, quoteAsset] = poolAssetIds.map((poolAssetId: any) => {
            const matchedAsset = assets.find((asset: any) => {
                return asset.localId == poolAssetId
            })
            return matchedAsset
        })
        const newLp: MyLp = {
            chainId: parachainId,
            poolAssets: [baseAsset?.localId, quoteAsset?.localId],
            liquidityStats: [baseAmount, quoteAmount]
        }
        return newLp
    })
    //If a pool asset is not found in the asset registry, remove it from the lps array
    lps = lps.filter((lp) => {
        return lp.poolAssets[0] != undefined || lp.poolAssets[1] != undefined
    })
    fs.writeFileSync('./hko/lps.json', JSON.stringify(lps, null, 2))
    api.disconnect()
}

async function saveLps() {
    const provider = new WsProvider('wss://heiko-rpc.parallel.fi');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number;
    const lpEntries = await api.query.amm.pools.entries();
    

    let assets: MyAsset[] = JSON.parse(fs.readFileSync('../../assets/hko/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    })
    let lps = lpEntries.map(([assetData, lpData]) => {
        const poolAssetIdData = assetData.args
        


        const lp = lpData.toJSON()
        const pool: Pool = api.createType('Pool', lp)
        const [baseAmount, quoteAmount] = [pool.baseAmount.toString(), pool.quoteAmount.toString()]
        const poolAssetIds = [(poolAssetIdData[0].toHuman() as any).replace(/,/g, ""), (poolAssetIdData[1].toHuman() as any).replace(/,/g, "")]
        const [baseAsset, quoteAsset] = poolAssetIds.map((poolAssetId: any) => {
            const matchedAsset = assets.find((asset: any) => {
                return asset.localId == poolAssetId
            })
            return matchedAsset
        })
        const newLp: MyLp = {
            chainId: parachainId,
            poolAssets: [baseAsset?.localId, quoteAsset?.localId],
            liquidityStats: [baseAmount, quoteAmount]
        }
        return newLp
    })
    //If a pool asset is not found in the asset registry, remove it from the lps array
    lps = lps.filter((lp) => {
        return lp.poolAssets[0] != undefined || lp.poolAssets[1] != undefined
    })
    fs.writeFileSync('../hko/lps.json', JSON.stringify(lps, null, 2))
}

async function getLps(): Promise<MyLp[]> {
    return JSON.parse(fs.readFileSync('../hko/lps.json', 'utf8'));
}

async function main() {
    await saveLps()
    // await getLps()
    process.exit(0)
}

// main()