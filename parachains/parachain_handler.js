//KUSAMA:
//Get Kusama parachain ID list: storage.paras.parachains

//Get DMP/UMP channels storage.dmp.messagequeueheads storage.ump...

//Get HRMP channels storage.hrmp.hrmpChannels OR hrmpEgressChannelIndex hrmpIngressChannelIndex

var apiHelper = require('./api_utils')
var hexConversion = require('./hex.js')
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { isObject, isDeepStrictEqual } = require('util');
const karHandler = require('../kar/kar_handler.js')
const bncHandler = require('../parachains/bifrost.js')
const movrHandler = require('../movr/movr_handler.js')
const kcHandler = require('../kucoin/kc_handler')

async function kusama() {
    const provider = new WsProvider('wss://kusama-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });

    const parachainIds = await (await api.query.paras.parachains()).toHuman();
    // parachainIds.forEach((value, index) => {
    //     console.log(index)
    //     console.log(value)
    // })

    const dmpChannels = await api.query.dmp.downwardMessageQueueHeads.entries();
    dmpChannels.map(([key, entry]) => {
        
        // console.log(`${key.args}: ${entry}`)
        // console.log("Entry: " + entry)
    })

    const hrmpChannels = await api.query.hrmp.hrmpChannels.entries();
    hrmpChannels.map(([key, entry]) => {
        let channels = key.args[0].toHuman();
        let sender = channels.sender;
        let recipient = channels.recipient;
        console.log(`${sender} -> ${recipient}`)
    })

    const egressChannels = await api.query.hrmp.hrmpEgressChannelsIndex.entries();
    egressChannels.map(([val, entry]) => {
        // console.log("Sender: " + val.toHuman())
        // console.log("Recipients: " + entry.toHuman())
    })

    const ingressChannels = await api.query.hrmp.hrmpIngressChannelsIndex.entries();
    ingressChannels.map(([val, entry]) => {
        // console.log("Recipient: " + val.toHuman())
        // console.log("Senders: " + entry.toHuman())
    })

}

//KARURA:
//Get parachain ID storage.parachainInfo.parachainId

//Get asset registry storage.assetRegistry.assetMetadatas

//Get asset paths storage.assetRegistry.locationToCurrencyIds

//Pair XC asset data with asset paths and save to file (kar_x_tokens)

//Build global asset registry database. save to file.


async function karura() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });
    
    const paraId = await (await api.query.parachainInfo.parachainId()).toHuman();
    const assetTokens = await apiHelper.getAssets(api)
    const assetLocations = await apiHelper.getAssetLocations(api)

    
    let assetDatas = []
    // PAIR ASSET LOCATION WITH TOKEN DATA FROM REGISTRY
    for (x in assetLocations) {
        let assetData = {};
        assetData.locationData = assetLocations[x];
        let foreignAssetId = assetLocations[x]["ForeignAsset"];
        for (y in assetTokens) {
            let tokenId = assetTokens[y]["localId"]["ForeignAssetId"];
            if (foreignAssetId == tokenId) {
                console.log("MATCHED DATA")
                assetData.tokenData = assetTokens[y];
            }
        }
        // console.log(assetData)
        assetDatas.push(assetData)
    }

    let xcAssetData = {};
    xcAssetData.parachainId = paraId;
    xcAssetData.xcAssets = assetDatas;
    console.log(xcAssetData.xcAssets)
    // console.log(assetDatas)
    // console.log(assetDatas.length)
    let fs = require('fs');
    // fs.writeFileSync('./x_token_data/kar_x_tokens', JSON.stringify(xcAssetData), 'utf8')
}

async function read_karura(){
    let fs = require('fs');
    let assets = fs.readFileSync('kar_x_tokens', 'utf8');
    assets = JSON.parse(assets);
    console.log(assets)
}

//MOONRIVER:
//Get parachain ID

//Get xc assets data storage.assets.metadata 108457044225666871745333730479173774551

//Get xc asset paths storage.assetManager.assetIdType

async function moonriver() {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    const paraId = await (await api.query.parachainInfo.parachainId()).toHuman();

    let xcAssets = await apiHelper.getAssets2(api);
    let xcAssetLocations = await apiHelper.getAssetLocations2(api)

    let xcAssetObjs = [];
    for (loc in xcAssetLocations) {
        let xcAssetObj = {}
        xcAssetObj.locationData = xcAssetLocations[loc];
        const locId = xcAssetLocations[loc]["id"];
        for (a in xcAssets) {
            const aId = xcAssets[a]["localId"]
            if (locId == aId) {
                xcAssetObj.tokenData = xcAssets[a];
            }
        }
        xcAssetObjs.push(xcAssetObj);
        console.log(xcAssetObj)
    }

    let xcAssetData = {};
    xcAssetData.parachainId = paraId;
    xcAssetData.xcAssets = xcAssetObjs;
    console.log(xcAssetObjs)
    // console.log(xcAssets)

    let fs = require('fs');
    // fs.writeFileSync('./x_token_data/movr_x_tokens', JSON.stringify(xcAssetData), 'utf8')
}

async function movr_addresses() {
    const provider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    const api = await ApiPromise.create({ provider: provider });

    let xcAssets = await api.query.assets.asset.entries();
    xcAssets.map(([key, val]) => {
        console.log(key.toHuman())
        // console.log(val.toHuman())
        let num = key.toHuman()[0].replace(/,/g, '')
        console.log(num)
        console.log(parseInt(num))
        // let hex = decToHex(parseInt(num))
        let hex2 = hexConversion.decToHex(num);
        
        // console.log(hex)
        console.log(hex2)
        hex2 = hex2.slice(2)
        console.log(hex2)
        // console.log("0xFFFFFFFF" + hex)
        console.log("0xFFFFFFFF" + hex2)
    })

    //0xFFFFFFFFF075423BE54811ECB478E911F22DDE7D

}

async function crossChainAssets() {
    let fs = require('fs');
    let movrTokens = fs.readFileSync('./x_token_data/movr_x_tokens', 'utf8');
    let karTokens = fs.readFileSync('./x_token_data/kar_x_tokens', 'utf8');
    let bncTokens = fs.readFileSync('./x_token_data/bnc_k_x_tokens', 'utf8')
    movrTokens = JSON.parse(movrTokens);
    karTokens = JSON.parse(karTokens);
    bncTokens = JSON.parse(bncTokens);
    // console.log(karTokens)

    let crossChainAssets = fs.readFileSync('xc_assets', 'utf8');
    crossChainAssets = JSON.parse(crossChainAssets)

    for (index in karTokens.xcAssets) {
        let token = karTokens.xcAssets[index];
        await addXcAsset(crossChainAssets, token, karTokens.parachainId)
    }
    for (index in movrTokens.xcAssets) {
        let token = movrTokens.xcAssets[index];
        await addXcAsset(crossChainAssets, token, movrTokens.parachainId)
    }
    for (index in bncTokens.xcAssets) {
        let token = bncTokens.xcAssets[index];
        // console.log(bncTokens.parachainId)
        await addXcAsset(crossChainAssets, token, bncTokens.parachainId)
        // console.log(token)
    }
    console.log(crossChainAssets)
    console.log(crossChainAssets.length)

    // fs.writeFileSync('xc_assets', JSON.stringify(crossChainAssets), 'utf8')


}

class CrossChanAsset{
    constructor(location) {
        this.location = location;
        this.name = []; //If parachain has different name for asset, will add to array
        this.symbol = [];
        this.parachains = [];
    }
}

async function containsXcAsset(xcList, token) {
    let test = { xtype: 'X1', Parachain: '2,007' };
    // console.log("token.locationData.location")
    // console.log(token.locationData.location)
    // for (x in xcList) {
    //     console.log(xcList[x].location)
    //     if (xcList[x].location == token.locationData.location) {
    //         console.log("XC TOKEN")
    //         console.log(xcList[x])
    //         return true;
    //     }
    // }
    // return false;
    let xcLocations = [];
    for (x in xcList) {
        xcLocations.push(xcList[x].location)
    }
    // console.log(xcLocations)
    console.log("Testing")
    for (x in xcLocations) {
        let loc = xcLocations[x];
        if (await locIsEqual(loc, token.locationData.location)) {
            console.log("MaTcH")
            // console.log(loc)
            // console.log(token.locationData.location)
            return true;
        }
        // if (loc["Parachain"] == test["Parachain"]) {
        //     console.log("MATCh")
        // }
    }
    return false;
}

async function locIsEqual(loc1, loc2) {
    var props1 = Object.getOwnPropertyNames(loc1)
    var props2 = Object.getOwnPropertyNames(loc2)
    if (props1.length != props2.length) {
        return false;
    }
    for (var i = 0; i < props1.length; i++){
        let val1 = loc1[props1[i]];
        let val2 = loc2[props1[i]];
        // let isObjects = (val1 !== null && typeof val1 === 'object') && (val2 !== null && typeof val2 === 'object');
        let isObjects = await isObject2(val1) && await isObject2(val2);

        if (isObjects && !isDeepStrictEqual(val1, val2) || !isObjects && val1 !== val2) {
            return false;
        }
    }
    return true;
}

async function isObject2(object) {
    return object != null && typeof object === 'object';
}

async function addXcAsset(xcList, asset, paraId) {
    // console.log("Current name: " + asset.tokenData.name)
    // console.log(`Current location:`)
    // console.log(asset.locationData.location)
    let found = false;
    // xcList.parachains = [];
    for (x in xcList) {
        // xcList[x].parachains = [];
        if (await locIsEqual(xcList[x].location, asset.locationData.location)) { // ADD NAME TO EXISTING ASSET
            // console.log("Found match")
            if (asset.tokenData != undefined && !xcList[x].name.includes(asset.tokenData.name)) {
                xcList[x].name.push(asset.tokenData.name)
            }
            if (asset.tokenData != undefined && !xcList[x].symbol.includes(asset.tokenData.symbol)) {
                xcList[x].symbol.push(asset.tokenData.symbol)
            }
            if (!xcList[x].parachains.includes(paraId)) {
                xcList[x].parachains.push(paraId)
            }
            found = true;
            // console.log(xcList[x])
        }
    }
    console.log("FOUND : " + found)
    if (!found) { //ADD NEW CROSS CHAIN ASSET
        // console.log("New asset")
        let xcAsset = new CrossChanAsset(asset.locationData.location);
        // console.log(xcAsset)
        if (xcAsset.tokenData != undefined) {
            xcAsset.name.push(asset.tokenData.name)
            xcAsset.symbol.push(asset.tokenData.symbol)
        }
        // console.log("Para ID: " + paraId)
        xcAsset.parachains.push(paraId)
        xcList.push(xcAsset)
    }
    // console.log(xcList)
    // return false;
}

//Call update functions from each chain
async function updateAllLiqPools() {
    karHandler.save_dex_to_file().then(() => console.log("kar complete"));
    bncHandler.saveZenLiqPools().then(() => console.log("bnc complete"));
    movrHandler.updateLiqPools().then(() => console.log("movr complete"));
    kcHandler.saveKucoinAssets().then(() => console.log("kucoin complete"));
}

async function main() {
    
    // karura()
    // read_karura();
    // moonriver()
    // kusama()
    // crossChainAssets()
    // movr_addresses()
    updateAllLiqPools()

}
main().then(() => console.log("Complete"))
