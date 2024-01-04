import * as fs from 'fs';
import path from 'path';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';

const kintWss = "wss://kintsugi-rpc.dwellir.com"

async function queryAssets(){
    const provider = new WsProvider(kintWss);
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number

    
    let kintAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: {Token: "KINT"},
        name: "KINT",
        symbol: "KINT",
        decimals: "12",
    }
    let kintTokenLocation = {
        X2: [
                {Parachain: "2092"},
                {
                    GeneralKey: {
                        length: "2",
                        data: "0x000c000000000000000000000000000000000000000000000000000000000000"
                    }
                }
            ]
    }
    let kintAssetRegistryObject: MyAssetRegistryObject = {
        tokenData: kintAsset,
        hasLocation: true,
        tokenLocation: kintTokenLocation
    }
    let kbtcAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: {Token: "KBTC"},
        name: "KBTC",
        symbol: "KBTC",
        decimals: "8",
    }
    let kbtcTokenLocation = {
        X2: [
                {Parachain: "2092"},
                {
                    GeneralKey: {
                        length: "2",
                        data: "0x000b000000000000000000000000000000000000000000000000000000000000"
                    }
                }
            ]
    }
    let kbtcAssetRegistryObject: MyAssetRegistryObject = {
        tokenData: kbtcAsset,
        hasLocation: true,
        tokenLocation: kbtcTokenLocation
    }

    return [kintAssetRegistryObject, kbtcAssetRegistryObject]
}

async function saveAssets() {
    let assetRegistry = await queryAssets();
    const filePath = path.join(__dirname, 'asset_registry.json')
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2))
    process.exit(0)
}

saveAssets()

