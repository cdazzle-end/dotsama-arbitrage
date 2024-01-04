import * as fs from 'fs';
import path from 'path';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';

const statemineWss = 'wss://statemine-rpc.dwellir.com'

async function queryAssets(){
    const provider = new WsProvider(statemineWss);
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number
    const assetEntries = await api.query.assets.metadata.entries();
    let assets = assetEntries.map( ([assetId, asset]) => {
        const assetData = asset.toHuman() as any;
        const id = (assetId.toHuman() as any)[0].replace(/,/g, "");


        const newAsset: MyAsset = {
            network: "kusama",
            chain: parachainId,
            localId: (assetId.toHuman() as any)[0].replace(/,/g, ""),
            name: assetData.name,
            symbol: assetData.symbol,
            decimals: assetData.decimals,
            deposit: assetData.deposit,
            isFrozen: assetData.isFrozen,
        }
        // console.log(newAsset)
        let tokenLocation = {
            X3: [
                {Parachain: "1000"},
                {PalletInstance: "50"},
                {GeneralIndex: newAsset.localId}
            ]
        }
        // console.log(JSON.stringify(tokenLocation, null, 2))
        let newAssetRegistryObject: MyAssetRegistryObject = {
            tokenData: newAsset,
            hasLocation: true,
            tokenLocation: tokenLocation
        }
        return newAssetRegistryObject
    })
    // console.log(JSON.stringify(assets, null, 2))
    // console.log(JSON.stringify(assets, null, 2))
    await api.disconnect()
    return assets
    
}

async function saveAssets() {
    let assetRegistry = await queryAssets();
    const filePath = path.join(__dirname, 'asset_registry.json')
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2))
    process.exit(0)
}

saveAssets()

