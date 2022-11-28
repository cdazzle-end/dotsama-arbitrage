var fileReader = require('./text_reader')

// 1. Import ethers
const ethers = require('ethers');
const { parse } = require('path');
const { formatUnits } = require('ethers/lib/utils');
const fs = require('fs');
const { fail } = require('assert');

// 2. Define network configurations
const providerRPC = {
    moonbeam: {
        name: 'moonbeam',
        rpc: 'https://moonbeam.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 1284, // 0x504 in hex,
    },
};

// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.moonbeam.rpc,
    {
        chainId: providerRPC.moonbeam.chainId,
        name: providerRPC.moonbeam.name,
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

// https://api-moonriver.moonscan.io/api?module=account&action=tokenbalance&contractaddress=0x98878B06940aE243284CA214f92Bb71a2b032B8A&address=0xAe8Da4A9792503f1eC97eD035e35133A9E65a61f&tag=latest&apikey=ZZKXM87BB86Y81NWM31ERC66NZPXPNFS95
// https://api-moonriver.moonscan.io/api?module=account&action=tokenbalance&contractaddress=0x98878B06940aE243284CA214f92Bb71a2b032B8A&address=&tag=latest&apikey=ZZKXM87BB86Y81NWM31ERC66NZPXPNFS95

// ZZKXM87BB86Y81NWM31ERC66NZPXPNFS95
const wGlmr = "0xAcc15dC74880C9944775448304B263D191c6077F";
const myAddress = "0xAe8Da4A9792503f1eC97eD035e35133A9E65a61f";
// https://api.covalenthq.com/v1/1284/tokens/0xAcc15dC74880C9944775448304B263D191c6077F/token_holders/?&key=ckey_7582964824a047a1985ba46d29a

// MANUALLY GET GLMR TOKEN HOLDERS

// 1) GET ALL ASSETS THRU GLMR TRADING PAIR
// 1a) Start with glmr holders
// 1b) Find dex contracts, get token pair
//
// 2) Get token pair holders
// 2a) Use COVALENT api for token holders
// 2b) manually get the rest of token holders

// 3) GET LIQ POOLS FOR ALL ASSETS
//
// 4) Build LIQPOOL registry and TOKEN registry
// 

//MANUALLY SAVE GLMR TOKEN HOLDERS AND PARSE THE TEXT
async function getGlmrLiqPools() {
    let glmrHolders = fileReader.readAddressFile('wglmr.txt');
    console.log(glmrHolders)

    let glmrLps = [];
    for (i = 0; i < glmrHolders.length; i++){
        const is_dex = await isDex(glmrHolders[i])
        if (is_dex) {
            glmrLps.push(glmrHolders[i]);
            console.log("yes")
        } else {
            console.log("no")
        }
    }
    console.log(glmrLps)
    console.log(glmrLps.length)
    fs.writeFileSync('wglmr_liq_pool_addresses.txt', glmrLps.toString(), 'utf8')
}

//GET ALL ASSETS THROUGH QUERYING GLMR LP POOLS
async function getGlmrTradingPairs() {
    let data = fs.readFileSync('wglmr_liq_pool_addresses.txt', 'utf8');
    data = data.split(',')

    let allAssets = [];
    for (i = 0; i < data.length; i++){
        try {
            let dexContract = new ethers.Contract(data[i], dexContractAbi, provider)
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
                console.log("COULD NOT CREATE DEX CONTRACT")
                console.log(err)
            }
        }
    }
    console.log(allAssets)
    console.log(allAssets.length)

    fs.writeFileSync('wglmr_trading_pairs.txt', allAssets.toString(), 'utf8')
}

// TOKEN HOLDERS FROM GLMR TRADING PAIR ASSETS
async function getAllTokenHolders() {
    let data = fs.readFileSync('wglmr_trading_pairs.txt', 'utf8');
    data = data.split(',');

    let successArray = [];
    let failArray = [];
    let allTokenHolders = [];
    for (i = 0; i < data.length; i++){
        const uri = 'https://api.covalenthq.com/v1/1284/tokens/' + data[i] + '/token_holders/?key=ckey_7582964824a047a1985ba46d29a';
        try {
            const res = await fetch(uri)
            const answer = await res.json()
            console.log(answer)
            console.log("SUCCESS: QUERY NEW TOKEN")
            successArray.push(data[i])
            answer.data.items.map((item) => {
                // console.log(item.address)
                allTokenHolders.push(item.address)
            })
        } catch (err) {
            console.log("FAIL")
            failArray.push(data[i]);
        }
    }

    let manualTokenHolders = getManual();
    allTokenHolders.push.apply(allTokenHolders, manualTokenHolders);

    fs.writeFileSync("all_token_holders.txt", allTokenHolders.toString(), 'utf8')
}

// Using list of all token holders, filter out dex contracts
async function getAllLiquidityPools() {
    let data = fs.readFileSync('all_token_holders.txt', 'utf8');
    data = data.split(',')

    let allPools = [];
    for (i = 0; i < data.length; i++){
        let is_dex = await isDex(data[i]);
        if (is_dex) {
            allPools.push(data[i]);
        }
    }
    console.log(allPools)
    console.log(allPools.length)
    fs.writeFileSync('all_liq_pools.txt', allPools.toString(), 'utf8')
}

//Get all liquidity pools and save them as LiqPool and Token objects
async function buildRegistry() {
    let data = fs.readFileSync('all_liq_pools.txt', 'utf8');
    data = data.split(',');
    console.log(data.length)
    let liqPools = [];
    let tokenAddresses = [];
    let tokens = [];
    for (i = 0; i < data.length; i++){
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

    for (i = 0; i < tokens.length; i++){
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


// https://api.covalenthq.com/v1/1284/xy=k/stellaswap/pools/?&key=ckey_7582964824a047a1985ba46d29a

// https://api.covalenthq.com/v1/1284/address/:address/transactions_v2/?&key=ckey_7582964824a047a1985ba46d29a

async function getGlmrTxs() {
    let uri = "https://api.covalenthq.com/v1/1284/address/0xAcc15dC74880C9944775448304B263D191c6077F/transactions_v2/?&key=ckey_7582964824a047a1985ba46d29a"
    const res = await fetch(uri)
    const answer = await res.json()
    console.log(answer)
    // console.log("SUCCESS: QUERY NEW TOKEN")
    // successArray.push(data[i])
}

async function ethersTest(){
    let contract = new ethers.Contract(wGlmr, tokenContractAbi, provider);

    let results = contract.filters.Transfer(null);
    console.log(results);
}
async function main() {
    // getGlmrLiqPools()
    // getGlmrTradingPairs()
    // getGlmrTxs();
    // ethersTest()
    // getManual()
    // getAllTokenHolders()
    // getAllLiquidityPools()
    // buildRegistry()
    getRegistry()
}

// BUILD DATABASE LOGIC
// main(){
//     1{
//     getGlmrLiqPools();
//     getGlmrTradingPairs();
//     }
//     2) getAllTokenHolders()
//     
//     3) getAllLiquidityPools()
// 
//     4) buildRegistry()
// }

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

async function getToken(address) {
    let tokenContract = await ethers.Contract(address, tokenContractAbi, provider);
    // let name = tokenContract.name();
    let token = new Token(address, tokenContract.symbol(), tokenContract.name(), tokenContract.decimals())
    return token;
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