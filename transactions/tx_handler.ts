import * as fs from 'fs';

import { Keyring, ApiPromise, WsProvider, } from '@polkadot/api';
import { MultiLocation, VersionedMultiAssets, MultiAsset, Fungibility, FeeDetails, WeightLimitV2, XcmVersion, HrmpChannelId, ParaId } from '@polkadot/types/interfaces'
// import { MultiLocation } from '@polkadot/api-derive'
import keyring from '@polkadot/ui-keyring';
import { cryptoWaitReady, mnemonicGenerate, signatureVerify } from '@polkadot/util-crypto';
import { stringToU8a, u8aToHex, stringToHex } from '@polkadot/util';
// For file storage where available, e.g. in Electron environments.
// This takes an path in the constructor, new FileStore('~./keyring-data')
import { FileStore } from '@polkadot/ui-keyring/stores';
import { KeyringJson } from '@polkadot/ui-keyring/types';
import { AnyTuple } from '@polkadot/types-codec/types';
import { StorageKey } from '@polkadot/types';
// import { getNativeAsset } from '../kar/native_assets';
const karAssetHandler = require('../kar/native_assets')

const dazzlePolk = 'GXeHEVY5SSJFQqcFmANaY3mTsRpcE9EUVzDcGpowbbe41ZZ'
const polkSub = '5G22cv9fT5RNVm2AV4MKgagmKH9aoZL4289UDcYrToP9K6hQ'
const dazzleDevAddress = 'GZsxo5sTCkmDyQbog1TBRuhGp8PuqWCbsREBcGHj3dNEFT2' //KSM format
const dazzleKsmAddress = 'FhJqSqbBHSizdDwJ3LMBcQNkVtE3wnzvdXpTQ3eNErLPJcy' //KSM format
//Basilisk Testnet 2090
//Rockmint 1000

async function createWallet() {
    const wallet = process.env.WALLET;
    const provider = new WsProvider('wss://kusama-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const storage = new FileStore('../transactions/keyring-data')

    let dazzleKsmJson = JSON.parse(fs.readFileSync('../transactions/test_add.json', 'utf8'));
    let dazzleDevJson = JSON.parse(fs.readFileSync('../transactions/dazzleDev1.json', 'utf8'))

    const dazzleKeyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    // dazzleKeyring
    
    let dazzleKsmPair = await dazzleKeyring.addFromJson(dazzleKsmJson);
    let devPair = await dazzleKeyring.addFromJson(dazzleDevJson)
    devPair.unlock(wallet)
    const signed = devPair.sign("Message test")
    const isValid = devPair.verify("Message test", signed, devPair.publicKey)
    console.log(isValid)
    
    // // Sign and send a transfer from Alice to Bob
    const transferAmount = 0.001 * Math.pow(10, 12)
    console.log(transferAmount)

    // api.tx.xc
    
    // dazzleKsmPair.unlock()
    // const txHash = await api.tx.balances
    //     .transfer(devPair.address, transferAmount)
    //     .signAndSend(dazzleKsmPair);

    // // Show the hash
    // console.log(`Submitted with hash ${txHash}`);
    
    // console.log("WALLET")
    // console.log(wallet)
    api.disconnect();
}

async function xcmTransfer() {
    const provider = new WsProvider('wss://rococo-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    const wallet = process.env.WALLET;
    let dazzleKsmJson = JSON.parse(fs.readFileSync('../transactions/test_add.json', 'utf8'));
    let dazzleDevJson = JSON.parse(fs.readFileSync('../transactions/dazzleDev1.json', 'utf8'))
    let devPair = await keyring.addFromJson(dazzleDevJson)
    devPair.unlock(wallet)
    
    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');
    const BOB = keyring.addFromUri('//Bob')
    const polk = keyring.addFromAddress(dazzlePolk)
    polk.address
    
    // polk.addressRa
    // 0xaee65bf22cdf1f98c91b6c176854d8072f1328e027d2e84d23607b517b1b9429
    // 0x475865484556593553534a46517163466d414e6159336d547352706345394555567a446347706f7762626534315a5a
    // console.log(stringToHex('F7fq1jMZkfuCuoMTyiEVAP2DMpMt18WopgBqTJznLihLNbZ'))
    // console.log(u8aToHex(stringToU8a('sFbqBjxPBgfBmGPgxSWqweKLnHawKvMc9K1JNgcijAZjt1X')))
    // create the message, actual signature and verify
    const message = stringToU8a('this is our message');
    const signature = alice.sign(message);
    const isValid = alice.verify(message, signature, alice.publicKey);
    const devAddress = api.createType('AccountId', devPair.address)
    console.log(devAddress.toHex())

    const dest2 = {
        parents: 0,
        interior: {
            X1: {
                Parachain: 1000
            }
        }
    }
    const dLoc = api.createType('MultiLocation', dest2)
    console.log("destination")
    console.log(dLoc.toHuman())

    const ben2 = {
        parents: 0,
        interior: {
            X1: {
                AccountId32: {
                    id: "0xaee65bf22cdf1f98c91b6c176854d8072f1328e027d2e84d23607b517b1b9429",
                    network: {
                        Any: ""
                    }
                }
            }
        }
    }
    const bLoc = api.createType('MultiLocation', ben2)
    console.log("beneficiary")
    console.log(bLoc.toHuman())
    const txAssets2 = {
        id: {
            Concrete: {
                parents: '0',
                interior: 'Here'
            }
        },
        fungibility: {
            Fungible: "20000000000"
        }
    }
    const mAsset: MultiAsset = api.createType('MultiAsset', txAssets2);
    // mAsset.
    console.log("Asset")
    console.log(mAsset.toHuman())
    const fee = 0
    const weight = {
        "Unlimited": ""
    }

    // api.query.
    // devPair.unlock(wallet)
    // console.log(api.tx.xcmPallet.limitedTeleportAssets.meta.fields)
    // api.tx.xcmPallet.limitedTeleportAssets.meta.fields.forEach((field, index, siField) => {
    //     console.log(field.name.toHuman())
    //     console.log(field.type.toHuman())
    //     console.log(field.typeName.toHuman())
    //     console.log(field.docs.toHuman())
    //     // console.log(index)
    //     // console.log(siField)
    // })3.2198
    const vdLocTemp = {
        V1: dLoc
    }
    const vbLocTemp = {
        V1: bLoc
    }
    const vmAssetsTemp = {
        V1: [mAsset]
    }
    const vdLoc = api.createType('VersionedMultiLocation', vdLocTemp)
    const vbLoc = api.createType('VersionedMultiLocation', vbLocTemp)
    const vmAssets = api.createType('VersionedMultiAssets', vmAssetsTemp)

    console.log(vdLoc.toHuman())
    console.log(vbLoc.toHuman())
    console.log(vmAssets.toHuman())

    // console.log(devPair.)

    const xcmTeleport = await api.tx.xcmPallet
        .limitedTeleportAssets(vdLoc, vbLoc, vmAssets, fee, weight)
        .signAndSend(devPair)
    
    console.log(`Submitted with hash ${xcmTeleport}`)
    // console.log(alice.);
    // console.log(BOB.address)
    // const txHash = await api.tx.balances
    //     .transfer(BOB.address, 1)
    //     .signAndSend(alice);
    
    // console.log(`Submitted with hash ${txHash}`);
    
}

async function reserveTransfer(chainId: number) {
    const provider = new WsProvider('wss://rococo-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    const wallet = process.env.WALLET;
    let dazzleKsmJson = JSON.parse(fs.readFileSync('../transactions/test_add.json', 'utf8'));
    let dazzleDevJson = JSON.parse(fs.readFileSync('../transactions/dazzleDev1.json', 'utf8'))
    let devPair = await keyring.addFromJson(dazzleDevJson)
    devPair.unlock(wallet)
    const dest = {
        V0: {
            interior: {
                X1: {
                    Parachain: chainId
                }
            }
        }   
    }
    const beneficiary = {
        V0: {
            interior: {
                X1: {
                    AccountId32: {
                        id: "0xb09ae874f4b65252a3878749fa86f474131a893b70e8b291cf4a34aeeec40952",
                        network: {
                            Any: ""
                        }
                    }
                }
            }
        }
    }
    const xcAsset = {
        V0: [{
            id: {
                Concrete: {
                    parents: '0',
                    interior: 'Here'
                }
            },
            fungibility: {
                Fungible: "20000000000"
            }
        }]
    }
    const vDest = api.createType('VersionedMultiLocation', dest)
    const vBeneficiary = api.createType('VersionedMultiLocation', beneficiary)
    const vxcAsset = api.createType('VersionedMultiAssets', xcAsset)
    const fee = 0
    const weight = {
        "Unlimited": ""
    }
    console.log(vDest.toHuman())
    console.log(vBeneficiary.toHuman())
    console.log(vxcAsset.toHuman())
    const xcmReserveTransfer = await api.tx.xcmPallet
        .limitedReserveTransferAssets(vDest, vBeneficiary, vxcAsset, fee, weight)
        .signAndSend(devPair)
    
    console.log(xcmReserveTransfer.toHuman())
}
async function xcTest() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const provider = new WsProvider('wss://rococo-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady


}

async function getHrmpChannels() {
    const provider = new WsProvider('wss://kusama-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady

    const channelKeys = await api.query.hrmp.hrmpChannels.keys();
    let hrmpChannels: HrmpChannelId[] = [];
    channelKeys.forEach((channelKey: StorageKey<AnyTuple>) => {

        const key = channelKey as any
        const sender = key.toHuman()[0]['sender'].replace(/,/g, "");
        const receiver = key.toHuman()[0]['recipient'].replace(/,/g, "");
        console.log(sender)
        console.log(receiver)
        let hrmpData = {
            sender: sender,
            receiver: receiver
        }

        let hrmp: HrmpChannelId = api.createType('HrmpChannelId', hrmpData)
        hrmpChannels.push(hrmp)
        // console.log(hrmp.toHuman())
    })
    // console.log(hrmpChannels)
    // hrmpChannels.forEach((x) => {
    //     console.log(x.toHuman())
    // })
    fs.writeFileSync('../transactions/hrmpChannels', JSON.stringify(hrmpChannels))
}

//Find asset in asset registry for start chain
//Find asset in destination chain
//Find bridge between to chains
//
async function karuraCrossChain(localAssetId: string) {
    // const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const api = await ApiPromise.create({ provider: provider });
    //start chain: 2000
    //dest chain: 2023
    const destChain = 1000;

    //Parse local asset id
    let assetIdString = localAssetId.replace(/{/g, "").replace(/}/g, "")
    let assetId = assetIdString.split(":")
    console.log(assetId[0])
    if (assetId[0] == "NativeAssetId") {
        let assetSymbol = assetId[2].replace(/"/g, "")
        console.log(assetSymbol)
        // const karNativeAssets = getNativeAsset();
        let karAsset = await karAssetHandler.getNativeAsset(assetSymbol);
        console.log(karAsset.location)

        // karAsset.location
    }

    // let channels = JSON.parse(fs.readFileSync('../transactions/hrmpChannels', 'utf8'))
    // const karChannels = channels.filter((channel: HrmpChannelId) => {
    //     if (channel.sender as any == 2000 && channel.receiver as any == destChain) {
    //         return channel
    //     }
    // })
    // console.log(karChannels)
    
    // channels.forEach((channel: HrmpChannelId) => {
    //     console.log(channel)
    // })
}
async function main() {
    // createWallet()
    // xcmTransfer()
    // reserveTransfer(2090)
    // xcTest()
    // getHrmpChannels()
    karuraCrossChain("{NativeAssetId:{\"Token\":\"KSM\"}")
}

main()