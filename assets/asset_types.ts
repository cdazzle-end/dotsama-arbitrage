// import { Junction } from '@polkadot/types/interfaces'
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api'
import { Junction, MultiLocation } from '@polkadot/types/interfaces'
import { AnyJson } from '@polkadot/types-codec/types';
import * as bncHandler from './bnc/asset_handler'

//This is the interface for the asset registry object. Combines token metadata and token location
export interface MyAssetRegistryObject {
    tokenData: MyAsset | CexAsset,
    hasLocation: boolean,
    tokenLocation?: any
}

export interface CexAsset {
    exchange: string,
    assetTicker: string,
    name: string,
    chain: string,
    precision: number,
    contractAddress: string,
}

//This is the unifying interface for all asset from all chains
export interface MyAsset {
    network: "kusama" | "polkadot"
    chain: number,
    localId: any,
    name: string,
    symbol: string,
    decimals: string,
    minimalBalance?: string,
    isFrozen?: boolean,
    deposit?: string,
    contractAddress?: string,
}

//MultiLocations 
export interface MyMultiLocation {
    [index: string]: any
    
}



//Use this to help convert data into '@polkadot/types/interfaces/Junction' when having trouble with the api.createType() method
export interface MyJunction {
    [index: string]: any,
    Parent?: boolean,
    Parachain?: number,
    AccountId32?: {
        networkId: string,
        id: String
    }
    AccountIndex64?: {
        networkId: string,
        index: String
    }
    AccountKey20?: {
        network: string,
        key: string
    }
    PalletInstance?: number,
    GeneralIndex?: number,
    OnlyChild?: boolean,
    Plurality?: {
        id: string,
        part: string
    }
}

const x: MyJunction = {
    Parachain: 100
}

// const y: Junction = {
//     Parachain: 100
// }
async function getAllAssets() {
    console.log(await bncHandler.getAssets())
}

async function main() {
    getAllAssets()
}

main()

// export enum j{
//     Parent,
//     Parachain: number,

// }

//   readonly type: 'Parent' | 'Parachain' | 'AccountId32' | 'AccountIndex64' | 'AccountKey20' | 'PalletInstance' | 'GeneralIndex' | 'GeneralKey' | 'OnlyChild' | 'Plurality';