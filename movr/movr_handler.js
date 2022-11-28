// 1. Import ethers
const ethers = require('ethers');
const { parse } = require('path');
const { formatUnits } = require('ethers/lib/utils');

// 2. Define network configurations
const providerRPC = {
    moonriver: {
        name: 'moonriver',
        rpc: 'https://moonriver.api.onfinality.io/public', // Insert your RPC URL here
        chainId: 1285, // 0x505 in hex,
    },
};
// 3. Create ethers provider
const provider = new ethers.providers.StaticJsonRpcProvider(
    providerRPC.moonriver.rpc,
    {
        chainId: providerRPC.moonriver.chainId,
        name: providerRPC.moonriver.name,
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

async function getMovrTokenHoldersBlockscout() {
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

    fs.writeFileSync('wmovr.txt', tokenHolders.toString(), 'utf8');

    let tokenUris = [];
    // let tokenPromises =[]
    for (i = 1; i < 20; i++){
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        tokenUris.push(uri);
        // const res = await fetch(uri)
        // const answer = await res.json()
        // console.log(answer.result.length)
    }
    let tokenRequests = tokenUris.map(uri => {
        return fetch(uri).then((val) => val)
    })
}

async function getPromises() {
    let promises = await testPromises();
    // for (i = 0; i < promises.length; i++){
    //     console.log(promises[i].json)
    // }
    let jsonPromises = [];
    for (i = 0; i < promises.length; i++){
        jsonPromises.push(promises[i].json())
    }

    let response = Promise.all(jsonPromises).then(val => {
        return val
    })
    console.log(response[0])
}

async function testPromises() {
    let tokenUris = [];
    // let tokenPromises =[]
    for (i = 1; i < 3; i++) {
        let page = i;
        let offset = 50;
        let uri = `https://blockscout.com/astar/api?module=token&action=getTokenHolders&contractaddress=0xAeaaf0e2c81Af264101B9129C00F4440cCF0F720&page=${page}&offset=${offset}`;
        tokenUris.push(uri);
        // const res = await fetch(uri)
        // const answer = await res.json()
        // console.log(answer.result.length)
    }
    let tokenRequests = tokenUris.map(uri => {
        console.log("fetching uri: " + uri)
        let response = fetch(uri);
        // return fetch(uri).then(() => true, () => false)
        return response.then(val => val, () => false)
    });

    // return Promise.all(tokenRequests).then(result => {
    //     return tokenUris.filter((_, i) => result[i])
    // })
    console.log(tokenRequests)
    return Promise.all(tokenRequests).then(val => {
        // let responses = [];
        // for (i = 0; i < val.length; i++){
        //     console.log(val[i].json)
        //     responses.push(val[i].json())
        // }
        // console.log(responses)
        // return responses
        // return val[0].json()
        return val
    })

    
}

async function main(){
    getPromises();
    // const wsProvider = new WsProvider('wss://moonriver.api.onfinality.io/public-ws');
    // const api = await ApiPromise.create({ provider: wsProvider });

    // const addr = '0xAe8Da4A9792503f1eC97eD035e35133A9E65a61f';

    // const now = await api.query.timestamp.now();
    // const { nonce, data: balance } = await api.query.system.account(addr);
    // const nextNonce = await api.rpc.system.accountNextIndex(addr);
    // console.log("HELP")
    // console.log(`${now}: balance of ${balance.free} and a current nonce of ${nonce} and next nonce of ${nextNonce}`);
}

main()