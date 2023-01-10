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
exports.getTokenList = void 0;
const api_1 = require("@polkadot/api");
const sdk_api_1 = require("@zenlink-dex/sdk-api");
const sdk_core_1 = require("@zenlink-dex/sdk-core");
const rxjs_1 = require("rxjs");
const axios_1 = __importDefault(require("axios"));
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        // const provider = new WsProvider(BifrostConfig.wss[0]);
        const provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
        yield provider.isReady;
        const dexApi = new sdk_api_1.ModuleBApi(provider);
        yield dexApi.initApi(); // init the api;
        const response = yield axios_1.default.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
        const tokensMeta = response.data.tokens;
        // generate Tokens
        const tokens = tokensMeta.map((item) => {
            let token = new sdk_core_1.Token(item);
            console.log(token);
            return token;
        });
        // query the standard pair and pool
        // const standardPairs = await dexApi.standardPairOfTokens(tokens);
        // console.log(standardPairs)
        const standardPairs = yield (0, rxjs_1.firstValueFrom)(dexApi.standardPairOfTokens(tokens));
        const standardPools = yield (0, rxjs_1.firstValueFrom)(dexApi.standardPoolOfPairs(standardPairs));
        let i = 0;
        console.log(standardPools.length);
        while (i < standardPools.length) {
            console.log(`Token 0: ${standardPools[i].token0.name} ${standardPools[i].token0.symbol} - Token 1: ${standardPools[i].token1.name} ${standardPools[i].token1.symbol}`);
            console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`);
            i++;
        }
        // console.log(standardPairs.length)
        // while (i < standardPairs.length) {
        //     console.log(standardPairs[i])
        //     // console.log(`${standardPools[i].reserve1.numerator.toString()} - ${standardPools[i].reserve1.numerator.toString()}`)
        //     i++;
        // }
    });
}
function getTokenList() {
    return __awaiter(this, void 0, void 0, function* () {
        const provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
        yield provider.isReady;
        const dexApi = new sdk_api_1.ModuleBApi(provider);
        yield dexApi.initApi(); // init the api;
        const response = yield axios_1.default.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
        const tokensMeta = response.data.tokens;
        // generate Tokens
        const tokens = tokensMeta.map((item) => {
            let token = new sdk_core_1.Token(item);
            console.log(token);
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
        const response = yield axios_1.default.get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost.json');
        const tokensMeta = response.data.tokens;
        // generate Tokens
        const tokens = tokensMeta.map((item) => {
            let token = new sdk_core_1.Token(item);
            console.log(token);
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
main();
