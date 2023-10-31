import path from 'path';
import * as fs from 'fs';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import { Mangata } from '@mangata-finance/sdk';

const MAINNET_1 = 'wss://mangata-x.api.onfinality.io/public-ws'
const MAINNET_2 = 'wss://prod-kusama-collator-01.mangatafinance.cloud'
const MAINNET_3 = 'wss://mangatax.api.onfinality.io/public-ws'
const MAINNET_4 = 'wss://prod-kusama-collator-02.mangatafinance.cloud'
const MAINNET_5 = 'wss://kusama-rpc.mangata.online'
const NATIVE_KUSAMA_ID = 4;
const MGX_ID = 0;
const KAR_ID = 6;

export async function getAssets() {
    return JSON.parse(fs.readFileSync('./mgx/asset_registry.json', 'utf8'));
}

export async function saveAssets() {
    const mangata = Mangata.getInstance([MAINNET_5])
    const api = await mangata.getApi();
    await api.isReady;

    let mgxLocationToAssets = await api.query.assetRegistry.locationToAssetId.entries();
    let mgxAssets = await queryAssets()

    // 1 get all assets from metadata query
    // 2 Check if ID is KAR, MGX, or KSM
    let assetRegistry = mgxAssets.map((asset) => {
        if (asset != null) { // Not sure why this is needed **LMAO copilot just suggested this comment to me as i was about to type it. I suck
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
                    X1: { parachain: "2110" }
                }
                let formattedLocation = api.createType('Junctions', tokenLocation).toJSON()
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: true,
                    tokenLocation: tokenLocation
                }
                return newAssetRegistryObject
            }
            if (asset.localId == KAR_ID) {
                let tokenLocation = {
                    X2: [{ Parachain: "2000" }, { GeneralKey: { "length": "2", "data": "0x0080000000000000000000000000000000000000000000000000000000000000" }}]
                }
                let formattedLocation = api.createType('Junctions', tokenLocation).toJSON()
                let newAssetRegistryObject: MyAssetRegistryObject = {
                    tokenData: asset,
                    hasLocation: true,
                    tokenLocation: tokenLocation
                }
                return newAssetRegistryObject
            }

            let assetLocation0 = mgxLocationToAssets.find(([location, id]) => {
                if (id.toHuman() == asset.localId) {
                    // console.log("found lcoation match")
                    return true
                }
            })
            // console.log(assetLocation0)

            if (assetLocation0 != undefined) {
                // console.log("ASSET")
                // console.log(asset)
                let [location, id] = assetLocation0   
                let locationData = (location.toHuman() as any)[0];
                // console.log(locationData)
                const junction = Object.keys(locationData.interior)[0]
                if (junction == "X1") {
                    console.log("X1")
                    console.log(asset)
                    console.log(JSON.stringify(locationData))
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
                        tokenLocation: newLocation
                    }
                    console.log(JSON.stringify(newAssetRegistryObject))
                    // console.log(newAssetRegistryObject)
                    return newAssetRegistryObject
                } else {
                    // console.log("Junction is not X1")
                    // console.log(asset)
                    // console.log(locationData)
                    const junctions = locationData.interior[junction];
                    let junctionList: MyJunction[] = [];
                    for (const x in junctions) {
                        let junctionType = Object.keys(junctions[x])[0]
                        let junctionValue = junctions[x][junctionType]

                        if (junctionType == "GeneralKey") {
                            let keys = Object.keys(junctions[x])[0]
                            let val = junctions[x][keys]
                            let newJunction: MyJunction = {
                                GeneralKey: {
                                    length: val.length,
                                    data: val.data
                                }
                            };
                            junctionList.push(newJunction)

                        } else {
                            junctionValue = junctionValue.toString().replace(/,/g, "")
                            let newJunction: MyJunction = {};
                            newJunction[junctionType] = junctionValue;
                            junctionList.push(newJunction)
                        }
                    
                        
                    }
                    let newLocation: MyMultiLocation = {
                        [junction]: junctionList
                    }
                    let formattedLocation = api.createType('Junctions', newLocation).toJSON()
                    let newAssetRegistryObject: MyAssetRegistryObject = {
                        tokenData: asset,
                        hasLocation: true,
                        tokenLocation: newLocation
                    }
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
    // console.log(assetRegistry)
    // assetRegistry.forEach((asset) => {
    //     console.log("A R Object")
    //     console.log(asset)
    //     console.log(asset?.tokenLocation)
    // })
    const filePath = path.join(__dirname, './asset_registry.json')
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2));
}

async function queryAssets() {
    // const provider = new WsProvider(MAINNET_5);
    const mangata = Mangata.getInstance([MAINNET_5])
    const api = await mangata.getApi();
    await api.isReady;

    let mgxAssetsMeta = await api.query.assetRegistry.metadata.entries();
    let parachainIdResult = await api.query.parachainInfo.parachainId();
    let parachainId = parachainIdResult.toHuman() as string;
    // console.log(parachainId)
    let formatParachainId = parachainId.replace(/,/g, "") as any as number
    // console.log(formatParachainId)
    
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
    await saveAssets()
    process.exit(0)
}

// main()
// {"X2":[{"Parachain":"2001"},{"GeneralKey":{"length":"2","data":"0x0207000000000000000000000000000000000000000000000000000000000000"}}]}
// {"X2":[{"Parachain":"2001"},{"GeneralKey":{"length":2,"data":"0x0207000000000000000000000000000000000000000000000000000000000000"}}]}