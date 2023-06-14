//KAR, BIFROST, 
exports.getAssets = async (api) => {
    const assetRegistry = await api.query.assetRegistry.assetMetadatas.entries();
    let assetTokens = assetRegistry.map(([key, value]) => {
        let id = key.toHuman()[0];
        let val = value.toHuman()
        let asset = {};
        asset.localId = id;
        asset.name = val.name;
        asset.symbol = val.symbol;
        asset.decimals = val.decimals;
        asset.minimalBalance = val.minimalBalance;
        return asset;
    })
    return assetTokens;
}

//MOVR
exports.getAssets2 = async (api) => {
    let xcAssets = await api.query.assets.metadata.entries();
    xcAssets = xcAssets.map(([key, value]) => {
        let asset = {};
        const assetId = key.toHuman()[0];
        asset.localId = assetId;
        // console.log(value.toHuman())
        const assetData = value.toHuman();
        asset.deposit = assetData.deposit;
        asset.name = assetData.name;
        asset.symbol = assetData.symbol;
        asset.decimals = assetData.decimals;
        asset.isFrozen = assetData.isFrozen;
        return asset
    })
    return xcAssets;
}

//KAR, BIFROST
exports.getAssetLocations = async (api) => {
    const assetPaths = await api.query.assetRegistry.locationToCurrencyIds.entries();
    let assetLocations = assetPaths.map(([location, currencyId]) => {
        const locationInterior = location.toHuman()[0].interior
        const locationParent = location.toHuman()[0].parents
        let assetObj = {};
        let locationData = {};
        currencyKey = Object.keys(currencyId.toHuman());
        currencyValue = currencyId.toHuman()[currencyKey];
        assetObj[currencyKey] = currencyValue;
        assetObj.parents = locationParent;

        if (locationInterior == "Here") {
            locationData = "Here";
        } else { 
            for (x in locationInterior) {
                locationData.xtype = x;
                if (x == "X1") {
                    let key = Object.keys(locationInterior[x])
                    let value = locationInterior[x][key];
                    locationData[key] = value;
                } else {
                    for (var property in locationInterior[x]) {
                        let key = Object.keys(locationInterior[x][property])
                        let value = locationInterior[x][property][key];
                        locationData[key] = value;
                    }
                }
            }
        }
        assetObj.location = locationData
        return assetObj
    })
    return assetLocations;
}

//MOVR
exports.getAssetLocations2 = async (api) => {
    let xcAssetLocations = await api.query.assetManager.assetIdType.entries();
    xcAssetLocations = xcAssetLocations.map(([key, value]) => {
        const data = value.toHuman();
        const interior = data["Xcm"]["interior"];
        let assetLocationData = {};
        let location = {};
        assetLocationData.id = key.toHuman()[0]
        assetLocationData.parents = data["Xcm"]["parents"]
        if (interior == "Here") {
            // console.log("Asset from here")
            location = "Here";
        } else {
            for (x in interior) {
                location.xtype = x;
                if (x == "X1") {
                    let propertyData = interior[x];
                    let key = Object.keys(propertyData);
                    let val = propertyData[key];
                    location[key] = val;
                } else {
                    console.log(x)
                    let interiorData = interior[x]
                    for (property in interiorData) {
                        let propertyData = interiorData[property];
                        let key = Object.keys(propertyData);
                        let val = propertyData[key];
                        location[key] = val;
                    }
                }

            }
        }
        // console.log(location)
        assetLocationData.location = location;
        return assetLocationData;
    })
    return xcAssetLocations;
}
function testFunction() {
    console.log("Test")
}

// module.exports = {
//     testFunction: testFunction
// };