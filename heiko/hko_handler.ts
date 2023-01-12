import { options } from '@parallel-finance/api';
import { ApiPromise } from '@polkadot/api';
import { WsProvider } from '@polkadot/rpc-provider';

async function getHeikoAssets() {
    
}

async function main() {
    const provider = new WsProvider('wss://heiko-rpc.parallel.fi');
    const api = new ApiPromise(options({ provider }));
    await api.isReady;

}

main().then(() => console.log("heiko complete"))