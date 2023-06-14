export interface MyLp{
    chainId: number,
    contractAddress?: string,
    poolAssets: any[]
    liquidityStats: string[]
}

export interface CexLp{
    exchange: string,
    assetTicker: string,
    price: [number, number],
    priceDecimals: [number, number],
}

export interface StableSwapPool{
    chainId: number,
    contractAddress?: string,
    poolAssets: any[],
    liquidityStats: string[],
    tokenPrecisions: string[],
    swapFee: string,
    a: number,
    aPrecision: number,
    aBlock: number,
    futureA: number,
    futureABlock: number,
    totalSupply: string,
    poolPrecision: string,
}