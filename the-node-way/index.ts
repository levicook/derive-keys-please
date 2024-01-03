#!/usr/bin/env node

import { Keypair } from "@solana/web3.js"
import { mnemonicToSeedSync } from "bip39"
import { HDKey } from "micro-ed25519-hdkey"

const DEFAULT_SEED_PHRASE = "method bronze music hero response market impact sound bone magic unfair salad"
const DEFAULT_PASS_PHRASE = ""

const seed_phrase = process.env.TEST_SEED_PHRASE ?? DEFAULT_SEED_PHRASE
const pass_phrase = process.env.TEST_PASS_PHRASE ?? DEFAULT_PASS_PHRASE

const seed = mnemonicToSeedSync(seed_phrase, pass_phrase)
const hd = HDKey.fromMasterSeed(seed.toString("hex"));

let paths: string[] = [];
paths.push("m/44'/501'");
for (let i = 0; i < 10; i++) {
    paths.push(`m/44'/501'/${i}'`);
    paths.push(`m/44'/501'/${i}'/0'`);
    for (let j = 0; j < 10; j++) {
        paths.push(`m/44'/501'/${i}'/0'/${j}'`);
    }
}

let pairs: [string, string][] = paths.map(path => {
    let keypair = Keypair.fromSeed(hd.derive(path).privateKey)
    return [path, keypair.publicKey.toBase58()]
})


console.log(`${"derivation path".padEnd(20)} ${"pubkey"}`);
for (let [path, pubkey] of pairs) {
    console.log(`${path.padEnd(20)} ${pubkey}`);
}
