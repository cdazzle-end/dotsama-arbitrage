import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { getNativeAsset, getStableAsset } from './native_assets';
import { Keyring, ApiPromise } from '@polkadot/api';
import {WsProvider } from '@polkadot/rpc-provider'
import { options } from '@acala-network/api'
import { AnyJson, Codec } from '@polkadot/types-codec/types';
// import {  } from '@acala-network/types'
import { CurrencyId, AssetId, AcalaAssetMetadata } from '@acala-network/types/interfaces'
import { Junction, MultiLocation } from '@polkadot/types/interfaces'

//This can be converted to unified MyAsset type
interface KaruraAsset {
    network: "kusama";
    chain: number,
    localId: string,
    name: string,
    symbol: string,
    decimals: string,
    minimalBalance: string,
}

class MyKaruraAsset implements KaruraAsset {
    // network: "kusama";
    chain = 2000
    constructor(
        public network: "kusama",
        public localId: string,
        public name: string,
        public symbol: string,
        public decimals: string,
        public minimalBalance: string,
    ) { }
}

export async function saveAssets() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    //Asset metadata and asset locations
    const assetData: KaruraAsset[] = await queryAssets(api);
    const assetLocations: [MultiLocation , CurrencyId][] = await queryAssetLocations(api);

    //Combine metadata and location to create Asset Registry objects
    let assetRegistry = await Promise.all(assetData.map(async (asset: KaruraAsset) => {
        // console.log(asset.localId)
        console.log(asset.localId)
        // console.log(asset)
        if (Object.keys(asset.localId)[0] == "ForeignAssetId") {
            const assetLocation = await assetLocations.find((location) => {
                if (Object.values(asset.localId)[0] as any === Object.values(location[1].toHuman() as any)[0] as any) {
                    return true;
                }
            });
            const newAssetRegistryObject: MyAssetRegistryObject = assetLocation ? {
                tokenData: asset as MyAsset,
                hasLocation: true,
                tokenLocation: assetLocation[0]
            } : {
                    tokenData: asset as MyAsset,
                    hasLocation: false,
                };
            return newAssetRegistryObject;
        }
        else if (Object.keys(asset.localId)[0] == "NativeAssetId") {
            const assetId = Object.values(Object.values(asset.localId)[0])[0]
            const assetLocation = await getNativeAsset(assetId);
            api.createType('Junctions', assetLocation.interior)
            if (assetLocation == "here") {
                const newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset as MyAsset,
                    hasLocation: true,
                    tokenLocation: assetLocation
                }
                return newAssetRegistryObject
            } else {
                const newAssetRegistryObject: MyAssetRegistryObject = assetLocation? {
                    tokenData: asset as MyAsset,
                    hasLocation: true,
                    tokenLocation: api.createType('Junctions', assetLocation.interior).toJSON()
                } : {
                    tokenData: asset as MyAsset,
                    hasLocation: false,
                }
                return newAssetRegistryObject
            }
            
        } else if (Object.keys(asset.localId)[0] == "StableAssetId") {
            const assetId = Object.values(Object.values(asset.localId)[0])[0]
            const assetLocation = await getStableAsset(assetId);
            const newAssetRegistryObject: MyAssetRegistryObject = assetLocation ? {
                tokenData: asset as MyAsset,
                hasLocation: true,
                tokenLocation: api.createType('Junctions', assetLocation.interior).toJSON()
            } : {
                tokenData: asset as MyAsset,
                hasLocation: false,
            }
            return newAssetRegistryObject
        } else if (Object.keys(asset.localId)[0] == "Erc20") {
            const newAssetRegistryObject: MyAssetRegistryObject = {
                tokenData: asset as MyAsset,
                hasLocation: false,
            }
            return newAssetRegistryObject
        }

    }));
    fs.writeFileSync('../../assets/kar/asset_registry.json', JSON.stringify(assetRegistry, null, 2));
}

export async function getAssets(): Promise<MyAssetRegistryObject[]> {
    return JSON.parse(fs.readFileSync('../assets/kar/asset_registry.json', 'utf8'));
}

//api is funky with asset id's, convert asset id's to currency id's
async function queryAssets(api: ApiPromise): Promise<KaruraAsset[]> {
    await api.isReady;
    const assetRegistry = await api.query.assetRegistry.assetMetadatas.entries();
    const assets = assetRegistry.map(([key, value]) => {
        let localId = (key.toHuman() as any)[0];
        const metaData = value.toHuman() as any;
        console.log(metaData)
        const asset: KaruraAsset = new MyKaruraAsset("kusama", localId, metaData.name, metaData.symbol, metaData.decimals, metaData.minimalBalance.toString().replace(/,/g, ""));
        return asset
    })
    return assets;
}

//Karura js API only retrieves ForeignAssets
async function queryAssetLocations(api: ApiPromise): Promise<[MultiLocation, CurrencyId][]> {
    await api.isReady;
  
    const locationEntries = await api.query.assetRegistry.locationToCurrencyIds.entries();
    const assetLocations: any = locationEntries.map(([location, currencyId]) => {
        let locationData = (location.toHuman() as any)[0];
        const junction = Object.keys(locationData.interior)[0]
        if (junction == "X1") {
            const junctionData = locationData.interior[junction];
            const junctionType = Object.keys(junctionData)[0]
            let junctionValue = junctionData[junctionType]
            junctionValue = junctionValue.toString().replace(/,/g, "")
            let newJunction: MyJunction = {};
            newJunction[junctionType] = junctionValue;
            let newLocation: MyMultiLocation = {
                X1: newJunction
            }
            let formattedLocation = api.createType('Junctions', newLocation).toJSON()
            return [formattedLocation, currencyId]
        } else if (junction == "Here") {
            let newLocation = "here"
            return [newLocation, currencyId]
        } else {
            const junctions = locationData.interior[junction];
            let junctionList: MyJunction[] = [];
            for (const x in junctions) {
                let junctionType = Object.keys(junctions[x])[0]
                let junctionValue = junctions[x][junctionType]
                junctionValue = junctionValue.toString().replace(/,/g, "")
                let newJunction: MyJunction = {};
                newJunction[junctionType] = junctionValue;
                junctionList.push(newJunction)
            }

            let newLocation: MyMultiLocation = {
                [junction]: junctionList
            }
            let formattedLocation = api.createType('Junctions', newLocation).toJSON()
            return [formattedLocation, currencyId]
        }
    })

    //Make sure that location and currencyId are correct
    const multiLocations: [MultiLocation, CurrencyId][] = assetLocations.map(([location, currencyId]: [MyMultiLocation, Codec]) => {
        const multiLocation: MultiLocation = api.createType('Junctions', location)
        let karCurrencyId = api.createType('CurrencyId', currencyId.toHex())
        return [multiLocation, karCurrencyId]
    })
    return assetLocations;
}
    
async function main() {
    // getAssets()
    await saveAssets()
    // getAssetLocations(api)
    // process.exit(0)
}

// main()