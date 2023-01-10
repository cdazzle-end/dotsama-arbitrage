"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.getLiqPools = exports.getTokenList = void 0;
const api_1 = require("@polkadot/api");
const sdk_api_1 = require("@zenlink-dex/sdk-api");
const sdk_core_1 = require("@zenlink-dex/sdk-core");
const rxjs_1 = require("rxjs");
const axios_1 = __importDefault(require("axios"));
const types_1 = require("./types");
function main() {
    return __awaiter(this, void 0, void 0, function* () {
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
        getResponse();
    });
}
function getResponse() {
    return __awaiter(this, void 0, void 0, function* () {
        const CHAINID_TO_NETWORKID = (id) => {
            switch (id) {
                case types_1.ChainId.BIFROST:
                    return types_1.NetworkId.KUSAMA;
                case types_1.ChainId.MOONRIVER:
                    return types_1.NetworkId.KUSAMA;
                case types_1.ChainId.MOONBEAM:
                    return types_1.NetworkId.POLKADOT;
                case types_1.ChainId.ASTAR:
                    return types_1.NetworkId.POLKADOT;
                default:
                    throw new Error(`Unknown chain id: ${id}`);
            }
        };
        const CHAINID_TO_CHAINNAME = (id) => {
            switch (id) {
                case types_1.ChainId.BIFROST:
                    return types_1.ChainName.BIFROST;
                case types_1.ChainId.MOONBEAM:
                    return types_1.ChainName.MOONBEAM;
                case types_1.ChainId.ASTAR:
                    return types_1.ChainName.ASTAR;
                case types_1.ChainId.MOONRIVER:
                    return types_1.ChainName.MOONRIVER;
                default:
                    throw new Error(`Unknown chain id: ${id}`);
            }
        };
        const API_URL = 'https://api.zenlink.pro/rpc';
        const response = yield axios_1.default.post(API_URL, {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "asset.get",
            "params": []
            // "method": "zenlinkProtocol_getAllAssets",
            // "params": []
        });
        const allTokens = response.data.result || [];
        const sdkTokens = allTokens
            .filter((token) => token.chainId === 2001)
            .filter((token) => token.isNative === types_1.Native.P || token.isNative === types_1.Native.N);
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
        console.log(response);
    });
}
function getTokenList() {
    return __awaiter(this, void 0, void 0, function* () {
        const provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
        yield provider.isReady;
        const dexApi = new sdk_api_1.ModuleBApi(provider);
        yield dexApi.initApi(); // init the api;
        const response = yield axios_1.default.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json');
        const tokensMeta = response.data.tokens;
        // generate Tokens
        const tokens = tokensMeta.map((item) => {
            let token = new sdk_core_1.Token(item);
            // console.log(token)
            return token;
        });
        return tokens;
    });
}
exports.getTokenList = getTokenList;
function getLiqPools() {
    return __awaiter(this, void 0, void 0, function* () {
        const provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
        yield provider.isReady;
        const dexApi = new sdk_api_1.ModuleBApi(provider);
        yield dexApi.initApi(); // init the api;
        const response = yield axios_1.default.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json');
        const tokensMeta = response.data.tokens;
        // generate Tokens
        const tokens = tokensMeta.map((item) => {
            let token = new sdk_core_1.Token(item);
            // console.log(token)
            return token;
        });
        // query the standard pair and pool
        const standardPairs = yield (0, rxjs_1.firstValueFrom)(dexApi.standardPairOfTokens(tokens));
        const standardPools = yield (0, rxjs_1.firstValueFrom)(dexApi.standardPoolOfPairs(standardPairs));
        // let i = 0;
        // console.log(standardPools.length)
        // while (i < standardPools.length) {
        //     // console.log("Token 0 Reserves: " + standardPools[i].reserve0)
        //     console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`)
        //     console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
        //     i++;
        // }
        return standardPools;
    });
}
exports.getLiqPools = getLiqPools;
main();
