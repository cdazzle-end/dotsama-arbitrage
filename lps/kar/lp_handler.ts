import * as fs from 'fs';
import {MyLp, StableSwapPool} from '../lp_types'
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { options } = require('@acala-network/api');
import { BigNumber } from 'bignumber.js';
const endpoint1 = 'wss://karura.api.onfinality.io/public-ws';
const endpoint2 = 'wss://karura-rpc-2.aca-api.network/ws';
const endpoint6 = 'wss://karura-rpc.dwellir.com'
const endpoint3 = 'wss://karura-rpc-0.aca-api.network'
const endpoint4 = 'wss://karura-rpc-1.aca-api.network'
const endpoint5 = 'wss://karura-rpc-2.aca-api.network/ws'
// wss://karura-rpc-3.aca-api.network/ws
TESTTTTT
declare const fetch: any;

export async function updateLps() {
    const provider = new WsProvider(endpoint3);
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    let stables = updateStables(api);
    const parachainId = await api.query.parachainInfo?.parachainId();
    const assetRegistry = JSON.parse(fs.readFileSync('../assets/kar/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    });
    const lpEntries = await api.query.dex.liquidityPool.entries();
    const lps = lpEntries.map((lp: any) => {
        const lpAssetIds = lp[0].toHuman()[0];
        let liquidity = lp[1].toHuman();
        const tokens = lpAssetIds.map((lpAssetId: any) => {
            const matchedAsset = assetRegistry.find((asset: any) => {
                return Object.keys(lpAssetId)[0] === "ForeignAsset"
                    && Object.keys(asset.localId)[0] === "ForeignAssetId"
                    && Object.values(lpAssetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(lpAssetId)[0] === "Token"
                    && Object.keys(asset.localId)[0] === "NativeAssetId"
                    && Object.values(lpAssetId)[0] === (Object.values(asset.localId)[0] as any)["Token"]
                    || Object.keys(lpAssetId)[0] === "StableAssetPoolToken"
                    && Object.keys(asset.localId)[0] === "StableAssetId"
                    && Object.values(lpAssetId)[0] === Object.values(asset.localId)[0]
            })
            return matchedAsset.localId
        })
        liquidity = liquidity.map((l: any) => {
            return l.toString().replace(/,/g, "")
        })
        const newLp: MyLp = {
            chainId: parachainId.toJSON() as number,
            poolAssets: tokens,
            liquidityStats: liquidity
        }
        return newLp
    });
    
    // let stablePools = await queryStableLps(api);

    // fs.writeFileSync('./kar/stablePools.json', JSON.stringify(stablePools, null, 2))
    fs.writeFileSync('./kar/lps.json', JSON.stringify(lps, null, 2))
    await stables.then(() => console.log("kar stables complete"));
    api.disconnect()
}

async function saveLps() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const parachainId = await api.query.parachainInfo?.parachainId();
    const assetRegistry = JSON.parse(fs.readFileSync('../../assets/kar/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    });
    const lpEntries = await api.query.dex.liquidityPool.entries();
    const lps = lpEntries.map( (lp: any) => {
        const lpAssetIds = lp[0].toHuman()[0];
        let liquidity = lp[1].toHuman();
        const tokens = lpAssetIds.map((lpAssetId: any) => {
            const matchedAsset = assetRegistry.find((asset: any) => {
                return Object.keys(lpAssetId)[0] === "ForeignAsset"
                    && Object.keys(asset.localId)[0] === "ForeignAssetId"
                    && Object.values(lpAssetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(lpAssetId)[0] === "Token"
                    && Object.keys(asset.localId)[0] === "NativeAssetId"
                    && Object.values(lpAssetId)[0] === (Object.values(asset.localId)[0] as any)["Token"]
                    || Object.keys(lpAssetId)[0] === "StableAssetPoolToken"
                    && Object.keys(asset.localId)[0] === "StableAssetId"
                    && Object.values(lpAssetId)[0] === Object.values(asset.localId)[0]
            })
            // console.log(matchedAsset.localId)
            return matchedAsset.localId
        })
        liquidity = liquidity.map((l: any) => {
            return l.toString().replace(/,/g, "")
        })
        const newLp: MyLp = {
            chainId: parachainId.toJSON() as number,
            poolAssets: tokens,
            liquidityStats: liquidity
        }
        return newLp
    });

    fs.writeFileSync('../kar/lps.json', JSON.stringify(lps, null, 2))
}

async function updateStables(api: any) {
    // const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const api = new ApiPromise(options({ provider }));
    // await api.isReady;

    const parachainId = await api.query.parachainInfo?.parachainId();
    const assetRegistry = JSON.parse(fs.readFileSync('../assets/kar/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    });
    const lpEntries = await api.query.stableAsset.pools.entries();
    let pools = await Promise.all(lpEntries.map(async ([key, value]: any) => {
        let valueData = value.toHuman() as any;
        let assets = valueData.assets;
        let liquidity = valueData.balances;

        let matchedAssets = assets.map((assetId: any) => {
            const matchedAsset = assetRegistry.find((asset: any) => {
                return Object.keys(assetId)[0] === "ForeignAsset"
                    && Object.keys(asset.localId)[0] === "ForeignAssetId"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(assetId)[0] === "Token"
                    && Object.keys(asset.localId)[0] === "NativeAssetId"
                    && Object.values(assetId)[0] === (Object.values(asset.localId)[0] as any)["Token"]
                    || Object.keys(assetId)[0] === "StableAssetPoolToken"
                    && Object.keys(asset.localId)[0] === "StableAssetId"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(assetId)[0] === "Erc20"
                    && Object.keys(asset.localId)[0] === "Erc20"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]

            })
            // console.log(matchedAsset.localId)
            return matchedAsset.localId
        })
        let A;
        let aPrecision = 100
        // Special handling for Ksm/Lksm pool
        if (matchedAssets.length === 2) {
            let ksmLksmLiq = await getKsmLksmBalance(api);
            liquidity.push(ksmLksmLiq[0]);
            liquidity.push(ksmLksmLiq[1]);
            liquidity = liquidity.map((l: any) => {
                return l.toString().replace(/,/g, "")
            })
            A = 0.03 * aPrecision;
        } else {
            liquidity = liquidity.map((l: any) => {
                return l.toString().replace(/,/g, "")
            })
            A = 100 * aPrecision;
        }
        let tokenPrecisions = valueData.precisions.map((p: any) => {
            return p.toString().replace(/,/g, "")
        })
        let newStablePool: StableSwapPool = {
            chainId: parachainId.toJSON() as number,
            poolAssets: matchedAssets,
            liquidityStats: liquidity,
            tokenPrecisions: tokenPrecisions,
            swapFee: valueData.swapFee.replace(/,/g, ""),
            a: A,
            aPrecision: aPrecision,
            aBlock: valueData.aBlock.replace(/,/g, ""),
            futureA: valueData.futureA.replace(/,/g, ""),
            futureABlock: valueData.futureABlock.replace(/,/g, ""),
            totalSupply: valueData.totalSupply.replace(/,/g, ""),
            poolPrecision: valueData.precision.replace(/,/g, "")
        }
        return newStablePool
    }));

    // console.log(pools)
    fs.writeFileSync('./kar/stablePools.json', JSON.stringify(pools, null, 2))
    // return pools;
}

async function queryStableLps(api: any) {
    // const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const api = new ApiPromise(options({ provider }));
    // await api.isReady;
    
    const parachainId = await api.query.parachainInfo?.parachainId();
    const assetRegistry = JSON.parse(fs.readFileSync('../../assets/kar/asset_registry.json', 'utf8')).map((asset: any) => {
        return asset.tokenData
    });
    const lpEntries = await api.query.stableAsset.pools.entries();
    let pools = await Promise.all(lpEntries.map(async ([key, value]: any) => {
        let valueData = value.toHuman() as any;
        let assets = valueData.assets;
        let liquidity = valueData.balances;

        let matchedAssets = assets.map((assetId: any) => {
            const matchedAsset = assetRegistry.find((asset: any) => {
                return Object.keys(assetId)[0] === "ForeignAsset"
                    && Object.keys(asset.localId)[0] === "ForeignAssetId"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(assetId)[0] === "Token"
                    && Object.keys(asset.localId)[0] === "NativeAssetId"
                    && Object.values(assetId)[0] === (Object.values(asset.localId)[0] as any)["Token"]
                    || Object.keys(assetId)[0] === "StableAssetPoolToken"
                    && Object.keys(asset.localId)[0] === "StableAssetId"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]
                    || Object.keys(assetId)[0] === "Erc20"
                    && Object.keys(asset.localId)[0] === "Erc20"
                    && Object.values(assetId)[0] === Object.values(asset.localId)[0]
                
            })
            // console.log(matchedAsset.localId)
            return matchedAsset.localId
        })
        let A;
        let aPrecision = 100
        // Special handling for Ksm/Lksm pool
        if (matchedAssets.length === 2) {
            let ksmRealLiq = await getKsmLksmBalance(api);
            liquidity.push(ksmRealLiq[0])
            liquidity.push(ksmRealLiq[1])
            liquidity = liquidity.map((l: any) => {
                return l.toString().replace(/,/g, "")
            })
            A = 0.03 * aPrecision;
        } else {
            liquidity = liquidity.map((l: any) => {
                return l.toString().replace(/,/g, "")
            })
            A = 100 * aPrecision;
        }
        let tokenPrecisions = valueData.precisions.map((p: any) => {
            return p.toString().replace(/,/g, "")
        })
        let newStablePool: StableSwapPool = {
            chainId: parachainId.toJSON() as number,
            poolAssets: matchedAssets,
            liquidityStats: liquidity,
            tokenPrecisions: tokenPrecisions,
            swapFee: valueData.swapFee.replace(/,/g, ""),
            a: A,
            aPrecision: aPrecision,
            aBlock: valueData.aBlock.replace(/,/g, ""),
            futureA: valueData.futureA.replace(/,/g, ""),
            futureABlock: valueData.futureABlock.replace(/,/g, ""),
            totalSupply: valueData.totalSupply.replace(/,/g, ""),
            poolPrecision: valueData.precision.replace(/,/g, "")
        }
        return newStablePool
    }));

    // console.log(pools)
    fs.writeFileSync('./stablePools.json', JSON.stringify(pools, null, 2))
    return pools;
}



async function getDexSwapAmount(pool:StableSwapPool, tokenIn: any, tokenOut: any, input: any) {
    let poolBalances = pool.liquidityStats.map((balance: any) => {
        return balance / 10 ** 12
    })
    console.log(poolBalances)
    let increments = input / 100;

    let totalOut = 0;
    for (let i = 0; i < 100; i++) {
        let out = poolBalances[tokenOut] * increments / (poolBalances[tokenIn] + increments);
        let slip = (out / poolBalances[tokenOut]) * out;
        totalOut += out - slip;
        poolBalances[tokenOut] -= out - slip;
        poolBalances[tokenIn] += increments;
    }
    console.log("Total out: " + totalOut);
    // let kusdOut = kusdChangingLiq * rmrkInput / (rmrkChangingLiq + rmrkInput);
    // let slip = (kusdOut / kusdChangingLiq) * kusdOut;
    // totalKusd += kusdOut - slip;
    // kusdChangingLiq -= kusdOut - slip;
    // rmrkChangingLiq += rmrkInput;
    // totalSlip += slip;

    // i++;
}

async function getKsmLksmBalance(api: any) {
    // const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    // const api = new ApiPromise(options({ provider }));
    await api.isReady;
    const ksmLksmPool = 'qmmNug1GQstpimAXBphJPSbDawH47vwMmhuSUq9xRqAsDAr';
    const ksm = {
        Token: "KSM"
    }
    const lksm = {
        Token: "LKSM"
    }
    let ksmBalance = await api.query.tokens.accounts(ksmLksmPool, ksm);
    let lksmBalance = await api.query.tokens.accounts(ksmLksmPool, lksm);
    // console.log(ksmBalance.toHuman())
    // console.log(lksmBalance.toJSON())
    let ksmFormatted = ksmBalance.toHuman().free.toString().replace(/,/g, "");
    let lksmFormatted = lksmBalance.toHuman().free.toString().replace(/,/g, "");
    // console.log(ksmFormatted)
    // console.log(lksmFormatted)
    return [ksmFormatted, lksmFormatted]
}

async function querySubscan() {
    // karura.api.subscan.io
    const apiKey = "6b15635a685849d2a0afb1b33754f73d";
    // 'https://statemine.api.subscan.io/api/scan/assets/assets'
    const url = 'https://karura.api.subscan.io/api/scan/accounts/tokens';
    const headers = {
        'Content-Type': 'application/json',
        'X-API-Key': apiKey,
    };
    const data = {
        'address': 'sFbqBjxPBgfBmGPgxSWqweKLnHawKvMc9K1JNgcijAZjt1X'
        // row: 1,
        // page: 0
    };

    let response = await fetch(url, {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
    })
        .then((response: any) => response.json())
        .then((data: any) => console.log(data))
        .catch((error: any) => console.error(error));
}

async function getLps() {
    // console.log(JSON.parse(fs.readFileSync('../kar/lps.json', 'utf8')))
    return JSON.parse(fs.readFileSync('../kar/lps.json', 'utf8'))
}

async function testStableSwap() {
    // liquidityStats: ['1030529523909449995', '33112418564000000', '32952968957000000']
    let kusdLiquidity = 1030529523909449995;
    let usdcLiquidity = 33112418564000000;
    let usdtLiquidity = 32952968957000000;

    let inputTether = 10;
    let input = inputTether;
    console.log("Input tether: " + input)
    let increments = input / 10;
    input = increments - (increments * 0.003)
    let i = 0;
    let kusdChangingLiq = kusdLiquidity / (10 ** 12);
    let usdtChangingLiq = usdtLiquidity / (10 ** 12);
    let usdcChangingLiq = usdcLiquidity / (10 ** 12);
    let totalUsdChangingLiq = ((usdcLiquidity / (10 ** 12)) + (usdtLiquidity / (10 ** 12)))
    let totalKusd = 0;
    let totalSlip = 0

    // SUMxi = D
    // PRODUCTxi = (D/n)^n
    const sum = kusdChangingLiq + usdtChangingLiq + usdcChangingLiq;
    const prod = Math.pow((sum / 3), 3);
    const prod2 = kusdChangingLiq * usdtChangingLiq * usdcChangingLiq;
    console.log("SUM: " + sum)
    console.log("PRODUCT: " + prod)
    console.log("PRODUCT2: " + prod2)
}
async function readStables() {
    return JSON.parse(fs.readFileSync('../kar/stablePools.json', 'utf8'))

}

// async function queryStableLps() {

// }
async function swapKsmLksm(api: any) {
    // let pools = await queryStableLps(api);
    let pools = await readStables()
    let ksmPool2 = await getKsmLksmBalance(api);
    console.log(ksmPool2)
    let ksmPool = pools[0];
    let usdPool = pools[1];
    // console.log(ksmPool);
    // await getSwapAmount(usdPool, 0, 1, 50.34, 100)
    // console.log("-----")
    await getSwapAmount2(usdPool, 0, 1, 50.34, 100)
    console.log("-----------------")
    // await getSwapAmount(ksmPool, 0, 1, 0.863, 0.09)
    // console.log("-----")
    let out = await getSwapAmount2(ksmPool, 1, 0, 7.751364477999, 30)
    // let out = await getSwapAmount2(ksmPool, 0, 1, 1, 30)
    console.log(`LKSM pool 1 : ${ksmPool.liquidityStats[1]} pool 2: ${ksmPool2[1]}`)
    let lksmRation = BigNumber(ksmPool2[1]).div(BigNumber(ksmPool.liquidityStats[1]))
    // let totalOut = BigNumber(out).div(lksmRation)
    // let totalOutToLksm = lksmRation.times(BigNumber(out))
    let totalOutToKsm = BigNumber(out).div(lksmRation)
    console.log("Lksm Ratio: " + lksmRation)
    console.log("Out LKSM: " + out)
    console.log("Total Out: " + totalOutToKsm)
    // getDexSwapAmount(ksmPool, 0, 1, 1)

}

async function getSwapAmount2(pool: StableSwapPool, tokenIn: number, tokenOut: number, input: number, A: number) {
    let poolBalances = (pool.liquidityStats as any).map((liq: any) => {
        // return liq / (10 ** 12)
        return BigNumber(liq)
    });
    let a = BigNumber(A);
    let dx = BigNumber(input);
    console.log("Pool Balances")
    console.log(poolBalances)
    let d = await getD2(poolBalances, A);
    poolBalances[tokenIn] = poolBalances[tokenIn].plus(dx);
    let y = getY2(poolBalances, tokenOut, d, a);
    let dy = poolBalances[tokenOut].minus(y);
    console.log("poolBalances[tokenOut]: " + poolBalances[tokenOut])
    console.log("y                     : " + y)

    let swapFee = BigNumber(pool.swapFee.replace(/,/g, "") as any as number);
    let feePrecisions = BigNumber(10000000000);
    let feeAmount = dy.times(swapFee).div(feePrecisions);
    console.log("Fee: " + feeAmount)

    poolBalances[tokenOut] = y
    // totalOut += dy - feeAmount;
    let totalOut = dy.minus(feeAmount);
    console.log("Total out: " + totalOut)
    return totalOut
}

async function getSwapAmount(pool: StableSwapPool, tokenIn: number, tokenOut: number, input: number, A: number) {
    let poolBalances = (pool.liquidityStats as any).map((liq: any) => {
        return liq / (10 ** 12)
        // return liq
    });
    // let poolBalances = pool.liquidityStats;
    console.log(poolBalances)
    let sum = poolBalances.reduce((a: number, b: number) => a + b, 0);
    console.log("D sum: " + sum)
    // console.log("totalSupply: " + totalSupply)
    // let A = .09;
    // let A = 3000;
    // let A = 100;
    // let poolBalances = ksmL.map((liq: any) => {
    //     return (liq.replace(/,/g, "")) / (10 ** 12)
    // })

    let swapFee = pool.swapFee.replace(/,/g, "") as any as number;
    let feePrecisions = 10000000000;

    let totalOut = 0;
    let increments = 1;
    let dx = input / increments;
    for (let i = 0; i < increments; i++) {
        // console.log("Input: " + dx)
        // let balances = [kusdChangingLiq, usdcChangingLiq, usdtChangingLiq];
        let balances = poolBalances;
        // console.log(balances)
        let D = getD(balances, A);
        let D2 = sum;
        console.log(D + " --- " + D2)
        balances[tokenIn] = balances[tokenIn] + dx;
        console.log(balances)
        let y = getY(balances, tokenOut, D, A);
        let dy = (balances[tokenOut] - y);

        let feeAmount = dy * swapFee / feePrecisions;
        console.log("Fee: " + feeAmount)

        balances[tokenOut] = y
        totalOut += dy - feeAmount;
    }
    console.log("Total Output: " + totalOut)

}

async function main() {
    const provider = new WsProvider('wss://karura.api.onfinality.io/public-ws');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;
    // await saveLps()
    // await getLps()
    await queryStableLps(api);
    await swapKsmLksm(api);

    process.exit(0)
}
// let A = 100.1;


function getD(balances: any, A: any) {
    let sum = 0;
    let i = 0;
    // let _A = 1;
    let Ann = A;
    
    for (i = 0; i < balances.length; i++) {
        sum = sum + balances[i];
        Ann = Ann * balances.length;
    }

    let prevD = 0;
    let D = sum;
    console.log("Start D: " + D);
    for (i = 0; i < 256; i++) {
        console.log("D: " + D)
        let pD = D;
        for (let j = 0; j < balances.length; j++) {
            // pD = pD * D / (_x * balance.length)
            pD = pD * D / (balances[j] * balances.length);
        }
        prevD = D
        // D = (Ann * sum + pD * balance.length) * D / ((Ann - 1) * D + (balance.length + 1) * pD)
        console.log(`Ann: ${Ann}, sum: ${sum}, pD: ${pD}, balances.length: ${balances.length}`)
        D = (Ann * sum + pD * balances.length) * D / ((Ann - 1) * D + (balances.length + 1) * pD);
        // console.log("D: " + D)
        if (D > prevD) {
            if (D - prevD <= 1) break;
        } else {
            if (prevD - D <= 1) break;
        }
    }
    return D;
}
async function getD2(balances: any, A: any) {
    let sum = BigNumber(0);
    let one = BigNumber(1);
    let i = 0;
    // let _A = 1;
    let Ann = BigNumber(A);
    let balance_size = BigNumber(0);
    console.log("A: " + A);
    if (A == 30) {
        balance_size = BigNumber(balances.length - 2);
        for (i = 0; i < balance_size.toNumber(); i++) {
            sum = sum.plus(balances[i]);
            Ann = Ann.times(balance_size);
        }
    } else {
        balance_size = BigNumber(balances.length);
        for (i = 0; i < balance_size.toNumber(); i++) {
            sum = sum.plus(balances[i]);
            Ann = Ann.times(balance_size);
        }
    }
    console.log("BALANCE SIZE: " + balance_size)
    console.log("balance size: " + balance_size.toNumber())
    

    let prevD = BigNumber(0);
    let D = sum;
    console.log("Start D: " + D);
    for (i = 0; i < 256; i++) {
        console.log("D: " + D)
        let pD = D;
        for (let j = 0; j < balance_size.toNumber(); j++) {
            // pD = pD * D / (_x * balance.length)
            pD = pD.times(D).div(balances[j].times(balance_size));
        }
        // console.log("pD: " + pD)
        prevD = D
        // D = (Ann * sum + pD * balance.length) * D / ((Ann - 1) * D + (balance.length + 1) * pD)
        
        let t_1 = Ann.times(sum).plus(pD.times(balance_size));
        // console.log("t_1: " + t_1)
        // console.log(`Ann: ${Ann}, One: 1, D: ${D}`)
        let t_2 = Ann.minus(1).times(D)
        // console.log("t_2: " + t_2)
        let t_3 = balance_size.plus(1).times(pD)
        // console.log("t_3: " + t_3)
        let t_4 = t_2.plus(t_3)
        // console.log("t_4: " + t_4)
        let t_5 = t_1.times(D).div(t_4);
        // console.log("t_5: " + t_5)

        D = Ann.times(sum).plus(pD.times(balance_size)).times(D).div(Ann.minus(1).times(D).plus(balance_size.plus(1).times(pD)));
        if (D.gt(prevD)) {
            if (D.minus(prevD) <= one) break;
        } else {
            if (prevD.minus(D) <= one) break;
        }
    }
    return D;
}
function getY2(balances: BigNumber[], j: number, D: BigNumber, A: any) {
    let c = D;
    let S = BigNumber(0);
    let Ann = A;
    let one = BigNumber(1);
    let i = 0;
    let balance_size = BigNumber(0);

    console.log("A y: " + A)
    if (A == 30) {
        balance_size = BigNumber(balances.length - 2);
        console.log("Balance size y: " + balance_size)
        for (i = 0; i < balance_size.toNumber(); i++) {
            Ann = Ann.times(balance_size);
            if (i == j) continue
            S = S.plus(balances[i]);
            c = c.times(D).div(balances[i].times(balance_size));
            console.log("c: " + c)
        }
    } else {
        balance_size = BigNumber(balances.length);
        for (i = 0; i < balance_size.toNumber(); i++) {
            Ann = Ann.times(balance_size);
            if (i == j) continue
            S = S.plus(balances[i]);
            c = c.times(D).div(balances[i].times(balance_size));
            console.log("c: " + c)
        }
    }
    
    console.log("BALANCE SIze y: " + balance_size)
    c = c.times(D).div(Ann.times(balance_size));
    let b = S.plus(D.div(Ann));
    let x = D.div(Ann);
    console.log("Ann: " + Ann)
    console.log("X: " + x)
    let prevY = BigNumber(0);
    let y = D;
    console.log(`c: ${c}, b: ${b}, y: ${y}`);
    for (i = 0; i < 256; i++) {
        prevY = y;
        y = y.times(y).plus(c).div(y.times(2).plus(b).minus(D));
        console.log("Y: " + y)
        if (y.gt(prevY)) {
            if (y.minus(prevY) <= one) break;
        } else {
            if (prevY.minus(y) <= one) break;
        }
    }
    return y;
}
function getY(balances: any, j: number, D: number, A: number ) {
    let c = D;
    let S = 0;
    let Ann = A;
    let i = 0;

    for (i = 0; i < balances.length; i++) {
        Ann = Ann * balances.length;
        if (i == j) continue
        S = S + balances[i];
        c = c * D / (balances[i] * balances.length);
    }

    c = c * D / (Ann * balances.length);
    let b = S + D / Ann;
    let prevY = 0;
    let y = D;

    for (i = 0; i < 256; i++) {
        prevY = y;
        y = (y * y + c) / (2 * y + b - D);
        console.log("Y: " + y)
        if (y > prevY) {
            if (y - prevY <= 1) break;
        }
        else {
            if (prevY - y <= 1) break;
        }
    }
    return y;
}

// main()
