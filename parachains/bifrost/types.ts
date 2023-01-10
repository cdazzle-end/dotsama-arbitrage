export enum ChainId {
    BIFROST = 2001,
    MOONBEAM = 2004,
    ASTAR = 2006,
    MOONRIVER = 2023
}

export enum ChainName {
    BIFROST = 'bifrost',
    MOONBEAM = 'moonbeam',
    ASTAR = 'astar',
    MOONRIVER = 'moonriver'
}

export enum NetworkId {
    KUSAMA = 200,
    POLKADOT = 300,
}

export enum Native {
    N = 'N',
    L = 'L',
    P = 'P'
}

export interface ApiToken {
    isNative: Native,
    chainId: number,
    assetType: number,
    assetIndex: number,
    symbol: string,
    decimals: number,
    name: string,
    account: string
}

export interface SdkToken {
    networkId: NetworkId,
    address: string,
    chainId: number,
    assetType: number,
    assetIndex: number,
    symbol: string,
    decimals: number,
    name: string
}