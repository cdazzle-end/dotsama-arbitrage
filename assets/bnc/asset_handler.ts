import * as fs from 'fs';
import path from 'path';
import { MyJunction, MyAsset, MyAssetRegistryObject, MyMultiLocation } from '../asset_types';
import { Keyring, ApiPromise } from '@polkadot/api';
import { ApiOptions,  } from '@polkadot/api/types';
import { WsProvider  } from '@polkadot/rpc-provider'
import { Codec } from '@polkadot/types-codec/types';
import { Junction, MultiLocation, } from '@polkadot/types/interfaces'
import { firstValueFrom } from 'rxjs';
import { ModuleBApi, BifrostConfig } from '@zenlink-dex/sdk-api';
import { Percent, Token, TokenAmount, TradeType, StandardPair, StandardPool, StablePair, StableSwap, AssetMeta, AssetType } from '@zenlink-dex/sdk-core';
const axios = require('axios').default;

export async function saveAssets() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number;
    const bncAssets: MyAsset[] = await queryAssets();
    bncAssets.forEach((asset) => {
        console.log(asset)
    });
    const bncAssetLocations = await queryLocations();

    //Match assets with their locations
    const assetRegistry = await Promise.all(bncAssets.map(async (asset: MyAsset) => {
        let [assetIdKey, assetIdValue] = Object.entries(asset.localId)[0];
        // if (assetIdValue === "ZLK") {
        //     let colorLocation = await getColorLocation(assetIdKey, assetIdValue);
        //     let assetRegistryObject: MyAssetRegistryObject = {
        //         tokenData: asset,
        //         hasLocation: true,
        //         tokenLocation: colorLocation
        //     }
        //     return assetRegistryObject
        // }
        // if (assetIdKey === "VToken" && assetIdValue === "KSM") {
        //     let colorLocation = await getColorLocation(assetIdKey, assetIdValue);
        //     let assetRegistryObject: MyAssetRegistryObject = {
        //         tokenData: asset,
        //         hasLocation: true,
        //         tokenLocation: colorLocation
        //     }
        //     return assetRegistryObject
        // }
        // if (assetIdKey === "VSToken" && assetIdValue === "KSM") {
        //     let colorLocation = await getColorLocation(assetIdKey, assetIdValue);
        //     let assetRegistryObject: MyAssetRegistryObject = {
        //         tokenData: asset,
        //         hasLocation: true,
        //         tokenLocation: colorLocation
        //     }
        //     return assetRegistryObject
        // }
        // if (assetIdKey == "Native" && assetIdValue === "BNC") {
        //     let colorLocation = await getColorLocation(assetIdKey, assetIdValue);
        //     let assetRegistryObject: MyAssetRegistryObject = {
        //         tokenData: asset,
        //         hasLocation: true,
        //         tokenLocation: colorLocation
        //     }
        //     return assetRegistryObject
        // }
        const assetLocation = bncAssetLocations.find((location) => {
            let [locationIdKey, locationIdValue] = Object.entries(location[1])[0];
            if (locationIdKey.toLocaleLowerCase() == assetIdKey.toLocaleLowerCase() && locationIdValue == assetIdValue) {
                return true
            }
        }) 
        let assetRegistryObject:MyAssetRegistryObject = assetLocation ? {
            tokenData: asset,
            hasLocation: true,
            tokenLocation: assetLocation[0]
        } : {
            tokenData: asset,
            hasLocation: false,
        }

        console.log(assetRegistryObject)
        return assetRegistryObject
    }))
    const zenAssets = await queryZenAssets();
    zenAssets.forEach((zenAsset: any) => {

    })
    assetRegistry.forEach((asset) => {
        let data = asset.tokenData as MyAsset;
        console.log(data.localId)
        console.log(asset.hasLocation)
        if(asset.hasLocation){
            console.log(asset.tokenLocation)
        }
    })
    // console.log(assetRegistry.length)
    //Save to file
    // fs.writeFileSync(`../../assets/bnc/asset_registry.json`, JSON.stringify(assetRegistry, null, 2));
    const filePath = path.join(__dirname, 'asset_registry.json');
    fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2));
    // function writeToFile(assetRegistry) {
    //     fs.writeFileSync(filePath, JSON.stringify(assetRegistry, null, 2));
    // }

}

async function queryZenAssets() {
    const provider = new WsProvider(BifrostConfig.wss[0]);
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json');
    const tokensMeta = response.data.tokens;
    const zenTokens = tokensMeta.map((item: AssetMeta) => {
        return new Token(item);
    });
    const zenAssets = zenTokens.map((token: any) => {
        const tokenId = {
            Zenlink: {
                assetType: token.assetType,
                assetIndex: token.assetIndex,
            }
        }
        let tokenSymbol = token.symbol as string
        if (tokenSymbol.toLowerCase() == "ausd") {
            // console.log(token)
            const newAsset: MyAsset = {
                network: "kusama",
                chain: 2001,
                localId: tokenId,
                name: "KUSD",
                symbol: "KUSD",
                decimals: token.decimals
            }
            return newAsset
        } else {
            const newAsset: MyAsset = {
                network: "kusama",
                chain: 2001,
                localId: tokenId,
                name: token.name,
                symbol: token.symbol,
                decimals: token.decimals
            }
            return newAsset
        }
    })
    // console.log(zenAssets)
    return zenAssets
}
async function queryLocations() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    // let opt: ApiOptions = {
    //     provider: provider
    // }
    // const api = new ApiPromise(opt);
    await api.isReady;

    const locationEntries = await api.query.assetRegistry.locationToCurrencyIds.entries();
    const assetLocations = locationEntries.map(([location, id]) => {
        const currencyId = id.toJSON() as string;
        let locationData = (location.toHuman() as any)[0];
        // console.log(locationData)
        const junction = Object.keys(locationData.interior)[0]
        // console.log(locationData.interior)
        if (locationData.interior == "Here") {
            console.log("FOUND HERE")
            let newLocation = "here"
            return [newLocation, currencyId]
        } else if (junction == "X1") {
            const junctionData = locationData.interior[junction];
            const junctionType = Object.keys(junctionData)[0]
            let junctionValue = junctionData[junctionType]
            let newLocation: MyMultiLocation;
            let newJunction: MyJunction = {};
            // junctionValue = junctionValue.toString().replace(/,/g, "")
            // let newJunction: MyJunction = {};
            // newJunction[junctionType] = junctionValue;
            // let newLocation: MyMultiLocation = {
            //     X1: newJunction
            // }
            if (junctionType == "GeneralKey") {
                // let keys = Object.keys(junctions[x])[0]
                // let val = junctions[x][keys]
                newJunction = {
                    GeneralKey: {
                        length: junctionValue.length,
                        data: junctionValue.data
                    }
                };
                console.log
                
            } else {
                junctionValue = junctionValue.toString().replace(/,/g, "")
                
                newJunction[junctionType] = junctionValue;
                // junctionList.push(newJunction)
                // return newJunction
            }
            newLocation = {
                [junction]: newJunction
            }
            // let formattedLocation = api.createType('Junctions', newLocation).toJSON()
            return [newLocation, currencyId]
        } else {
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
                    console.log
                    console.log(newJunction)
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
            return [newLocation, currencyId]
        }
    })
    //Can't retrieve a MultiLocation from api.createType, so we will use MyMultilocation to represent a properly formatted MultiLocation
    let formattedAssetLocations = assetLocations.map(([location, currencyId]) => {
        return [location, currencyId as any]
    })
    return formattedAssetLocations

}

async function queryAssets(): Promise<MyAsset[]> {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;
    const parachainId = await (await api.query.parachainInfo.parachainId()).toJSON() as number;
    const bncAssets = await api.query.assetRegistry.currencyMetadatas.entries();
    const assets = bncAssets.map(([key, value]) => {
        const localId = (key.toHuman() as any)[0];
        const metaData = value.toHuman() as any;
        let localString = (key.toHuman() as any)[0];
        
        //Remove "," from values in VSBond array
        for (const [key, value] of Object.entries(localString)) {
            if (key === "VSBond" && Array.isArray(value)) {
                localString[key] = value.map((item: any) => {
                    return item.replace(/,/g, "")
                })
            }
        }
        const asset: MyAsset = {
            network: "kusama",
            chain: parachainId,
            localId: localString,
            name: metaData.name,
            symbol: metaData.symbol,
            decimals: metaData.decimals as string,
            minimalBalance: metaData.minimalBalance.toString().replace(/,/g, "")
        }
        return asset
        
    })
    return assets;
}


async function getColorLocation(assetIdKey: any, assetIdValue: any) {
    const colorAssets = JSON.parse(fs.readFileSync(`../bnc/kusama_2001_assets.json`, 'utf8'));
    const match = await colorAssets.find((colorAsset: any) => {
        let [colorKey, colorValue] = Object.entries(colorAsset.asset)[0];
        if (colorKey == assetIdKey && colorValue == assetIdValue) {
            return true
        }
    })
    let colorLocation = await JSON.parse(match.xcmInteriorKey)
    // console.log(location)
    let xtype = "x" + (colorLocation.length - 1).toString()
    let locationInterior = {
        // parents: 1,
        [xtype]: colorLocation.slice(1)
    }
    return locationInterior
}

export async function getAssets(): Promise<MyAssetRegistryObject[]> {
    return JSON.parse(fs.readFileSync(`../assets/bnc/asset_registry.json`, 'utf8'));
}

async function main() {
    // queryAssets();
    // queryLocations();
    await saveAssets();
    // await queryZenAssets()
    process.exit(0)
}

// main()