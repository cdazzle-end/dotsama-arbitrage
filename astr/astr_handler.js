var fileReader = require('./text_reader')

// 1. Import ethers
const ethers = require('ethers');
const { parse } = require('path');
const { formatUnits } = require('ethers/lib/utils');
const fs = require('fs');
const { fail } = require('assert');

// 2. Define network configurations
const providerRPC = {
    astar: {
        name: 'astar',
        rpc: 'https://astar.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 592, // 0x504 in hex,
    },
};

// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.astar.rpc,
    {
        chainId: providerRPC.astar.chainId,
        name: providerRPC.astar.name,
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

// MANUALLY GET ASTR TOKEN HOLDERS

// 1) GET ALL ASSETS THRU ASTR TRADING PAIR
// 1a) Start with ASTR holders
// 1b) Find dex contracts, get token pair
//
// 2) Get token pair holders
// 2a) Use COVALENT api for token holders
// 2b) manually get the rest of token holders

// 3) GET LIQ POOLS FOR ALL ASSETS
//
// 4) Build LIQPOOL registry and TOKEN registry
//

//MANUALLY SAVE ASTR TOKEN HOLDERS AND PARSE THE TEXT
async function getAstrLiqPools() {
    let astrHolders = fileReader.readAddressFile('wastr.txt');
    console.log(astrHolders)

    let astrLps = [];
    for (i = 0; i < astrHolders.length; i++) {
        const is_dex = await isDex(astrHolders[i])
        if (is_dex) {
            astrLps.push(astrHolders[i]);
            console.log("yes")
        } else {
            console.log("no")
        }
    }
    console.log(astrLps)
    console.log(astrLps.length)
    fs.writeFileSync('wastr_liq_pool_addresses.txt', astrLps.toString(), 'utf8')
}

//API to get token holders
async function getAstrTokenHoldersBlockscout() {
    let tokenHolders = [];
    for (i = 1; i < 20; i++){
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        const res = await fetch(uri)
        const answer = await res.json()
        console.log(answer.result.length)
        for (j = 0; j < answer.result.length; j++){
            let holderAddress = answer.result[j].address;
            tokenHolders.push(holderAddress)
            // console.log(holderAddress)
        }
    }
    console.log(tokenHolders)
    console.log(tokenHolders.length)

    fs.writeFileSync('wastr.txt', tokenHolders.toString(), 'utf8');
}

async function getAstrLiqPools() {
    let data = fs.readFileSync('wastr.txt', 'utf8');
    data = data.split(',');

    let lpArray = [];
    for (i = 0; i < data.length; i++){
        console.log(i)
        let is_dex = await isDex(data[i]);
        if (is_dex) {
            lpArray.push(data[i])
            console.log("TREU")
        }
    }
    console.log(lpArray)
    console.log(lpArray.length)

    fs.writeFileSync('wastr_liq_pool_addresses.txt', lpArray.toString(), 'utf8');
}

// All assets that have a pair trading against astr
async function getAllAssets() {
    let data = fs.readFileSync('wastr_liq_pool_addresses.txt', 'utf8');
    data = data.split(',');

    let allAssets = [];
    for (i = 0; i < data.length; i++){
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

async function getAllTokenHolders() {
    let data = fs.readFileSync('all_assets.txt', 'utf8');
    data = data.split(',');

    let successArray = [];
    let failArray = [];
    let allTokenHolders = [];
    for (i = 0; i < data.length; i++) {
        // const uri = 'https://api.covalenthq.com/v1/1284/tokens/' + data[i] + '/token_holders/?key=ckey_7582964824a047a1985ba46d29a';
        console.log("Getting token holders for " + data[i])
        console.log("INDEX: " + i)
        try {
            for (j = 1; j < 11; j++) {
                console.log("PAGE " + j)
                try {
                    let page = j;
                    let offset = 50;
                    const uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=${data[i]}&page=${page}&offset=${offset}`;
                    const res = await fetch(uri)
                    const answer = await res.json()
                    // console.log(answer)
                    
                    successArray.push(data[i])
                    // let holderAddress = 
                    answer.result.map((res) => {
                        console.log(res.address)
                        allTokenHolders.push(res.address);
                    });
                } catch (err) {
                    console.log(err)
                    console.log("Failed to query more assets")
                    console.log(data[i])
                }
            }
            console.log("SUCCESS: QUERY NEW TOKEN")
            successArray.push(data[i])
        } catch (err) {
            console.log(err)
            console.log("BIG FAIL")
            failArray.push(data[i]);
        }
    }

    console.log("ALL")
    console.log(allTokenHolders.length)
    console.log("SUCCESS")
    console.log(successArray.length)
    console.log("FAILURE")
    console.log(failArray.length)
    // let manualTokenHolders = getManual();
    // allTokenHolders.push.apply(allTokenHolders, manualTokenHolders);

    fs.writeFileSync("all_token_holders.txt", allTokenHolders.toString(), 'utf8')
}

async function testTokenHolders() {
    let data = fs.readFileSync("all_token_holders.txt", 'utf8').split(',');
    let uniqueAddr = [];
    for (i = 0; i < data.length; i++){
        if (!uniqueAddr.includes(data[i])) {
            uniqueAddr.push(data[i])
        }
    }

    console.log(uniqueAddr.length)
    console.log(data.length)
}

async function testPromises() {
    let data = fs.readFileSync('all_assets.txt', 'utf8');
    data = data.split(',');

    let promises = []
    let successArray = [];
    let failArray = [];
    let allTokenHolders = [];
    for (i = 0; i < 3; i++) {
        // const uri = 'https://api.covalenthq.com/v1/1284/tokens/' + data[i] + '/token_holders/?key=ckey_7582964824a047a1985ba46d29a';
        console.log("Getting token holders for " + data[i])
        console.log("INDEX: " + i)
        try {
            for (j = 1; j < 11; j++) {
                console.log("PAGE " + j)
                try {
                    let page = j;
                    let offset = 50;
                    const uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=${data[i]}&page=${page}&offset=${offset}`;
                    const res = await fetch(uri)
                    const answer = await res.json()
                    // console.log(answer)
                    console.log("SUCCESS: QUERY NEW TOKEN")
                    successArray.push(data[i])
                    // let holderAddress = 
                    answer.result.map((res) => {
                        console.log(res.address)
                        // allTokenHolders.push(res.address);
                        promises.push(res.address)
                    });
                } catch (err) {
                    console.log(err)
                    console.log("Failed to query more assets")
                    console.log(data[i])
                }
            }
            successArray.push(data[i])
        } catch (err) {
            console.log(err)
            console.log("BIG FAIL")
            failArray.push(data[i]);
        }
    }
    console.log("Promises")
    console.log(await promises)
    // console.log("ALL")
    // console.log(allTokenHolders.length)
    // console.log("SUCCESS")
    // console.log(successArray.length)
    // console.log("FAILURE")
    // console.log(failArray.length)
}

async function testPromise2() {
    let array = [];
    let myPromise = new Promise(function (myResolve, myReject) {
        setTimeout(function () { myResolve("I love You !! ONE"); }, 3000);
    });
    let myPromise2 = new Promise(function (myResolve, myReject) {
        setTimeout(function () { myResolve("I love You !! TWO"); }, 1000);
    });

    myPromise.then(function (value) {
        // console.log(value);
        array.push(value)
    });

    myPromise2.then(function (value) {
        array.push(value)
    })

    console.log(await array)
}

// Using list of all token holders, filter out dex contracts
async function getAllLiquidityPools() {
    let data = fs.readFileSync('all_token_holders.txt', 'utf8');
    data = data.split(',')

    let allPools = [];
    for (i = 0; i < data.length; i++) {
        console.log(i)
        let is_dex = await isDex(data[i]);
        if (is_dex) {
            allPools.push(data[i]);
        }
    }
    console.log(allPools)
    console.log(allPools.length)
    fs.writeFileSync('all_liq_pools.txt', allPools.toString(), 'utf8')
}

async function getLps() {
    let data = fs.readFileSync('all_liq_pools.txt', 'utf8');
    data = data.split(',')

    console.log(data)
    console.log(data.length)
}

//Get all liquidity pools and save them as LiqPool and Token objects
async function buildRegistry() {
    let data = fs.readFileSync('all_liq_pools.txt', 'utf8');
    data = data.split(',');
    console.log(data.length)
    let liqPools = [];
    let tokenAddresses = [];
    let tokens = [];
    for (i = 0; i < data.length; i++) {
        console.log(i)
        try {
            const contract = new ethers.Contract(data[i], dexContractAbi, provider);
            const token_0 = await contract.token0();
            const token_1 = await contract.token1();
            const reserves = await contract.getReserves();

            let token_0_contract = new ethers.Contract(token_0, tokenContractAbi, provider);
            let token_0_name = await token_0_contract.name();
            let token_0_symbol = await token_0_contract.symbol();
            let token_0_decimals = await token_0_contract.decimals();
            let token_0_obj = new Token(token_0, token_0_symbol, token_0_name, token_0_decimals);

            console.log("TOKEN 0")
            console.log(token_0_obj)

            let token_1_contract = new ethers.Contract(token_1, tokenContractAbi, provider);
            let token_1_name = await token_1_contract.name();
            let token_1_symbol = await token_1_contract.symbol();
            let token_1_decimals = await token_1_contract.decimals();
            let token_1_obj = new Token(token_1, token_1_symbol, token_1_name, token_1_decimals);

            console.log("TOKEN 1")
            console.log(token_1_obj)

            if (!tokenAddresses.includes(token_0)) {
                tokenAddresses.push(token_0)
                tokens.push(token_0_obj)
            }

            if (!tokenAddresses.includes(token_1)) {
                tokenAddresses.push(token_1)
                tokens.push(token_1_obj)
            }

            let liqPool = new LiqPool(data[i], token_0_obj, token_1_obj, reserves);
            console.log(liqPool)
            liqPools.push(liqPool)

        } catch (err) {
            try {
                const contract = new ethers.Contract(data[i], dexContractAbi, provider);
                const token_0 = await contract.token0();
                const token_1 = await contract.token1();
                const reserves = await contract.getReserves();

                let token_0_contract = new ethers.Contract(token_0, tokenContractAbi, provider);
                let token_0_name = await token_0_contract.name();
                let token_0_symbol = await token_0_contract.symbol();
                let token_0_decimals = await token_0_contract.decimals();
                let token_0_obj = new Token(token_0, token_0_symbol, token_0_name, token_0_decimals);

                console.log("TOKEN 0")
                console.log(token_0_obj)

                let token_1_contract = new ethers.Contract(token_1, tokenContractAbi, provider);
                let token_1_name = await token_1_contract.name();
                let token_1_symbol = await token_1_contract.symbol();
                let token_1_decimals = await token_1_contract.decimals();
                let token_1_obj = new Token(token_1, token_1_symbol, token_1_name, token_1_decimals);

                console.log("TOKEN 1")
                console.log(token_1_obj)

                if (!tokenAddresses.includes(token_0)) {
                    tokenAddresses.push(token_0)
                    tokens.push(token_0_obj)
                }

                if (!tokenAddresses.includes(token_1)) {
                    tokenAddresses.push(token_1)
                    tokens.push(token_1_obj)
                }

                let liqPool = new LiqPool(data[i], token_0_obj, token_1_obj, reserves);
                console.log(liqPool)
                liqPools.push(liqPool)
            } catch (err) {
                console.log("Failed to create LiqPool")
                console.log(err)
            }
        }


    }
    console.log(liqPools)
    console.log(tokens)
    console.log(liqPools.length)
    console.log(tokens.length)

    for (i = 0; i < tokens.length; i++) {
        // tokens[i]
    }

    fs.writeFileSync('liq_pool_registry.txt', JSON.stringify(liqPools), 'utf8')
    fs.writeFileSync('token_registry.txt', JSON.stringify(tokens), 'utf8')
}

async function getRegistry() {
    let data = fs.readFileSync('liq_pool_registry.txt', 'utf8');
    data = JSON.parse(data);

    let tokens = fs.readFileSync('token_registry.txt', 'utf8');
    tokens = JSON.parse(tokens);

    console.log(tokens);
}

async function main() {

    buildRegistry()
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