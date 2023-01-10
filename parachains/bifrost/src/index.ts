import { WsProvider, Keyring } from '@polkadot/api';
import { ModuleBApi, BifrostConfig } from '@zenlink-dex/sdk-api';
import { Percent, Token, TokenAmount, TradeType, StablePair, StableSwap, AssetMeta } from '@zenlink-dex/sdk-core';
import { firstValueFrom } from 'rxjs';
import axios from 'axios';

async function main() {
    // const provider = new WsProvider(BifrostConfig.wss[0]);
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item: AssetMeta) => {
        let token = new Token(item);
        console.log(token)
        return token;
    });

    // query the standard pair and pool
    // const standardPairs = await dexApi.standardPairOfTokens(tokens);
    // console.log(standardPairs)
    const standardPairs = await firstValueFrom(dexApi.standardPairOfTokens(tokens));
    const standardPools = await firstValueFrom(dexApi.standardPoolOfPairs(standardPairs));

    let i = 0;
    console.log(standardPools.length)
    while (i < standardPools.length) {
        console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`)
        console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
        i++;
    }
    // console.log(standardPairs.length)
    // while (i < standardPairs.length) {
    //     console.log(standardPairs[i])
    //     // console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
}

export async function getTokenList() {


    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item: AssetMeta) => {
        let token = new Token(item);
        console.log(token)
        return token;
    });

    return tokens;
}

async function getLiqPools() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;
    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item: AssetMeta) => {
        let token = new Token(item);
        console.log(token)
        return token;
    });

    // query the standard pair and pool
    const standardPairs = await firstValueFrom(dexApi.standardPairOfTokens(tokens));
    const standardPools = await firstValueFrom(dexApi.standardPoolOfPairs(standardPairs));

    // let i = 0;
    // console.log(standardPools.length)
    // while (i < standardPools.length) {
    //     // console.log("Token 0 Reserves: " + standardPools[i].reserve0)
    //     console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`)
    //     console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
    //     i++;
    // }
    return standardPools;
}

main()