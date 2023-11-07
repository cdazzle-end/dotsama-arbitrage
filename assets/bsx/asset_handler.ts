import * as fs from 'fs';
import path from 'path';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';

export async function saveAssets() {
    const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    let assetData = await queryAssets(api);
    let assetLocations = await queryLocations(api);

    console.log(assetLocations)

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
            console.log(matchedLocation[0])
            const newAssetRegistryObject: MyAssetRegistryObject = {
                tokenData: asset,
                hasLocation: true,
                tokenLocation: matchedLocation[0]
            }
            return newAssetRegistryObject
        }
    });
    // assetRegistry.forEach((asset: any) => {
    //     console.log(asset)
    //     console.log(asset.tokenLocation)
    // })

    const filePath = path.join(__dirname, 'asset_registry.json')
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2))
    process.exit(0)
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

// Find parachain value and remove commas
function updateValueByKey(obj: any, targetKey: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    for (let key in obj) {
        if (key === targetKey && typeof obj[key] === 'string') {
            // Remove commas if the value is a string
            obj[key] = obj[key].replace(/,/g, '');
        } else {
            // Recursively check and update nested objects
            obj[key] = updateValueByKey(obj[key], targetKey);
        }
    }

    return obj;
}
//Remove all commas
function removeCommasFromAllValues(obj: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    for (let key in obj) {
        if (typeof obj[key] === 'string') {
            // Remove commas if the value is a string
            obj[key] = obj[key].replace(/,/g, '');
        } else {
            // Recursively remove commas from nested objects
            obj[key] = removeCommasFromAllValues(obj[key]);
        }
    }

    return obj;
}

function findValueByKey(obj: any, targetKey: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return null;
    }
    for (let key in obj) {
        if (key === targetKey) {
            return obj[key];
        }

        let foundValue: any = findValueByKey(obj[key], targetKey);
        if (foundValue !== null) {
            return foundValue;
        }
    }

    return null;
}

async function queryLocations(api:any) {
    // const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    // const api = await ApiPromise.create({ provider: provider });
    // await api.isReady;
    let locationEntries = await api.query.assetRegistry.assetLocations.entries();
    let locations = locationEntries.map(([id, location]: any) => {
        const currencyId = (id.toHuman() as any)[0].replace(/,/g, "");
        let locationData = (location.toHuman() as any);
        // console.log(JSON.stringify(locationData))
        // let para = findValueByKey(locationData, "Parachain")
        // console.log(para)
        // locationData = updateValueByKey(locationData, "Parachain")
        // console.log(JSON.stringify(para))
        locationData = removeCommasFromAllValues(locationData)
        const junction = Object.keys(locationData.interior)[0]
        let junctionList: MyJunction[] = [];

        if (locationData.interior == "Here") {
            const newLocation = "here"
            return [newLocation, currencyId]
        } else {
            const junctionData = locationData.interior[junction];

            if (!Array.isArray(junctionData)) { // If junction is X1
                let newLocation: MyMultiLocation;
                let newJunction: MyJunction = junctionData;
                newLocation = {
                    [junction]: newJunction
                }
                return [newLocation, currencyId]

            } else {
                const junctions = locationData.interior[junction];
                junctions.forEach((junction: any) => {
                    const newJunction: MyJunction = junction;
                    junctionList.push(newJunction)
                })
                let newLocation: MyMultiLocation = {
                    [junction]: junctionList
                }
                return [newLocation, currencyId]
            }
        }

    })
    let bsxLocation = {
        X2: [
            { Parachain: "2090" },
            { GeneralIndex: "0" }
        ]
    }
    locations.push([bsxLocation, 0]) 
    locations.forEach(([location, id]: any) => {
        console.log(id)
        console.log(JSON.stringify(location))
    })
    return locations;

}

async function main() {
    const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    // await queryAssets(api);
    // await queryLocations(api);
    await saveAssets();

    process.exit(0)
}

main()