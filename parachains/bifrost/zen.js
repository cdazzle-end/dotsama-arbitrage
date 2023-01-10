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
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
exports.getLiqPools = exports.getTokenList = void 0;
var api_1 = require("@polkadot/api");
var sdk_api_1 = require("@zenlink-dex/sdk-api");
var sdk_core_1 = require("@zenlink-dex/sdk-core");
var rxjs_1 = require("rxjs");
var axios_1 = require("axios");
var types_1 = require("./types");
function main() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
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
            getLiqPools();
            return [2 /*return*/];
        });
    });
}
function getResponse() {
    return __awaiter(this, void 0, void 0, function () {
        var CHAINID_TO_NETWORKID, CHAINID_TO_CHAINNAME, API_URL, response, allTokens, sdkTokens;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    CHAINID_TO_NETWORKID = function (id) {
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
                                throw new Error("Unknown chain id: ".concat(id));
                        }
                    };
                    CHAINID_TO_CHAINNAME = function (id) {
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
                                throw new Error("Unknown chain id: ".concat(id));
                        }
                    };
                    API_URL = 'https://api.zenlink.pro/rpc';
                    return [4 /*yield*/, axios_1["default"].post(API_URL, {
                            "jsonrpc": "2.0",
                            "id": 1,
                            "method": "asset.get",
                            "params": []
                            // "method": "zenlinkProtocol_getAllAssets",
                            // "params": []
                        })];
                case 1:
                    response = _a.sent();
                    allTokens = response.data.result || [];
                    sdkTokens = allTokens
                        .filter(function (token) { return token.chainId === 2001; })
                        .filter(function (token) { return token.isNative === types_1.Native.P || token.isNative === types_1.Native.N; });
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
                    return [2 /*return*/];
            }
        });
    });
}
function getTokenList() {
    return __awaiter(this, void 0, void 0, function () {
        var provider, dexApi, response, tokensMeta, tokens;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
                    return [4 /*yield*/, provider.isReady];
                case 1:
                    _a.sent();
                    dexApi = new sdk_api_1.ModuleBApi(provider);
                    return [4 /*yield*/, dexApi.initApi()];
                case 2:
                    _a.sent(); // init the api;
                    return [4 /*yield*/, axios_1["default"].get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json')];
                case 3:
                    response = _a.sent();
                    tokensMeta = response.data.tokens;
                    tokens = tokensMeta.map(function (item) {
                        var token = new sdk_core_1.Token(item);
                        // console.log(token)
                        return token;
                    });
                    return [2 /*return*/, tokens];
            }
        });
    });
}
exports.getTokenList = getTokenList;
function getLiqPools() {
    return __awaiter(this, void 0, void 0, function () {
        var provider, dexApi, response, tokensMeta, tokens, standardPairs, standardPairArray, firstPair, pools;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    provider = new api_1.WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws');
                    return [4 /*yield*/, provider.isReady];
                case 1:
                    _a.sent();
                    dexApi = new sdk_api_1.ModuleBApi(provider);
                    return [4 /*yield*/, dexApi.initApi()];
                case 2:
                    _a.sent(); // init the api;
                    return [4 /*yield*/, axios_1["default"].get('https://raw.githubusercontent.com/zenlinkpro/token-list/main/tokens/bifrost-kusama.json')];
                case 3:
                    response = _a.sent();
                    tokensMeta = response.data.tokens;
                    tokens = tokensMeta.map(function (item) {
                        var token = new sdk_core_1.Token(item);
                        // console.log(token)
                        // console.log(token)
                        return token;
                    });
                    return [4 /*yield*/, (0, rxjs_1.firstValueFrom)(dexApi.standardPairOfTokens(tokens))];
                case 4:
                    standardPairs = _a.sent();
                    standardPairArray = [];
                    firstPair = standardPairs[0];
                    standardPairArray.push(firstPair);
                    standardPairArray.push(standardPairs[1]);
                    // standardPairArray.push(standardPairs[2]);
                    standardPairArray.push(standardPairs[3]);
                    standardPairArray.push(standardPairs[4]);
                    standardPairArray.push(standardPairs[5]);
                    standardPairArray.push(standardPairs[6]);
                    standardPairArray.push(standardPairs[7]);
                    return [4 /*yield*/, dexApi.standardPoolOfPairs(standardPairArray)];
                case 5:
                    pools = _a.sent();
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
                    return [2 /*return*/, pools];
            }
        });
    });
}
exports.getLiqPools = getLiqPools;
main();
