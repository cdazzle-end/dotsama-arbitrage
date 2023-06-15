import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
// import { Keyring, ApiPromise, WsProvider, } from '@polkadot/api';

import { options } from '@parallel-finance/api/index';
import { CurrencyId } from '@parallel-finance/types/interfaces';
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';


async function test() {
    

    // api.createType('MultiLocation')
}

export async function saveAssets() {
    const provider = new WsProvider('wss://heiko-rpc.parallel.fi');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;
    const assetData = await queryAssets();
    const locationData = await queryLocations();

    let assetRegistry = await Promise.all(assetData.map(async (asset) => {
        // console.log(asset.localId)
        const matchedLocation = await locationData.find(([location, id]) => {
            return id == asset.localId
        })
        // console.log(matchedLocation)
        if (matchedLocation == undefined) {
            const newAssetRegistryObject: MyAssetRegistryObject = {
                tokenData: asset,
                hasLocation: false
            }
            return newAssetRegistryObject
        } else {
            const newAssetRegistryObject: MyAssetRegistryObject = {
                tokenData: asset,
                hasLocation: true,
                tokenLocation: matchedLocation[0]
            }
            return newAssetRegistryObject

        }
    }))
    // assetRegistry = assetRegistry.filter((asset) => {
    //     return asset != undefined
    // })
    console.log(assetRegistry)
    fs.writeFileSync('../../assets/hko/asset_registry.json', JSON.stringify(assetRegistry, null, 2))
    
}

export async function getAssets(): Promise < MyAssetRegistryObject[] > {
    return JSON.parse(fs.readFileSync('../assets/hko/asset_registry.json', 'utf8'));
}

async function queryAssets(): Promise<MyAsset[]> {
    const provider = new WsProvider('wss://heiko-rpc.parallel.fi');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;
    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number;
    let testData = await api.query.assetRegistry.assetIdType.entries();
    let xcAssetData = await api.query.assets.metadata.entries();
    let hkoAssets: MyAsset[] = xcAssetData.map(([key, value]) => {
        const currencyId = (key.toHuman() as any)[0].replace(/,/g, "");
        api.createType('CurrencyId', currencyId)
        const assetValue = value.toHuman() as any;
        const asset: MyAsset = {
            network: "kusama",
            chain: parachainId,
            localId: currencyId,
            name: assetValue.name,
            symbol: assetValue.symbol,
            decimals: assetValue.decimals,
            deposit: assetValue.deposit,
            isFrozen: assetValue.isFrozen,
        }
        console.log(asset)
        return asset
    })

    hkoAssets.push(
        {
            network: "kusama",
            chain: parachainId,
            localId: "0",
            deposit: "0",
            name: "HKO",
            symbol: "HKO",
            decimals: "12",
            isFrozen: false,
        }
    )
    return hkoAssets
}

async function queryLocations() {
    const provider = new WsProvider('wss://heiko-rpc.parallel.fi');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;
    const locationEntries = await api.query.assetRegistry.assetIdType.entries();
    let assetLocations = locationEntries.map(([key, value]) => {
        const currencyId = (key.toHuman() as any)[0].replace(/,/g, "");
        const locationValue = (value.toJSON() as any)['xcm']['interior'];
        const junction = Object.keys(locationValue)[0]
        if (junction == "here") {
            // console.log(("HERE"))
            const newLocation = "here"
            return [newLocation, currencyId]
        } else {
            // console.log(locationValue)
            const newLocation: MyMultiLocation = {
                [junction]: locationValue[junction]
            }
            // console.log(newLocation)
            return [newLocation, currencyId]
        }

    })
    console.log(assetLocations)

    //Make sure location and id are proper format
    assetLocations.forEach(([multiLocation, currencyId]) => {
        console.log(api.createType('CurrencyId', currencyId).toHuman())
        console.log(api.createType('Junctions', multiLocation).toHuman())
    })

    return assetLocations;
    // let xcAssetLocations = xcAssetLocationData.map(([key, value]) => {
}

async function main() {
    // queryAssets()
    // queryLocations()
    saveAssets()
    // getAssets()
}

// main()