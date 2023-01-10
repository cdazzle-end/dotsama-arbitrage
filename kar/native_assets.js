exports.getNativeAsset= async (asset) => {
    // let fs = require('fs');
    // let file = fs.readFileSync('native_assets.txt', 'utf8');
    // file = JSON.parse(file);
    // console.log(file)
    if (asset == "KAR") {
        let assetLocation = {};
        assetLocation.NativeAsset = "KAR"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0080" }
        return assetLocation;
    }
    if (asset == "KUSD") {
        let assetLocation = {};
        assetLocation.NativeAsset = "KUSD"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0081" }
        return assetLocation;
    }
    if (asset == "LKSM") {
        let assetLocation = {};
        assetLocation.NativeAsset = "LKSM"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0083" }
        return assetLocation;
    }
    //MIGHT BE WRONG INDEX
    if (asset == "TAI") {
        let assetLocation = {};
        assetLocation.NativeAsset = "TAI"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0084" }
        return assetLocation;
    }
    // if (asset == "taiKSM") {
    //     let assetLocation = {};
    //     assetLocation.NativeAsset = "taiKSM"
    //     assetLocation.parents = "1";
    //     assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0085" }
    //     return assetLocation;
    // }
    if (asset == "KSM") {
        let assetLocation = {};
        assetLocation.NativeAsset = "KSM"
        assetLocation.parents = "1";
        assetLocation.location = "Here"
        return assetLocation;
    }
    if (asset == "BNC") {
        let assetLocation = {};
        assetLocation.NativeAsset = "BNC"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2001", GeneralKey: "0x0001" }
        return assetLocation;
    }
    if (asset == "VSKSM") {
        let assetLocation = {};
        assetLocation.NativeAsset = "VSKSM"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2001", GeneralKey: "0x0404" }
        return assetLocation;
    }
    if (asset == "PHA") {
        let assetLocation = {};
        assetLocation.NativeAsset = "PHA"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X1", Parachain: "2004" }
        return assetLocation;
    }
    if (asset == "KINT") {
        let assetLocation = {};
        assetLocation.NativeAsset = "KINT"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2092", GeneralKey: "0x000c" }
        return assetLocation;
    }
    if (asset == "KBTC") {
        let assetLocation = {};
        assetLocation.NativeAsset = "KBTC"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2092", GeneralKey: "0x000b" }
        return assetLocation;
    }
}

//TAIGA KSM AND 3USD
exports.getStableAsset = async (asset) => {
    //MIGHT BE WRONG INDEX
    if (asset == "0") {
        let assetLocation = {};
        assetLocation.StableAsset = "0"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0085" }
        return assetLocation;
    }
    if (asset == "1") {
        let assetLocation = {};
        assetLocation.StableAsset = "1"
        assetLocation.parents = "1";
        assetLocation.location = { xtype: "X2", Parachain: "2000", GeneralKey: "0x0086" }
        return assetLocation;
    }
}