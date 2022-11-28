// 1. Import ethers
const ethers = require('ethers');
const { parse } = require('path');
const { formatUnits } = require('ethers/lib/utils');
const fs = require('fs');
const { fail } = require('assert');

// 2. Define network configurations
const providerRPC = {
    sdn: {
        name: 'sdn',
        rpc: 'https://shiden.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 592, // 0x504 in hex,
    },
};

// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.sdn.rpc,
    {
        chainId: providerRPC.sdn.chainId,
        name: providerRPC.sdn.name,
    }
);

const dexContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function getReserves() view returns (uint, uint, uint)",
    "function token0() view returns (address)",
    "function token1() view returns (address)"
]

//getReserves returns 2 uint instead of 3
const altDexContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function getReserves() view returns (uint, uint)",
    "function token0() view returns (address)",
    "function token1() view returns (address)"
]

const tokenContractAbi = [
    "function name() view returns (string)",
    "function symbol() view returns (string)",
    "function decimals() view returns (uint8)",
    "event Transfer(address indexed src, address indexed dst, uint val)"
]

async function testProvider() {
    let block = await provider.getBlockNumber()
    console.log(block)
}

//Get SDN holders
//Get SDN dex's
//Get SDN dex trading pair
//Get trading pair holders
//Get trading pair dexs
//Get trading pair trading pair
async function getSdnTokenHoldersBlockscout() {
    let tokenHolders = [];
    for (i = 1; i < 20; i++) {
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        const res = await fetch(uri)
        const answer = await res.json()
        console.log(answer.result.length)
        for (j = 0; j < answer.result.length; j++) {
            let holderAddress = answer.result[j].address;
            tokenHolders.push(holderAddress)
            // console.log(holderAddress)
        }
    }
    console.log(tokenHolders)
    console.log(tokenHolders.length)

    fs.writeFileSync('wsdn.txt', tokenHolders.toString(), 'utf8');
}

async function getSdnLiqPools() {
    let data = fs.readFileSync('wsdn.txt', 'utf8');
    data = data.split(',');

    let lpArray = [];
    for (i = 0; i < data.length; i++) {
        console.log(i)
        let is_dex = await isDex(data[i]);
        if (is_dex) {
            lpArray.push(data[i])
            console.log("TREU")
        }
    }
    console.log(lpArray)
    console.log(lpArray.length)

    fs.writeFileSync('wsdn_liq_pool_addresses.txt', lpArray.toString(), 'utf8');
}

async function getAllAssets() {
    let data = fs.readFileSync('wsdn_liq_pool_addresses.txt', 'utf8');
    data = data.split(',');

    let allAssets = [];
    for (i = 0; i < data.length; i++) {
        try {
            let dexContract = new ethers.Contract(data[i], dexContractAbi, provider);
            console.log("dex contract")

            let token0 = await dexContract.token0();
            let token1 = await dexContract.token1();
            if (!allAssets.includes(token0)) {
                allAssets.push(token0)
            }
            if (!allAssets.includes(token1)) {
                allAssets.push(token1)
            }
        } catch (err) {
            try {
                let dexContract = new ethers.Contract(data[i], altDexContractAbi, provider)
                console.log("ALT DEX CONTRACT")

                let token0 = await dexContract.token0();
                let token1 = await dexContract.token1();
                if (!allAssets.includes(token0)) {
                    allAssets.push(token0)
                }
                if (!allAssets.includes(token1)) {
                    allAssets.push(token1)
                }
            } catch (err) {
                console.log(err)
                console.log("FAILED TO MAKE DEX CONTRACT")
            }
        }
    }

    console.log(allAssets)
    console.log(allAssets.length)
    fs.writeFileSync('all_assets.txt', allAssets.toString(), 'utf8')
}

async function checkForMoreAssets() {
    let data = fs.readFileSync('all_assets.txt', 'utf8');
    data = data.split(',');

    let moreAssets = [];
    moreAssets.push.apply(moreAssets, data);
    for (i = 1; i < data.length; i++) {

        for (k = 1; k < 10; k++) {
            let page = k;
            let offset = 50;
            let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=${data[i]}&page=${page}&offset=${offset}`;
            const res = await fetch(uri)
            const answer = await res.json()
            console.log(answer.result.length)
            for (j = 0; j < answer.result.length; j++) {
                let holderAddress = answer.result[j].address;
                let is_dex = isDex(holderAddress);
                if (is_dex) {
                    try {
                        let dexContract = new ethers.Contract(holderAddress, dexContractAbi, provider);
                        console.log("dex contract")

                        let token0 = await dexContract.token0();
                        let token1 = await dexContract.token1();
                        if (!moreAssets.includes(token0)) {
                            moreAssets.push(token0)
                        }
                        if (!moreAssets.includes(token1)) {
                            moreAssets.push(token1)
                        }
                    } catch (err) {
                        try {
                            let dexContract = new ethers.Contract(holderAddress, altDexContractAbi, provider)
                            console.log("ALT DEX CONTRACT")

                            let token0 = await dexContract.token0();
                            let token1 = await dexContract.token1();
                            if (!moreAssets.includes(token0)) {
                                moreAssets.push(token0)
                            }
                            if (!moreAssets.includes(token1)) {
                                moreAssets.push(token1)
                            }
                        } catch (err) {
                            console.log(err)
                            console.log("FAILED TO MAKE DEX CONTRACT")
                        }
                    }
                }
                // tokenHolders.push(holderAddress)
                // console.log(holderAddress)
            }
        }
    }
    console.log(moreAssets)
    console.log(moreAssets.length)

    fs.writeFileSync('more_assets.txt', moreAssets.toString(), 'utf8')

}

async function main() {
    // await testProvider();
    // await getSdnTokenHoldersBlockscout();
    await getSdnLiqPools();
    await getAllAssets();
    await checkForMoreAssets();
    
}

main().then(() => console.log("Complete"))

//RETURNS TRUE OR FALSE
async function isDex(dexAddress) {
    const code = await provider.getCode(dexAddress);
    // console.log(code)
    if (code == "0x") {
        // console.log("Not a contract")
        return false;
    } else {
        try {

            const potentialContract = new ethers.Contract(dexAddress, dexContractAbi, provider);
            const name = await potentialContract.name();
            const token_0 = await potentialContract.token0();
            const token_1 = await potentialContract.token1();
            console.log("True")
            console.log(`lp Name: ${name}`)
            console.log(`1: ${token_0} - 2: ${token_1}`)
            return true;
        } catch {
            // console.log("Not a liq pool")
            return false;
        }
    }
}


async function getManual() {
    let data = fs.readdirSync('C:/Users/dazzl/substrate_projects/arb_project/glmr/manual_addresses/');
    let holderAddresses = [];
    for (i = 0; i < data.length; i++) {
        console.log(data[i])
        let filePath = "manual_addresses/" + data[i];
        let tokenData = fileReader.readAddressFile(filePath)
        holderAddresses.push.apply(holderAddresses, tokenData);
    }
    console.log(holderAddresses)
    console.log(holderAddresses.length)
    return holderAddresses;
}

class Token {
    constructor(address, symbol, name, decimals) {
        this.address = address;
        this.symbol = symbol;
        this.name = name;
        this.decimals = decimals;
    }

    print() {
        console.log("Token Address: " + this.address)
        console.log(`${this.name} - ${this.symbol} - decimals ${this.decimals}`)
    }
}

class LiqPool {
    constructor(address, token_1, token_2, reserves) {
        this.address = address;
        this.token_1 = token_1;
        this.token_2 = token_2;
        this.reserves = reserves;
    }

    print() {
        console.log(`${this.address}`)
    }

    getToken1() {

    }
}