import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';

async function saveAssets() {
    const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    let assetData = await queryAssets(api);
    let assetLocations = await queryLocations(api);

    let assetRegistry = await assetData.map(([asset, assetId]: any) => {
        let matchedLocation = assetLocations.find(([location, locationId]: any) => {
            return assetId == locationId
        })

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
    });
    assetRegistry.forEach((asset: any) => {
        console.log(asset)
    })
    fs.writeFileSync('./asset_registry.json', JSON.stringify(assetRegistry, null, 2))
}

async function queryAssets(api:any) {
    console.log("test")
    let assets = await api.query.assetRegistry.assets.entries();
    let assetMeta = await api.query.assetRegistry.assetMetadataMap.entries();
    let assetRegistry = assetMeta.map(([key,value]: any) => {
        let k1 = key.toHuman() as any
        let matchedAsset = assets.find(([assetKey, assetValue]: any) => {
            let k2 = assetKey.toHuman() as any;
            if (k1[0] == k2[0]) {
                console.log("Matched")
                return true
            }
        })
        if (matchedAsset != undefined) {
            let [assetKey, assetValue] = matchedAsset;
            let assetData1 = value.toHuman() as any;
            let assetData2 = assetValue.toHuman() as any;
            let newMyAsset: MyAsset = {
                network: "kusama",
                chain: 2090,
                localId: k1[0],
                name: assetData2["name"],
                symbol: assetData1["symbol"],
                decimals: assetData1["decimals"],
            }
            // console.log(newMyAsset)
            return [newMyAsset, k1[0]]
        }
    })
    // assetRegistry.forEach(([asset, id]: any) => {
    //     console.log(id)
    //     console.log(asset)
    // })
    // console.log(assets.length)
    // console.log(assetMeta.length)
    return assetRegistry
}

async function queryLocations(api:any) {
    // const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    // const api = await ApiPromise.create({ provider: provider });
    // await api.isReady;
    let locationData = await api.query.assetRegistry.assetLocations.entries();
    let locations = locationData.map(([id, data]: any) => {
        console.log(data.toHuman())
        const currencyId = (id.toHuman() as any)[0].replace(/,/g, "");
        const locationValue = (data.toJSON() as any)['interior'];
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
    let bsxLocation = {
        x2: [
            { parachain: 2090 },
            { generalIndex: 0 }
        ]
    }
    locations.push([bsxLocation, 0])
    return locations;

}

async function main() {
    // const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    // const api = await ApiPromise.create({ provider: provider });
    // await api.isReady;
    // await queryAssets(api);
    // await queryLocations(api);
    await saveAssets();

    process.exit(0)
}

main()