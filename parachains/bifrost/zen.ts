import { WsProvider, Keyring } from '@polkadot/api';
import { ModuleBApi, BifrostConfig } from '@zenlink-dex/sdk-api';
import { Percent, Token, TokenAmount, TradeType, StablePair, StableSwap, AssetMeta, StandardPair } from '@zenlink-dex/sdk-core';
import { firstValueFrom } from 'rxjs';
import axios from 'axios';
import { writeFile, readdir, mkdir } from 'node:fs/promises';
import { ApiToken, ChainId, ChainName, Native, NetworkId, SdkToken } from "./types";



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
        return new Token(item);
    });

    return tokens;
}

export async function testApi() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
    );

    await dexApi.initApi(); // init the api;
    // provider
    // dexApi.provider.disconnect();
    dexApi.api?.disconnect()
}

export async function getLiqPools() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
    );
    // dexApi.
    
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
    // standardPairs.
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
    
    return pools;
    
}

async function main() {
    getLiqPools()
    // testApi()
}

// main()