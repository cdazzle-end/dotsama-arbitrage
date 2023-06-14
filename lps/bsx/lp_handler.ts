import * as fs from 'fs';
import { MyLp } from '../lp_types';
import { Keyring, ApiPromise, WsProvider } from '@polkadot/api';
import { BigNumber } from 'bignumber.js';

export async function updateLps() {
    const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;

    let parachainId = await (await api.query.parachainInfo.parachainId()).toHuman() as any;
    parachainId = parachainId.replace(/,/g, "");
    let poolAssets = await api.query.xyk.poolAssets.entries();
    let lps = await Promise.all(poolAssets.map(async ([assetPoolAccount, assets]: any) => {
        let assetIds = assets.toJSON() as any;
        let accountFormatted = (assetPoolAccount.toHuman() as any)[0]
        let tokenLiqs = await Promise.all(assetIds.map(async (id: any) => {

            if (id == 0) {
                let accountData = await api.query.system.account(accountFormatted);
                let bsxLiq = (accountData.toHuman() as any).data.free.replace(/,/g, "")
                return bsxLiq
            } else {
                let accountData = await api.query.tokens.accounts(accountFormatted, id);
                let tokenLiq = (accountData.toHuman() as any).free.replace(/,/g, "")
                return tokenLiq
            }
        }))

        let assetIdsString = assetIds.map((id: any) => id.toString())

        let newLp: MyLp = {
            chainId: parseInt(parachainId),
            poolAssets: assetIdsString,
            liquidityStats: tokenLiqs
        }
        return newLp
    }))
    fs.writeFileSync("./bsx/lps.json", JSON.stringify(lps, null, 2), "utf8");
    api.disconnect()
}

async function saveLps() {
    const provider = new WsProvider('wss://basilisk-rpc.dwellir.com');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady;

    let parachainId = await (await api.query.parachainInfo.parachainId()).toHuman() as any;
    parachainId = parachainId.replace(/,/g, "");
    let poolAssets = await api.query.xyk.poolAssets.entries();
    let poolLiquidity = await api.query.xyk.totalLiquidity.entries();
    let lps = await Promise.all(poolAssets.map(async ([assetPoolAccount, assets]: any) => {
        let liquidity = poolLiquidity.find(([poolAccount, liquidity]) => {
            let pool1 = assetPoolAccount.toHuman();
            let pool2 = poolAccount.toHuman() as any;
            return pool1[0] === pool2[0]
        })

        // let liq0 = liquidity![0]
        let liqTotal = liquidity![1]
        // let testQ = await api.query.system.account("bXnMD3c7JFY4VhT8km2rcWv1Gs1DZFf6Fdm737HsLPsm4pQQ6");
        // console.log(testQ.toHuman())

        let assetIds = assets.toJSON() as any;
        console.log(assetIds)
        let accountFormatted = (assetPoolAccount.toHuman() as any)[0]
        let tokenLiqs = await Promise.all(assetIds.map(async (id: any) => {
            
            if (id == 0) {
                let accountData = await api.query.system.account(accountFormatted);
                let bsxLiq = (accountData.toHuman() as any).data.free.replace(/,/g, "")
                return bsxLiq
            } else {
                let accountData = await api.query.tokens.accounts(accountFormatted, id);
                let tokenLiq = (accountData.toHuman() as any).free.replace(/,/g, "")
                return tokenLiq
            }
        }))
        // console.log("Token liquidity: ")
        // console.log(tokenLiqs)
        console.log("Account: " + accountFormatted)
        let totalCalculate = tokenLiqs.reduce((a: any, b: any) => {
            console.log(a)
            console.log(b)
            return parseInt(a) * parseInt(b)
        })
        console.log("Total liquidity: " + totalCalculate)

        // console.log("LP: ")
        console.log(assetPoolAccount.toHuman())
        console.log("Pool liq " + liqTotal.toHuman())

        let assetIdsString = assetIds.map((id: any) => id.toString())

        let newLp: MyLp = {
            chainId: parseInt(parachainId),
            poolAssets: assetIdsString,
            liquidityStats: tokenLiqs
        }

        // console.log(newLp)
        return newLp
    }))
    console.log(lps)
    fs.writeFileSync("./lps.json", JSON.stringify(lps, null, 2), "utf8");
    api.disconnect()
}

async function calculateSwap() {
    let bsx_assets = JSON.parse(fs.readFileSync("../../assets/bsx/asset_registry.json", "utf8"));
    // let input_ksm = 1 * 10 ** 12;
    let input_amount = BigNumber(1);

    let pools = JSON.parse(fs.readFileSync("./lps.json", "utf8"));
    let bsxKsmPool = pools[5];
    console.log(bsxKsmPool)
    let bsxLiq = BigNumber(bsxKsmPool.liquidityStats[0]).div(BigNumber(10).pow(12));
    let ksmLiq = BigNumber(bsxKsmPool.liquidityStats[1]).div(BigNumber(10).pow(12));
    let input_index = 1
    let output_index = 0
    let inputLiq = BigNumber(bsxKsmPool.liquidityStats[input_index]).div(BigNumber(10).pow(12))
    // let inputLiq = bsxKsmPool[input_index]
    let outputLiq = BigNumber(bsxKsmPool.liquidityStats[output_index]).div(BigNumber(10).pow(12))

    let increments = input_amount.div(BigNumber(100));
    let totalOut = BigNumber(0);
    console.log("Input liq: " + inputLiq)
    console.log("Output liq: " + outputLiq)
    console.log("Input amount: " + input_amount)
    for (let i = 0; i < 100; i++) {
        let out = outputLiq.times(increments).div(inputLiq.plus(increments));
        // console.log(out)
        let slip = (out.div(outputLiq)).times(out);
        totalOut = totalOut.plus(out.minus(slip));
        outputLiq= outputLiq.minus(out.minus(slip));
        inputLiq = inputLiq.plus(increments);
    }
    // totalOut = totalOut / 10 ** 12;
    let swapFee = totalOut.times(0.003);
    totalOut = totalOut.minus(swapFee);
    console.log("Total out: " + totalOut);
    // let formatted_input = 

}

async function main() {
    await saveLps()
    // await calculateSwap()
}

// main().then(() => console.log("complete"))