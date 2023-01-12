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
exports.saveKucoinAssets = void 0;
var fs = require("fs");
var baseUrl = "https://api.kucoin.com";
var endpoint = "/api/v2/symbols";
var orderBook = "/api/v1/market/orderbook/level2_20";
var currency = "/api/v2/currencies/";
var ksmUsdt = "?symbol=KSM-USDT";
var test = "/api/v1/market/orderbook/level2_20?symbol=BTC-USDT";
//These are the assets that we will query info for
var tickers = ["KSM", "RMRK", "MOVR", "KAR", "BNC"];
var ksmLocation = {
    ticker: "KSM",
    here: true,
    xtype: "X0",
    properties: ["None"]
};
var rmrkLocation = {
    ticker: "RMRK",
    here: false,
    xtype: "X3",
    properties: ["GeneralIndex:8", "PalletInstance:50", "Parachain:1000"]
};
var movrLocation = {
    ticker: "MOVR",
    here: false,
    xtype: "X2",
    properties: ["PalletInstance:10", "Parachain:2023"]
};
var karLocation = {
    ticker: "KAR",
    here: false,
    xtype: "X2",
    properties: ["GeneralKey:0x0080", "Parachain:2000"]
};
var bncLocation = {
    ticker: "BNC",
    here: false,
    xtype: "X2",
    properties: ["GeneralKey:0x0001", "Parachain:2001"]
};
var assetLocations = [ksmLocation, rmrkLocation, movrLocation, karLocation, bncLocation];
function kucoinRequest() {
    return __awaiter(this, void 0, void 0, function () {
        var response, answer, bids, asks, bidPrice, askPrice;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, fetch(baseUrl + orderBook + ksmUsdt)];
                case 1:
                    response = _a.sent();
                    return [4 /*yield*/, response.json()];
                case 2:
                    answer = _a.sent();
                    console.log(answer.data);
                    bids = answer.data.bids;
                    asks = answer.data.asks;
                    return [4 /*yield*/, getSufficientSize(50.0, bids)];
                case 3:
                    bidPrice = _a.sent();
                    console.log("Bid price: ".concat(bidPrice));
                    return [4 /*yield*/, getSufficientSize(50.0, asks)];
                case 4:
                    askPrice = _a.sent();
                    console.log("Ask price: ".concat(askPrice));
                    return [2 /*return*/];
            }
        });
    });
}
function getAssetLocation(ticker) {
    return __awaiter(this, void 0, void 0, function () {
        var index;
        return __generator(this, function (_a) {
            for (index in assetLocations) {
                if (assetLocations[index].ticker == ticker) {
                    return [2 /*return*/, assetLocations[index]];
                }
            }
            return [2 /*return*/, null];
        });
    });
}
function matchTickersToLocations() {
    return __awaiter(this, void 0, void 0, function () {
        var prices, kcAssets, _i, prices_1, token, tokenData, assetLocation, bid_price_decimals, bid_price_format, ask_price_decimals, ask_price_format, kcAsset, usdtData, usdtAsset;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, getUsdtPricesForTickers()];
                case 1:
                    prices = _a.sent();
                    kcAssets = [];
                    _i = 0, prices_1 = prices;
                    _a.label = 2;
                case 2:
                    if (!(_i < prices_1.length)) return [3 /*break*/, 6];
                    token = prices_1[_i];
                    return [4 /*yield*/, currencyInfo(token[0])];
                case 3:
                    tokenData = _a.sent();
                    return [4 /*yield*/, getAssetLocation(token[0])];
                case 4:
                    assetLocation = _a.sent();
                    bid_price_decimals = getDecimalPlaces(token[1]);
                    bid_price_format = Math.round(moveDecimalRight(token[1], bid_price_decimals));
                    ask_price_decimals = getDecimalPlaces(token[2]);
                    ask_price_format = Math.round(moveDecimalRight(token[2], ask_price_decimals));
                    if (assetLocation != null) {
                        kcAsset = {
                            assetTicker: token[0],
                            name: tokenData.fullName,
                            chain: tokenData.chains[0].chainName,
                            precision: tokenData.precision,
                            contractAddress: tokenData.chains[0].contractAddress,
                            price: [bid_price_format, ask_price_format],
                            price_decimals: [bid_price_decimals, ask_price_decimals],
                            assetLocation: assetLocation
                        };
                        console.log(kcAsset);
                        kcAssets.push(kcAsset);
                    }
                    _a.label = 5;
                case 5:
                    _i++;
                    return [3 /*break*/, 2];
                case 6: return [4 /*yield*/, currencyInfo("USDT")];
                case 7:
                    usdtData = _a.sent();
                    usdtAsset = {
                        assetTicker: "USDT",
                        name: usdtData.fullName,
                        chain: "None",
                        precision: usdtData.precision,
                        contractAddress: "None",
                        price: [0, 0],
                        price_decimals: [0, 0]
                    };
                    // console.log(usdtAsset)
                    kcAssets.push(usdtAsset);
                    return [2 /*return*/, kcAssets];
            }
        });
    });
}
function moveDecimalRight(num, places) {
    var number = num * Math.pow(10, places);
    return number;
}
function getDecimalPlaces(num) {
    var decimalPlaces = 0;
    if (num % 1 !== 0) {
        var numStr = num.toString();
        var decimalIdx = numStr.indexOf(".");
        decimalPlaces = numStr.length - decimalIdx - 1;
    }
    return decimalPlaces;
}
function getUsdtPricesForTickers() {
    return __awaiter(this, void 0, void 0, function () {
        var prices, _i, tickers_1, ticker, requestParameter, uri, response, answer, bidPrice, askPrice;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    prices = [];
                    _i = 0, tickers_1 = tickers;
                    _a.label = 1;
                case 1:
                    if (!(_i < tickers_1.length)) return [3 /*break*/, 7];
                    ticker = tickers_1[_i];
                    requestParameter = "?symbol=" + ticker + "-USDT";
                    uri = baseUrl + orderBook + requestParameter;
                    return [4 /*yield*/, fetch(uri)];
                case 2:
                    response = _a.sent();
                    return [4 /*yield*/, response.json()];
                case 3:
                    answer = _a.sent();
                    console.log("Asset: " + ticker);
                    return [4 /*yield*/, getSufficientSize(50.0, answer.data.bids)];
                case 4:
                    bidPrice = _a.sent();
                    return [4 /*yield*/, getSufficientSize(50.0, answer.data.asks)];
                case 5:
                    askPrice = _a.sent();
                    // console.log(`Bid price: ${bidPrice} Ask price: ${askPrice}`)
                    prices.push([ticker, bidPrice, askPrice]);
                    _a.label = 6;
                case 6:
                    _i++;
                    return [3 /*break*/, 1];
                case 7:
                    console.log("All prices");
                    console.log(prices);
                    return [2 /*return*/, prices];
            }
        });
    });
}
//Get best price of orders atleast 50$ in value
function getSufficientSize(sufficientValue, orders) {
    return __awaiter(this, void 0, void 0, function () {
        var sufficientPrice, totalValue, i, price, amount, orderValue;
        return __generator(this, function (_a) {
            totalValue = 0;
            for (i = 0; i < orders.length; i++) {
                price = parseFloat(orders[i][0]);
                amount = parseFloat(orders[i][1]);
                orderValue = price * amount;
                // console.log(`Price: ${price} Amount: ${amount} Magnitude: ${orderValue} Total Value: ${totalValue}`)
                totalValue += orderValue;
                if (totalValue > sufficientValue) {
                    return [2 /*return*/, sufficientPrice = price];
                }
            }
            console.log("Failing to find sufficient size");
            //Return the last value from the orders
            return [2 /*return*/, parseFloat(orders[orders.length][0])];
        });
    });
}
function currencyInfo(ticker) {
    return __awaiter(this, void 0, void 0, function () {
        var response, answer;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, fetch(baseUrl + currency + ticker)];
                case 1:
                    response = _a.sent();
                    return [4 /*yield*/, response.json()];
                case 2:
                    answer = _a.sent();
                    // console.log(answer)
                    // console.log(answer.data)
                    return [2 /*return*/, answer.data];
            }
        });
    });
}
function saveKucoinAssets() {
    return __awaiter(this, void 0, void 0, function () {
        var kcAssets;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0: return [4 /*yield*/, matchTickersToLocations()];
                case 1:
                    kcAssets = _a.sent();
                    fs.writeFileSync('../kucoin/exchange_data', JSON.stringify(kcAssets), 'utf8');
                    return [2 /*return*/];
            }
        });
    });
}
exports.saveKucoinAssets = saveKucoinAssets;
function main() {
    return __awaiter(this, void 0, void 0, function () {
        return __generator(this, function (_a) {
            saveKucoinAssets();
            console.log("kucoin main");
            return [2 /*return*/];
        });
    });
}
main().then(function () { return console.log("complete"); });
