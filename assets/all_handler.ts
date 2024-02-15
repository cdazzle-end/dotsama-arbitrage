import { MyAssetRegistryObject, MyMultiLocation, CexAsset, MyAsset } from './asset_types'
// import * as bncHandler from './bnc/asset_handler'
// import * as karHandler from './kar/asset_handler'
// import * as hkoHandler from './hko/asset_handler'
// import * as movrHandler from './movr/asset_handler'
// import * as sdnHandler from './sdn/asset_handler'
// import * as kucoinHandler from './kucoin/asset_handler'
// import * as mgxHandler from './mgx/asset_handler'
// import * as bsxHandler from './bsx/asset_handler'
import * as fs from 'fs';
import path from 'path';

// async function getAllAssets() {
//     const bncAssets = await bncHandler.getAssets()
//     const karAssets = await karHandler.getAssets()
//     const hkoAssets = await hkoHandler.getAssets()
//     const movrAssets = await movrHandler.getAssets()
//     const sdnAssets = await sdnHandler.getAssets()
//     const kucoinAssets = await kucoinHandler.getAssets();
//     const mgxAssets = await mgxHandler.getAssets();
//     let allAssets = bncAssets.concat(karAssets).concat(hkoAssets).concat(movrAssets).concat(sdnAssets).concat(kucoinAssets).concat(mgxAssets)
//     console.log(allAssets)
//     let assetBuckets: { [key: string]: MyAssetRegistryObject[] } = {};
//     allAssets.forEach((token: any) => {
//         let locationString = JSON.stringify(token.tokenLocation);
//         if (assetBuckets[locationString] == undefined) {
//             assetBuckets[locationString] = []
//         }
//         assetBuckets[locationString].push(token)
//     })
//     Object.entries(assetBuckets).forEach(([key, value]) => {
//         console.log(key)
//         value.forEach((token: MyAssetRegistryObject) => {
//             if ('exchange' in token.tokenData) {
//                 console.log(token.tokenData.name + " " + token.tokenData.exchange);
//             } else {
//                 console.log(token.tokenData.name + " " + token.tokenData.chain);
//             }
//         });
//     })

// }

async function getAllAssets2() {
    const bncAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/bnc/asset_registry.json'), 'utf8'))
    const karAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kar/asset_registry.json'), 'utf8'))
    const hkoAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/hko/asset_registry.json'), 'utf8'))
    const movrAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/movr/asset_registry.json'), 'utf8'))
    const sdnAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/sdn/asset_registry.json'), 'utf8'))
    const kucoinAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kucoin/asset_registry.json'), 'utf8'))
    const mgxAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/mgx/asset_registry.json'), 'utf8'))
    const bsxAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/bsx/asset_registry.json'), 'utf8'))
    const statemineAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/statemine/asset_registry.json'), 'utf8'))
    const crustAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/crust/asset_registry.json'), 'utf8'))
    const kintAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kint/asset_registry.json'), 'utf8'))
    let allAssets = bncAssets.concat(karAssets).concat(hkoAssets).concat(movrAssets).concat(sdnAssets).concat(kucoinAssets).concat(mgxAssets).concat(bsxAssets)
    allAssets = allAssets.concat(crustAssets).concat(kintAssets)
    // allAssets = allAssets.concat(statemineAssets)
    let assetBuckets: { [key: string]: MyAssetRegistryObject[] } = {};
    allAssets.forEach((token: any) => {
        let locationString = JSON.stringify(token.tokenLocation);
        if (assetBuckets[locationString] == undefined) {
            assetBuckets[locationString] = []
        }
        assetBuckets[locationString].push(token)
    })

    const sortedKeys = Object.keys(assetBuckets).sort((keyA, keyB) => {
        const nameA = assetBuckets[keyA][0]?.tokenData.name || "";
        const nameB = assetBuckets[keyB][0]?.tokenData.name || "";
        return nameA.localeCompare(nameB);
    })

    sortedKeys.forEach((key) => {
        console.log(key)
        assetBuckets[key].forEach((token: MyAssetRegistryObject) => {
            if ('exchange' in token.tokenData) {
                console.log(token.tokenData.name + " " + token.tokenData.exchange);
            } else {
                console.log(token.tokenData.name + " " + token.tokenData.chain);
            }
        });
        console.log("-----------------")
        
    })

    // Object.entries(sortedKeys).forEach(([key, value]) => {
    //     console.log(key)
    //     value.forEach((token: MyAssetRegistryObject) => {
    //         if ('exchange' in token.tokenData) {
    //             console.log(token.tokenData.name + " " + token.tokenData.exchange);
    //         } else {
    //             console.log(token.tokenData.name + " " + token.tokenData.chain);
    //         }
    //     });
    // })
    // Object.entries(assetBuckets).forEach(([key, value]) => {
    //     console.log(key)
    //     value.forEach((token: MyAssetRegistryObject) => {
    //         if ('exchange' in token.tokenData) {
    //             console.log(token.tokenData.name + " " + token.tokenData.exchange);
    //         } else {
    //             console.log(token.tokenData.name + " " + token.tokenData.chain);
    //         }
    //     });
    // })
}

async function testJson() {
    const asset: MyAsset = {
        network: "kusama",
        chain: 1,
        localId: "test",
        name: "test",
        symbol: "ts",
        decimals: "1"
    }

    const ar: MyAssetRegistryObject = {
        tokenData: asset,
        hasLocation: false,
    }

    console.log(JSON.stringify(ar, null, 2))
}

function getLocationString(token: any): string {
    let location = token.tokenLocation
    let locationString = JSON.stringify(location);
    return locationString
}

async function saveAssets() {
    // await bncHandler.saveAssets()
    // await karHandler.saveAssets()
    // await hkoHandler.saveAssets()
    // await movrHandler.saveAssets()
    // await sdnHandler.saveAssets()
    // await kucoinHandler.saveAssets()
    // await mgxHandler.saveAssets()
    // await bsxHandler.saveAssets()
}
async function buildAllAssetRegistry() {
    const filePath = path.join(__dirname, 'asset_registry.json')
    const bncAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/bnc/asset_registry.json'), 'utf8'))
    const karAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kar/asset_registry.json'), 'utf8'))
    const hkoAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/hko/asset_registry.json'), 'utf8'))
    const movrAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/movr/asset_registry.json'), 'utf8'))
    const sdnAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/sdn/asset_registry.json'), 'utf8'))
    const kucoinAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kucoin/asset_registry.json'), 'utf8'))
    const mgxAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/mgx/asset_registry.json'), 'utf8'))
    const bsxAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/bsx/asset_registry.json'), 'utf8'))
    const statemineAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/statemine/asset_registry.json'), 'utf8'))
    const crustAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/crust/asset_registry.json'), 'utf8'))
    const kintAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/kint/asset_registry.json'), 'utf8'))
    const ksmAssets = JSON.parse(fs.readFileSync(path.join(__dirname, '/ksm/asset_registry.json'), 'utf8'))
    let allAssets = bncAssets.concat(karAssets).concat(hkoAssets).concat(movrAssets).concat(sdnAssets).concat(kucoinAssets).concat(mgxAssets).concat(bsxAssets)
    allAssets = allAssets.concat(statemineAssets).concat(crustAssets).concat(kintAssets).concat(ksmAssets)
    fs.writeFileSync('./allAssets.json', JSON.stringify(allAssets, null, 2), 'utf8')
}

async function main() {
    // saveAssets()
    // getAllAssets()
    // testJson()
    // getAllAssets()
    // getAllAssets2()
    // saveAssets()
    buildAllAssetRegistry()
}

main()  