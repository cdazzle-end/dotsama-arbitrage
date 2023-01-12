import { bool } from '@polkadot/types-codec';
import * as fs from 'fs'

let baseUrl = "https://api.kucoin.com";
let endpoint = "/api/v2/symbols";
const orderBook = "/api/v1/market/orderbook/level2_20";
const currency = "/api/v2/currencies/"
const ksmUsdt = "?symbol=KSM-USDT";
const test = "/api/v1/market/orderbook/level2_20?symbol=BTC-USDT"
//These are the assets that we will query info for
const tickers = ["KSM", "RMRK", "MOVR", "KAR", "BNC"];
// const locations = [
//     here: true, xtype: "X0", properties: None
//     here: false, xtype: "X3", properties: Some(["GeneralIndex:8", "PalletInstance:50", "Parachain:1000"])
//     here: false, xtype: "X2", properties: Some(["PalletInstance:10", "Parachain:2023"])
//     here: false, xtype: "X2", properties: Some(["GeneralKey:0x0080", "Parachain:2000"])
//     AssetLocation { here: false, xtype: "X2", properties: Some(["GeneralKey:0x0001", "Parachain:2001"])
// ]
// let ksmLocation = {
//     here: bool = true,
//     xtype: string = "X0",
//     properties: string = "None"
// }

type AssetLocation = {
    ticker: string,
    here: boolean,
    xtype: string,
    properties: string[]
}

type KucoinAsset = {
    assetTicker: string,
    name: string,
    chain: string,
    precision: number,
    contractAddress: string,
    price: number[], // [bid, ask]
    price_decimals: number[],
    assetLocation?: AssetLocation,
}

const ksmLocation: AssetLocation = {
    ticker: "KSM",
    here: true,
    xtype: "X0",
    properties: ["None"]
}
const rmrkLocation: AssetLocation = {
    ticker: "RMRK",
    here: false,
    xtype: "X3",
    properties: ["GeneralIndex:8", "PalletInstance:50", "Parachain:1000"]
}
const movrLocation: AssetLocation = {
    ticker: "MOVR",
    here: false,
    xtype: "X2",
    properties: ["PalletInstance:10", "Parachain:2023"]
}
const karLocation: AssetLocation = {
    ticker: "KAR",
    here: false,
    xtype: "X2",
    properties: ["GeneralKey:0x0080", "Parachain:2000"]
}
const bncLocation: AssetLocation = {
    ticker: "BNC",
    here: false,
    xtype: "X2",
    properties: ["GeneralKey:0x0001", "Parachain:2001"]
}

const assetLocations = [ksmLocation, rmrkLocation, movrLocation, karLocation, bncLocation];

async function kucoinRequest() {
    const response = await fetch(baseUrl + orderBook + ksmUsdt);
    let answer = await response.json();
    console.log(answer.data)
    let bids = answer.data.bids;
    let asks = answer.data.asks;
    // console.log("BIDS")
    let bidPrice = await getSufficientSize(50.0, bids);
    console.log(`Bid price: ${bidPrice}`)
    // console.log("ASKS")
    let askPrice = await getSufficientSize(50.0, asks);
    console.log(`Ask price: ${askPrice}`)
}

async function getAssetLocation(ticker: string): Promise<AssetLocation | null>{
    for (let index in assetLocations) {
        if (assetLocations[index].ticker == ticker) {
            return assetLocations[index];
        }
    }
    return null;
}

async function matchTickersToLocations(): Promise<KucoinAsset[]> {
    let prices = await getUsdtPricesForTickers();
    let kcAssets: KucoinAsset[] = [];

    for (const token of prices) {
        // console.log(token[0])
        let tokenData = await currencyInfo(token[0]);
        // console.log(tokenData)
        let assetLocation = await getAssetLocation(token[0]);
        let bid_price_decimals = getDecimalPlaces(token[1]);
        let bid_price_format = Math.round(moveDecimalRight(token[1], bid_price_decimals));

        let ask_price_decimals = getDecimalPlaces(token[2]);
        let ask_price_format = Math.round(moveDecimalRight(token[2], ask_price_decimals))
        if (assetLocation != null) {
            let kcAsset: KucoinAsset = {
                assetTicker: token[0],
                name: tokenData.fullName,
                chain: tokenData.chains[0].chainName,
                precision: tokenData.precision,
                contractAddress: tokenData.chains[0].contractAddress,
                price: [bid_price_format, ask_price_format],
                price_decimals: [bid_price_decimals, ask_price_decimals],
                assetLocation: assetLocation,
            }
            console.log(kcAsset)
            kcAssets.push(kcAsset);
        }
    }
    let usdtData = await currencyInfo("USDT");
    // console.log(usdtData);
    let usdtAsset: KucoinAsset = {
        assetTicker: "USDT",
        name: usdtData.fullName,
        chain: "None",
        precision: usdtData.precision,
        contractAddress: "None",
        price: [0, 0],
        price_decimals: [0, 0],
        // assetLocation: "None",

    }
    // console.log(usdtAsset)
    kcAssets.push(usdtAsset);
    return kcAssets
}

function moveDecimalRight(num: number, places: number) {
    let number = num * Math.pow(10, places);
    return number
}

function getDecimalPlaces(num: number) {
    let decimalPlaces = 0;
    if (num % 1 !== 0) {
        let numStr = num.toString();
        let decimalIdx = numStr.indexOf(".");
        decimalPlaces = numStr.length - decimalIdx - 1;
    }
    return decimalPlaces;
}

async function getUsdtPricesForTickers(): Promise<[string, number, number][]> {
    let prices: [string, number, number][] = []; //[Bid, Ask]

    for (const ticker of tickers) {
        let requestParameter = "?symbol=" + ticker + "-USDT";
        let uri = baseUrl + orderBook + requestParameter;
        const response = await fetch(uri);
        let answer = await response.json();
        console.log("Asset: " + ticker);
        let bidPrice = await getSufficientSize(50.0, answer.data.bids);
        let askPrice = await getSufficientSize(50.0, answer.data.asks);
        // console.log(`Bid price: ${bidPrice} Ask price: ${askPrice}`)
        prices.push([ticker, bidPrice, askPrice]);
    }
    console.log("All prices")
    console.log(prices)
    return prices;
}

//Get best price of orders atleast 50$ in value
async function getSufficientSize(sufficientValue: number, orders: string[]): Promise<number> {
    let sufficientPrice;
    let totalValue = 0;
    for (let i = 0; i < orders.length; i++){
        let price = parseFloat(orders[i][0]);
        let amount = parseFloat(orders[i][1]);

        let orderValue = price * amount;
        // console.log(`Price: ${price} Amount: ${amount} Magnitude: ${orderValue} Total Value: ${totalValue}`)
        totalValue += orderValue;
        if (totalValue > sufficientValue) {
            return sufficientPrice = price;
            
        }
    }
    console.log("Failing to find sufficient size")
    //Return the last value from the orders
    return parseFloat(orders[orders.length][0]);
    
}

async function currencyInfo(ticker: string) {
    const response = await fetch(baseUrl + currency + ticker);
    let answer = await response.json();
    // console.log(answer)
    // console.log(answer.data)
    return answer.data
}

export async function saveKucoinAssets() {
    let kcAssets = await matchTickersToLocations();
    fs.writeFileSync('../kucoin/exchange_data', JSON.stringify(kcAssets), 'utf8');
}
async function main() {
    saveKucoinAssets()
    console.log("kucoin main")
}

main().then(() => console.log("complete"))