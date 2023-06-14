import * as fs from 'fs';

import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import type { SignatureOptions } from '@polkadot/types/types'
import { options } from '@acala-network/api'
import { MultiLocation,VersionedMultiAssets, MultiAsset, Fungibility, FeeDetails, WeightLimitV2, XcmVersion, HrmpChannelId, ParaId } from '@polkadot/types/interfaces'
// import { MultiLocation } from '@polkadot/api-derive'
// import keyring from '@polkadot/ui-keyring';
import { cryptoWaitReady, mnemonicGenerate, signatureVerify } from '@polkadot/util-crypto';
import { stringToU8a, u8aToHex, stringToHex } from '@polkadot/util';
// For file storage where available, e.g. in Electron environments.
// This takes an path in the constructor, new FileStore('~./keyring-data')
import { FileStore } from '@polkadot/ui-keyring/stores';
import { KeyringJson } from '@polkadot/ui-keyring/types';
import { AnyTuple } from '@polkadot/types-codec/types';
import { StorageKey } from '@polkadot/types';
import {CurrencyId} from '@acala-network/types/interfaces'
import { Token } from '@zenlink-dex/sdk-core';
import { Currency } from '@zenlink-dex/sdk-core';
// import { getNativeAsset } from '../kar/native_assets';
const karAssetHandler = require('../kar/native_assets')

const dazzleDevAddress = 'GZsxo5sTCkmDyQbog1TBRuhGp8PuqWCbsREBcGHj3dNEFT2' //KSM format
const dazzleDev2 = 'GSTwhTKrHUrESyYpuMqL7ZFR2au5qQXN9nrgw9M2kNeTgA5'
const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
const wallet = process.env.WALLET;
let dazzleDev2Json = JSON.parse(fs.readFileSync('../transactions/wallet/dazzleDev2.json', 'utf8'))
const devPair = keyring.addFromJson(dazzleDev2Json)
const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
const api = new ApiPromise(options({ provider }));


async function toKsm() {
    // const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const api = new ApiPromise(options({ provider }));
    // await api.isReady;


    // let currencyIdString = {Token: 'KSM'}
    let currencyId = await api.createType('CurrencyId', { Token: 'KSM' })
    let amount = await api.createType('Balance', 2000000000)
    let dest = { V1: { interior: { X1: { AccountId32: { id: "0xaaf355d819ad304925911119e7b3d5ff9d2b0eb340ac1546f11cd2925498e072", network: { Any: "" } } } }, parents: 1 } }
    let destMultiLocation = await api.createType('VersionedMultiLocation', dest)
    let weight = await api.createType('WeightLimitV2')
    const xcmTx = await api.tx.xTokens.transfer(currencyId, amount, destMultiLocation, weight)

    // const data = await (await api.query.system.account(devPair.address)).toHuman() as any
    // let currentNonce = data.nonce;
    // const version = api.consts.system.version.toPrimitive() as any;
    // const versionSpec = version.specVersion
    // const currentBlockHash = await api.rpc.chain.getBlockHash()
    // const genBlockHash = await api.rpc.chain.getBlockHash(0)
    // const sigOptions = {
    //     nonce: currentNonce,
    //     blockHash: currentBlockHash,
    //     genesisHash: genBlockHash,
    //     runtimeVersion: versionSpec
    //     // era: 'immortal'
    // }
    // console.log(sigOptions)
    
    // console.log(wallet)
    // const signedTx = xcmTx.sign(devPair, sigOptions)
    
    // console.log(signedTx.toHuman())
    devPair.unlock(wallet)
    xcmTx.signAndSend(devPair)
        .then((hash) => {
            console.log(`Transation hash: ${hash}`)
        })
        .catch((error) => {
            console.log(`Error submitting tx: ${error}`)
        })
}

async function toStatemine() {
    await api.isReady;

    const currencyId0 = api.createType('CurrencyId', { ForeignAsset: 0 })
    const amount0 = api.createType('Balance', 1000000000)
    const currencyId1 = api.createType('CurrencyId', { Token: "KSM" })
    const amount1 = api.createType('Balance', 16000000000)

    const currencies = [[currencyId0, amount0],[currencyId1, amount1]];
    const fee = 1;
    const dest = api.createType('VersionedMultiLocation',
        { V1: {
                interior: {
                    X2:
                        [{ Parachain: 1000 }, { AccountId32: { id: "0xaaf355d819ad304925911119e7b3d5ff9d2b0eb340ac1546f11cd2925498e072", network: { Any: " " } } }]
                }, parents: 1
            }
        })
    const weight = api.createType('WeightLimitV2', 'Unlimited')

    const xcmTx = api.tx.xTokens.transferMulticurrencies(currencies, fee, dest, weight);

    devPair.unlock(wallet)
    xcmTx.signAndSend(devPair)
        .then((hash) => {
            console.log(`Transation hash: ${hash}`)
        })
        .catch((error) => {
            console.log(`Error submitting tx: ${error}`)
        })
}

async function toMovr() {
    await api.isReady;

    const currencyId = api.createType('CurrencyId', { ForeignAsset: 3 })
    const amount = api.createType('Balance', '10000000000000000')
    // const amount = 10000000000000000
    const destMultiLocation = api.createType('VersionedMultiLocation',
        {
            V1: {
                interior: {
                    X2:
                        [{ Parachain: 2023 }, { AccountKey20: { key: "0xae8da4a9792503f1ec97ed035e35133a9e65a61f", network: { Any: " " } } }]
                }, parents: 1
            }
        }
    )
    const weight = api.createType('WeightLimitV2')
    const xcmTx = await api.tx.xTokens.transfer(currencyId, amount, destMultiLocation, weight)

    devPair.unlock(wallet)
    xcmTx.signAndSend(devPair)
        .then((hash) => {
            console.log(`Transation hash: ${hash}`)
        })
        .catch((error) => {
            console.log(`Error submitting tx: ${error}`)
        })
}

async function toBnc() {
    const currencyId = api.createType('CurrencyId', { Token: 'BNC' })
    const amount = api.createType('Balance', 16000000000)
    const destMultiLocation = api.createType('VersionedMultiLocation',
        {
            V1: {
                interior: {
                    X2:
                        [{ Parachain: 2001 }, { AccountId32: { id: "0xaaf355d819ad304925911119e7b3d5ff9d2b0eb340ac1546f11cd2925498e072", network: { Any: " " } } }]
                }, parents: 1
            }
        }
    )
    const weight = api.createType('WeightLimitV2')
    const xcmTx = await api.tx.xTokens.transfer(currencyId, amount, destMultiLocation, weight)

    devPair.unlock(wallet)
    xcmTx.signAndSend(devPair)
        .then((hash) => {
            console.log(`Transation hash: ${hash}`)
        })
        .catch((error) => {
            console.log(`Error submitting tx: ${error}`)
        })
}

async function toHko() {
    await api.isReady;
    const currencyId = api.createType('CurrencyId', { ForeignAsset: 4 })
    const amount = api.createType('Balance', 300000000000)
    const destMultiLocation = api.createType('VersionedMultiLocation',
        {
            V1: {
                interior: {
                    X2:
                        [{ Parachain: 2085 }, { AccountId32: { id: "0xaaf355d819ad304925911119e7b3d5ff9d2b0eb340ac1546f11cd2925498e072", network: { Any: " " } } }]
                }, parents: 1
            }
        }
    )
    const weight = api.createType('WeightLimitV2')
    const xcmTx = await api.tx.xTokens.transfer(currencyId, amount, destMultiLocation, weight)

    devPair.unlock(wallet)
    xcmTx.signAndSend(devPair)
        .then((hash) => {
            console.log(`Transation hash: ${hash}`)
        })
        .catch((error) => {
            console.log(`Error submitting tx: ${error}`)
        })
}

async function karAssetList() {
    
}

async function test() {
    console.log(wallet)
}

async function main() {
    // toKsm()
    // toBnc()
    // toHko()
    // toStatemine()
    toMovr()
    // test()
}

main()