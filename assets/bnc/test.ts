import { ApiPromise, WsProvider } from '@polkadot/api';
import { options } from '@bifrost-finance/api'

async function test() {
    const provider = new WsProvider('wss://bifrost-parachain.api.onfinality.io/public-ws')
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

    const currencyId = api.createType('CurrencyId');
    console.log(currencyId)
}

async function main() {
    test()
}

main()