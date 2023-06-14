import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
const NATIVE_KUSAMA_ID = 4;
const MGX_ID = 0;
const KAR_ID = 6;

export async function getAssets() {
    return JSON.parse(fs.readFileSync('./mgx/asset_registry.json', 'utf8'));
}

async function saveAssets() {
    const provider = new WsProvider('wss://mangata-x.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;

    let mgxLocationToAssets = await api.query.assetRegistry.locationToAssetId.entries();
    let mgxAssets = await queryAssets()

    // let parachainId = await api.query.parachainInfo.parachainId();
    let assetRegistry = mgxAssets.map((asset) => {
        if (asset != null) {
            // console.log(asset)
            if (asset.localId == NATIVE_KUSAMA_ID) {
                // console.log("Found kusama")
                // console.log(asset)
                let tokenLocation = "here";
                // let formattedLocation = api.createType('Junctions', tokenLocation).toJSON()
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: true,
                    tokenLocation: tokenLocation
                }
                return newAssetRegistryObject
            }
            if (asset.localId == MGX_ID) {
                let tokenLocation = {
                    x1: { parachain: 2100 }
                }
                let formattedLocation = api.createType('Junctions', tokenLocation).toJSON()
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: true,
                    tokenLocation: formattedLocation
                }
                return newAssetRegistryObject
            }
            if (asset.localId == KAR_ID) {
                let tokenLocation = {
                    x2: [{ parachain: 2000 }, { generalKey: "0x0080" }]
                }
                let formattedLocation = api.createType('Junctions', tokenLocation).toJSON()
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: true,
                    tokenLocation: formattedLocation
                }
                return newAssetRegistryObject
            }

            let assetLocation0 = mgxLocationToAssets.find(([location, id]) => {
                if (id.toHuman() == asset.localId) {
                    console.log("found lcoation match")
                    return true
                }
            })
            // console.log(assetLocation0)

            if (assetLocation0 != undefined) {
                let [location, id] = assetLocation0   
                let locationData = (location.toHuman() as any)[0];
                console.log(locationData)
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
                    let newAssetRegistryObject: MyAssetRegistryObject = {
                        tokenData: asset,
                        hasLocation: true,
                        tokenLocation: formattedLocation
                    }
                    // console.log(newAssetRegistryObject)
                    return newAssetRegistryObject
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
                    let newAssetRegistryObject: MyAssetRegistryObject = {
                        tokenData: asset,
                        hasLocation: true,
                        tokenLocation: formattedLocation
                    }
                    // console.log(newAssetRegistryObject)
                    return newAssetRegistryObject
                }
            } else {
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: false,
                }
                return newAssetRegistryObject
            }
        }   
        
        
    })
    console.log("TEST")
    console.log(assetRegistry)
    assetRegistry.forEach((asset) => {
        console.log("A R Object")
        console.log(asset)
    })
    fs.writeFileSync('./asset_registry.json', JSON.stringify(assetRegistry, null, 2));
}

async function queryAssets() {
    const provider = new WsProvider('wss://mangata-x.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;

    let mgxAssetsMeta = await api.query.assetRegistry.metadata.entries();
    let parachainIdResult = await api.query.parachainInfo.parachainId();
    let parachainId = parachainIdResult.toHuman() as string;
    console.log(parachainId)
    let formatParachainId = parachainId.replace(/,/g, "") as any as number
    console.log(formatParachainId)
    
    let myAssets =  mgxAssetsMeta.map(([id, assetMeta]) => {

        let assetData = assetMeta.toHuman() as any;
        if (assetData != null) {
            let myAsset: MyAsset = {
                network: "kusama",
                chain: 2110,
                localId: (id.toHuman() as string)[0],
                name: assetData["name"],
                symbol: assetData["symbol"],
                decimals: assetData["decimals"],
            }
            // console.log(myAsset)
            return myAsset
        }
    })
    return myAssets
}

async function queryLocations() {
    const provider = new WsProvider('wss://mangata-x.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;

    let mgxLocationToAssets = await api.query.assetRegistry.locationToAssetId.entries();
}

async function main() {
    saveAssets()
}

main()