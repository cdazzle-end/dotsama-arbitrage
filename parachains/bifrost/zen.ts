import { WsProvider, Keyring } from '@polkadot/api';
import { ModuleBApi, BifrostConfig } from '@zenlink-dex/sdk-api';
import { Percent, Token, TokenAmount, TradeType, StablePair, StableSwap, AssetMeta, StandardPair } from '@zenlink-dex/sdk-core';
import { firstValueFrom } from 'rxjs';
import axios from 'axios';
import { writeFile, readdir, mkdir } from 'node:fs/promises';
import { ApiToken, ChainId, ChainName, Native, NetworkId, SdkToken } from "./types";

async function main() {
    // const provider = new WsProvider(BifrostConfig.wss[0]);
    // const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    // await provider.isReady;
    // const dexApi = new ModuleBApi(
    //     provider,
    //     // BifrostConfig
    // );
    // await dexApi.initApi(); // init the api;
    // const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    // const tokensMeta = response.data.tokens;

    // // generate Tokens
    // const tokens = tokensMeta.map((item: AssetMeta) => {
    //     let token = new Token(item);
    //     console.log(token)
    //     return token;
    // });

    // // query the standard pair and pool
    // // const standardPairs = await dexApi.standardPairOfTokens(tokens);
    // // console.log(standardPairs)
    // const standardPairs = await firstValueFrom(dexApi.standardPairOfTokens(tokens));
    // const standardPools = await firstValueFrom(dexApi.standardPoolOfPairs(standardPairs));

    // let i = 0;
    // console.log(standardPools.length)
    // while (i < standardPools.length) {
    //     console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`)
    //     console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
    // console.log(standardPairs.length)
    // while (i < standardPairs.length) {
    //     console.log(standardPairs[i])
    //     // console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
    // getResponse()
    getLiqPools()
}

async function getResponse() {

    const CHAINID_TO_NETWORKID = (id: ChainId): NetworkId => {
        switch (id) {
            case ChainId.BIFROST:
                return NetworkId.KUSAMA
            case ChainId.MOONRIVER:
                return NetworkId.KUSAMA
            case ChainId.MOONBEAM:
                return NetworkId.POLKADOT
            case ChainId.ASTAR:
                return NetworkId.POLKADOT
            default:
                throw new Error(`Unknown chain id: ${id}`);
        }
    }

    const CHAINID_TO_CHAINNAME = (id: ChainId): ChainName => {
        switch (id) {
            case ChainId.BIFROST:
                return ChainName.BIFROST
            case ChainId.MOONBEAM:
                return ChainName.MOONBEAM
            case ChainId.ASTAR:
                return ChainName.ASTAR
            case ChainId.MOONRIVER:
                return ChainName.MOONRIVER
            default:
                throw new Error(`Unknown chain id: ${id}`);
        }
    }
    const API_URL = 'https://api.zenlink.pro/rpc';

    const response = await axios.post(API_URL, {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "asset.get",
        "params": []
        // "method": "zenlinkProtocol_getAllAssets",
        // "params": []
    });

    const allTokens = response.data.result || [];
    const sdkTokens = allTokens
        .filter((token: { chainId: number }) => token.chainId === 2001)
        .filter((token: { isNative: Native; }) => token.isNative === Native.P || token.isNative === Native.N)
        // .map((token: { chainId: any; assetType: any; assetIndex: any; symbol: any; decimals: any; name: any; account: any; }) => {
        //     let networkId = null;
        //     const { chainId, assetType, assetIndex, symbol, decimals, name, account } = token;
        //     networkId = CHAINID_TO_NETWORKID(chainId)

        //     return {
        //         networkId,
        //         address: account,
        //         chainId,
        //         assetType,
        //         assetIndex,
        //         symbol,
        //         decimals,
        //         name,
        //     };
        // });

    console.log(response)
}

export async function getTokenList() {
    
    
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item: AssetMeta) => {
        let token = new Token(item);
        // console.log(token)
        return token;
    });

    return tokens;
}

export async function getLiqPools() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item: AssetMeta) => {
        let token = new Token(item);
        // console.log(token)
        // console.log(token)
        return token;
    });
    

    // query the standard pair and pool
    
    const standardPairs = await firstValueFrom(dexApi.standardPairOfTokens(tokens));
    // console.log(standardPairs)
    // standardPairs.forEach((pair) => {
    //     console.log(pair["lpToken"]);
    //     console.log(pair["token0"]);
    //     console.log(pair["token1"])
    // })

    
    // console.log(standardPairs.length)
    let standardPairArray: StandardPair[] = [];
    let firstPair: StandardPair = standardPairs[0];
    standardPairArray.push(firstPair)
    standardPairArray.push(standardPairs[1]);
    // standardPairArray.push(standardPairs[2]);
    standardPairArray.push(standardPairs[3]);
    standardPairArray.push(standardPairs[4]);
    standardPairArray.push(standardPairs[5]);
    standardPairArray.push(standardPairs[6]);
    standardPairArray.push(standardPairs[7]);
    // console.log("FIRST PAIR")
    // console.log(firstPair)
    
    let pools = await dexApi.standardPoolOfPairs(standardPairArray);
    
    // console.log(pools)
    // pools.forEach((pool) => {
    //     // console.log(pool.)
    //     // let vals = pool.entries();
    //     // console.log(vals.next().value["liquidityAmount"])
    //     // console.log(vals.next().value)
    //     pool.forEach((val) => {
    //         // console.log(val)
    //         console.log("TOKENS: ")
    //         console.log(`Token 0: ${val.token0.name} ${val.token0.symbol} - Token 1: ${val.token1.name} ${val.token1.symbol}`)
    //         console.log("POOLS")
    //         console.log(`${val.reserve0.numerator.toString()} - ${val.reserve1.numerator.toString()}`)
    //     })
        
    //     // console.log("test")
        
    // })

    // console.log("bad pair")
    // console.log(standardPairs[2])
    return pools;

    // const standardPools = await firstValueFrom(dexApi.standardPoolOfPairs(standardPairs));
    // console.log(standardPools)
    // console.log("TEST")
    // console.log("TEST")
    // console.log("TEST")

    // let i = 0;
    // console.log(standardPools.length)
    // while (i < standardPools.length) {
    //     // console.log("Token 0 Reserves: " + standardPools[i].reserve0)
    //     console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`)
    //     console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
    // return standardPools;
}

main()