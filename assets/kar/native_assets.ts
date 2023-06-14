export async function getNativeAsset(asset: string): Promise<any> {
    if (asset == "KAR") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                }, {
                    GeneralKey: "0x0080"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "KUSD") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                }, {
                    GeneralKey: "0x0081"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "LKSM") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                },{
                    GeneralKey: "0x0083"
                }]
            }
        }
        return assetLocation;
    }
    //MIGHT BE WRONG INDEX
    if (asset == "TAI") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                },{
                    GeneralKey: "0x0084"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "KSM") {
        const assetLocation = "here";
        return assetLocation;
        
    }
    if (asset == "BNC") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2001",
                },{
                    GeneralKey: "0x0001"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "VSKSM") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2001",
                },{
                    GeneralKey: "0x0404"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "PHA") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X1: {
                    Parachain: "2004"
                }
            }
        }
        return assetLocation;
    }
    if (asset == "KINT") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2092",
                },{
                    GeneralKey: "0x000c"
                }]
            }
        }

        return assetLocation;
    }
    if (asset == "KBTC") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2092",
                },{
                    GeneralKey: "0x000b"
                }]
            }
        }

        return assetLocation;
    }
}

//TAIGA KSM AND 3USD
export async function getStableAsset(asset: string): Promise<any> {
    //MIGHT BE WRONG INDEX
    if (asset == "0") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                },{
                    GeneralKey: "0x0085"
                }]
            }
        }
        return assetLocation;
    }
    if (asset == "1") {
        const assetLocation = {
            // parents: "1",
            interior: {
                X2: [{
                    Parachain: "2000",
                },{
                    GeneralKey: "0x0086"
                }]
            }
        }
        return assetLocation;
    }
}