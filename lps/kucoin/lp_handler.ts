import { bool } from '@polkadot/types-codec';
import * as fs from 'fs'
import { MyAssetRegistryObject, CexAsset } from '../../assets/asset_types'
import { CexLp } from '../lp_types'
declare const fetch: any;

let baseUrl = "https://api.kucoin.com";
let endpoint = "/api/v2/symbols";
const orderBook = "/api/v1/market/orderbook/level2_20";
const currency = "/api/v2/currencies/"
const ksmUsdt = "?symbol=KSM-USDT";
const test = "/api/v1/market/orderbook/level2_20?symbol=BTC-USDT"
//These are the assets that we will query info for
const tickers = ["KSM", "RMRK", "MOVR", "KAR", "BNC"];
const exchange = "kucoin";

export async function updateLps() {
    const assets = JSON.parse(fs.readFileSync('../assets/kucoin/asset_registry.json', 'utf8'));
    // console.log(assets)
    let lps = await Promise.all(assets.map(async (asset: MyAssetRegistryObject) => {
        let tokenData = asset.tokenData as CexAsset
        if (tokenData.assetTicker != "USDT") {
            let requestParameter = "?symbol=" + tokenData.assetTicker + "-USDT";
            let uri = baseUrl + orderBook + requestParameter;
            const response = await fetch(uri);
            let answer = await response.json();
            let bidPrice = await getSufficientSize(50.0, answer.data.bids);
            let bid_price_decimals = getDecimalPlaces(bidPrice);
            let bid_price_format = Math.round(moveDecimalRight(bidPrice, bid_price_decimals));

            let askPrice = await getSufficientSize(50.0, answer.data.asks);
            let ask_price_decimals = getDecimalPlaces(askPrice);
            let ask_price_format = Math.round(moveDecimalRight(askPrice, ask_price_decimals))
            let cexLp: CexLp = {
                exchange: exchange,
                assetTicker: tokenData.assetTicker,
                price: [bid_price_format, ask_price_format],
                priceDecimals: [bid_price_decimals, ask_price_decimals],
            }
            // console.log(cexLp)
            return cexLp;

        }
    }))
    lps = lps.filter((lp: CexLp) => lp != undefined)
    // console.log(lps)
    fs.writeFileSync('./kucoin/lps.json', JSON.stringify(lps, null, 2))
}

async function saveLps() {
    const assets = JSON.parse(fs.readFileSync('../../assets/kucoin/asset_registry.json', 'utf8'));
    // console.log(assets)
    let lps = await Promise.all(assets.map(async (asset: MyAssetRegistryObject) => {
        let tokenData = asset.tokenData as CexAsset
        if (tokenData.assetTicker != "USDT") {
            let requestParameter = "?symbol=" + tokenData.assetTicker + "-USDT";
            let uri = baseUrl + orderBook + requestParameter;
            const response = await fetch(uri);
            let answer = await response.json();
            let bidPrice = await getSufficientSize(50.0, answer.data.bids);
            let bid_price_decimals = getDecimalPlaces(bidPrice);
            let bid_price_format = Math.round(moveDecimalRight(bidPrice, bid_price_decimals));

            let askPrice = await getSufficientSize(50.0, answer.data.asks);
            let ask_price_decimals = getDecimalPlaces(askPrice);
            let ask_price_format = Math.round(moveDecimalRight(askPrice, ask_price_decimals))
            let cexLp: CexLp = {
                exchange: exchange,
                assetTicker: tokenData.assetTicker,
                price: [bid_price_format, ask_price_format],
                priceDecimals: [bid_price_decimals, ask_price_decimals],
            }
            // console.log(cexLp)
            return cexLp;
            
        }
    }))
    
    lps = lps.filter((lp: CexLp) => lp != undefined)
    console.log(lps)
    fs.writeFileSync('./lps.json', JSON.stringify(lps, null, 2))
}

//Get best price of orders atleast 50$ in value
async function getSufficientSize(sufficientValue: number, orders: string[]): Promise<number> {
    let sufficientPrice;
    let totalValue = 0;
    for (let i = 0; i < orders.length; i++) {
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
// async function getUsdtPricesForTickers(): Promise<[string, number, number][]> {
//     let prices: [string, number, number][] = []; //[Bid, Ask]

//     for (const ticker of tickers) {
//         let requestParameter = "?symbol=" + ticker + "-USDT";
//         let uri = baseUrl + orderBook + requestParameter;
//         const response = await fetch(uri);
//         let answer = await response.json();
//         let bidPrice = await getSufficientSize(50.0, answer.data.bids);
//         let askPrice = await getSufficientSize(50.0, answer.data.asks);
//         prices.push([ticker, bidPrice, askPrice]);
//     }
//     return prices;
// }

async function main() {
    await saveLps()
}

// main()