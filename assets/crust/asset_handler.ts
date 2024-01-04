import * as fs from 'fs';
import path from 'path';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';

const crustWss = 'wss://rpc-shadow.crust.network/'

async function queryAssets(){
    const provider = new WsProvider(crustWss);
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number

    // const assetEntries = await api.query.assets.metadata.entries();
    // let assets = assetEntries.map( ([assetId, asset]) => {
    //     const assetData = asset.toHuman() as any;
    //     const id = (assetId.toHuman() as any)[0].replace(/,/g, "");


    //     const newAsset: MyAsset = {
    //         network: "kusama",
    //         chain: parachainId,
    //         localId: (assetId.toHuman() as any)[0].replace(/,/g, ""),
    //         name: assetData.name,
    //         symbol: assetData.symbol,
    //         decimals: assetData.decimals,
    //         deposit: assetData.deposit,
    //         isFrozen: assetData.isFrozen,
    //     }
    //     // console.log(newAsset)
    //     let tokenLocation = {
    //         X3: [
    //             {Parachain: "1000"},
    //             {PalletInstance: "50"},
    //             {GeneralIndex: newAsset.localId}
    //         ]
    //     }
    //     // console.log(JSON.stringify(tokenLocation, null, 2))
    //     let newAssetRegistryObject: MyAssetRegistryObject = {
    //         tokenData: newAsset,
    //         hasLocation: true,
    //         tokenLocation: tokenLocation
    //     }
    //     return newAssetRegistryObject
    // })

    let csmAsset: MyAsset = {
        network: "kusama",
        chain: parachainId,
        localId: "0",
        name: "CSM",
        symbol: "CSM",
        decimals: "12",
        deposit: "0",
        isFrozen: false,
    }
    let csmTokenLocation = {
        X1: {Parachain: "2012"}
    }
    let csmAssetRegistryObject: MyAssetRegistryObject = {
        tokenData: csmAsset,
        hasLocation: true,
        tokenLocation: csmTokenLocation
    }
    let assets = [csmAssetRegistryObject]
    // console.log(JSON.stringify(assets, null, 2))
    // console.log(JSON.stringify(assets, null, 2))
    await api.disconnect()
    return assets
    
}

async function queryLocations() {
    // const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    // const api = await ApiPromise.create({ provider: provider });
    // await api.isReady

    // const locationEntries = await api.query.assetManager.assetIdType.entries();
    // // console.log(locationEntries.length + " " + loc2.length)
    // let assetLocations = await Promise.all(locationEntries.map(async ([key, value]) => {
    //     const currencyId = (key.toHuman() as any)[0].replace(/,/g, "");
    //     let locationData = (value.toHuman() as any)['Xcm']['interior'];
    //     locationData = removeCommasFromAllValues(locationData);
    //     const junction = Object.keys(locationData)[0]

    //     let genKey = await findValueByKey(locationData, "generalKey")

    //     let junctionList: MyJunction[] = []
    //     if (locationData == "Here") {
    //         const newLocation = "here"
    //         return [newLocation, currencyId]
    //     } else {
    //         const junctionData = locationData[junction]

    //         if (!Array.isArray(junctionData)) { // If junction is X1
    //             let newLocation: MyMultiLocation;
    //             let newJunction: MyJunction = junctionData;
    //             newLocation = {
    //                 [junction]: newJunction
    //             }
    //             return [newLocation, currencyId]

    //         } else {
    //             const junctions = locationData[junction];
    //             junctions.forEach((junction: any) => {
    //                 const newJunction: MyJunction = junction;
    //                 junctionList.push(newJunction)
    //             })
    //             let newLocation: MyMultiLocation = {
    //                 [junction]: junctionList
    //             }
    //             return [newLocation, currencyId]
    //         }
    //     }
    // }))

    // return assetLocations;
}

async function saveAssets() {
    let assetRegistry = await queryAssets();
    const filePath = path.join(__dirname, 'asset_registry.json')
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2))
    process.exit(0)
}

saveAssets()
