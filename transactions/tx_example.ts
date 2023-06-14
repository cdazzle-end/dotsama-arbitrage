import * as fs from 'fs';
import { Keyring, ApiPromise, WsProvider,  } from '@polkadot/api';
import { cryptoWaitReady, mnemonicGenerate, signatureVerify } from '@polkadot/util-crypto';
import { stringToU8a, u8aToHex } from '@polkadot/util';

const dazzleTestSeed = 'unhappy valid capital floor fruit exclude normal latin coil robust strategy void';

async function createWallet() {
    let addy_json = JSON.parse(fs.readFileSync('../transactions/test_add.json', 'utf8'));
    const provider = new WsProvider('wss://kusama-rpc.polkadot.io');
    const api = await ApiPromise.create({ provider: provider });
    await api.isReady


    // const keyring = await new Keyring({ type: 'sr25519' });
    const keyring = new Keyring();
    // let testAddy = await keyring.addFromJson(addy_json);
    

    // for (const key of keyring.getPairs()) {
    //     console.log(key.address)
    // }
    
    // const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
    // // alice
    // console.log(`${alice.meta.name}: has address ${alice.address} with publicKey [${alice.publicKey}]`);

    const mnemonic = mnemonicGenerate();
    // create & add the pair to the keyring with the type and some additional
    // metadata specified
    const pair = keyring.addFromUri(mnemonic, { name: 'first pair' }, 'ed25519');

    // the pair has been added to our keyring
    console.log(keyring.pairs.length, 'pairs available');

    // log the name & address (the latter encoded with the ss58Format)
    console.log(pair.meta.name, 'has address', pair.address);

    // create an ed25519 pair from the mnemonic
    const ep = keyring.createFromUri(mnemonic, { name: 'ed25519' }, 'ed25519');

    // create an sr25519 pair from the mnemonic (keyring defaults)
    const sp = keyring.createFromUri(mnemonic, { name: 'sr25519' });

    // log the addresses, different cryptos, different results
    console.log(ep.meta.name, ep.address);
    console.log(sp.meta.name, sp.address);

    //FORMATTING
    // ...
    // known mnemonic, well, now it is - don't use it for funds
    const MNEMONIC = 'sample split bamboo west visual approve brain fox arch impact relief smile';

    // type: ed25519, ssFormat: 42 (all defaults)
    // const keyring = new Keyring();
    const pair2 = keyring.createFromUri(MNEMONIC);

    // use the default as setup on init
    // 5CSbZ7wG456oty4WoiX6a1J88VUbrCXLhrKVJ9q95BsYH4TZ
    console.log('Substrate generic', pair2.address);

    // adjust the default ss58Format for Kusama
    // CxDDSH8gS7jecsxaRL9Txf8H5kqesLXAEAEgp76Yz632J9M
    keyring.setSS58Format(2);
    console.log('Kusama', pair2.address);

    // adjust the default ss58Format for Polkadot
    // 1NthTCKurNHLW52mMa6iA8Gz7UFYW5UnM3yTSpVdGu4Th7h
    keyring.setSS58Format(0);
    console.log('Polkadot', pair2.address);

    // 16,178,46,190,137,179,33,55,11,238,141,57,213,197,212,17,218,241,232,252,145,201,209,83,64,68,89,15,31,150,110,188
    console.log(pair2.publicKey);

    // DECODING Formatted Address -> Public Key
    // 16,178,46,190,137,179,33,55,11,238,141,57,213,197,212,17,218,241,232,252,145,201,209,83,64,68,89,15,31,150,110,188
    console.log(keyring.decodeAddress('5CSbZ7wG456oty4WoiX6a1J88VUbrCXLhrKVJ9q95BsYH4TZ'));

    // 16,178,46,190,137,179,33,55,11,238,141,57,213,197,212,17,218,241,232,252,145,201,209,83,64,68,89,15,31,150,110,188
    console.log(keyring.decodeAddress('CxDDSH8gS7jecsxaRL9Txf8H5kqesLXAEAEgp76Yz632J9M'));

    // 16,178,46,190,137,179,33,55,11,238,141,57,213,197,212,17,218,241,232,252,145,201,209,83,64,68,89,15,31,150,110,188
    console.log(keyring.decodeAddress('1NthTCKurNHLW52mMa6iA8Gz7UFYW5UnM3yTSpVdGu4Th7h'));

    // ENCODING Public Key -> Formatted Address
    // 5CSbZ7wG456oty4WoiX6a1J88VUbrCXLhrKVJ9q95BsYH4TZ
    console.log(keyring.encodeAddress(pair2.publicKey, 42));

    // CxDDSH8gS7jecsxaRL9Txf8H5kqesLXAEAEgp76Yz632J9M
    console.log(keyring.encodeAddress(pair2.publicKey, 2));

    // 1NthTCKurNHLW52mMa6iA8Gz7UFYW5UnM3yTSpVdGu4Th7h
    console.log(keyring.encodeAddress(pair2.publicKey, 0));

    // SIGN AND VERIFY
    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');

    // create the message, actual signature and verify
    const message = stringToU8a('this is our message');
    const signature = alice.sign(message);

    // Verify our own message
    const isValidOwn = alice.verify(message, signature, alice.publicKey);

    // output the result
    console.log(`${u8aToHex(signature)} is ${isValidOwn ? 'valid' : 'invalid'}`);

    // Verify using only address, this is how others verify your message
    // verify the message using Alice's address
    const { isValid } = signatureVerify(message, signature, alice.address);

    // output the result
    console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);



    api.disconnect();
}

async function main() {
    createWallet()
}

main()