// const { WsProvider, Keyring } = require('@polkadot/api');
// const { ModuleBApi, BifrostConfig } = require('@zenlink-dex/sdk-api');
// const { Percent, Token, TokenAmount, TradeType, StablePair, StableSwap } = require('@zenlink-dex/sdk-core');
// const { firstValueFrom } = require('rxjs');
const axios = require('axios').default;
import { WsProvider, Keyring } from '@polkadot/api';
import { ModuleBApi, BifrostConfig } from '@zenlink-dex/sdk-api';
import { Percent, Token, TokenAmount, TradeType, StablePair, StableSwap } from '@zenlink-dex/sdk-core';
import { firstValueFrom } from 'rxjs';

exports.getZenlinkAssets = async () => {
    const provider = new WsProvider(BifrostConfig.wss[0]);
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        // BifrostConfig
    );
    await dexApi.initApi(); // init the api;

    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item) => {
        const token = new Token(item);
        console.log(token)
        return token;
    });
    return tokens;
}

async function main() {
    const provider = new WsProvider(BifrostConfig.wss[0]);
    await provider.isReady;
    const dexApi = new ModuleBApi(
        provider,
        BifrostConfig
    );
    await dexApi.initApi(); // init the api;

    const response = await axios.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
    const tokensMeta = response.data.tokens;

    // generate Tokens
    const tokens = tokensMeta.map((item) => {
        const token = new Token(item);
        console.log(token)
        return token;
    });
    // StablePair
    

    // query the standard pair and pool
    const standardPairs = await firstValueFrom(dexApi.standardPairOfTokens(tokens));
    const standardPools = await firstValueFrom(dexApi.standardPoolOfPairs(standardPairs));

    // query the stable pair and pool
    // query the stable pair;
    // const stablePairs
    const stablePairs: StablePair[] = await firstValueFrom(dexApi.stablePairOf());
    console.log('stablePairs', stablePairs);

    // query the stable pool of stable pair;
    const stablePools = await firstValueFrom(dexApi.stablePoolOfPairs());
    console.log('stablePairs', stablePools);
    // console.log(standardPairs)
    // console.log(standardPools)
    // console.log("test")


    // // query the stable pair and pool
    // // query the stable pair;
    // const stablePairs: StablePair[] = await firstValueFrom(dexApi.stablePairOf());
    // console.log('stablePairs', stablePairs);

    // // query the stable pool of stable pair;
    // const stablePools: StableSwap[] = await firstValueFrom(dexApi.stablePoolOfPairs());
    // console.log('stablePairs', stablePools);
}

main()