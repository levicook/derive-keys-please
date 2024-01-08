#!/usr/bin/env node
import { program } from "commander"

import { Keypair } from "@solana/web3.js"
import { mnemonicToSeedSync } from "bip39"
import { HDKey } from "micro-ed25519-hdkey"

const DEFAULT_URL = 'https://api.mainnet-beta.solana.com'

program.name("the-node-way")
    .command('derive-keys-please').action(deriveKeysPlease)

program
    .command('list-assets-please')
    .argument('<ownerAddress>', 'owner address to list assets for')
    .option('-u --url <url>', 'rpc url')
    .action(
        async (ownerAddress: String, options: { url?: string }) => {
            let url = options.url ?? DEFAULT_URL;
            await listAssetsPlease(ownerAddress, url);
        });

program.parse(process.argv);

function deriveKeysPlease() {
    const DEFAULT_SEED_PHRASE = "method bronze music hero response market impact sound bone magic unfair salad"
    const DEFAULT_PASS_PHRASE = ""

    const seed_phrase = process.env.TEST_SEED_PHRASE ?? DEFAULT_SEED_PHRASE
    const pass_phrase = process.env.TEST_PASS_PHRASE ?? DEFAULT_PASS_PHRASE

    const seed = mnemonicToSeedSync(seed_phrase, pass_phrase)
    const hd = HDKey.fromMasterSeed(seed.toString("hex"));
    // TODO root keypair

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


    console.log(`${"path".padEnd(20)} ${"pubkey"}`);
    // TODO print root keypair
    for (let [path, pubkey] of pairs) {
        console.log(`${path.padEnd(20)} ${pubkey}`);
    }
}

async function listAssetsPlease(ownerAddress: String, url: String) {
    const assets = await getAssetsByOwner(ownerAddress, url)
    console.log(JSON.stringify(assets, null, '  '))
}

async function getAssetsByOwner(ownerAddress: String, url: any) {
    let page = 1;
    let hasMore = true;
    const allAssets: string[] = [];

    while (hasMore) {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: 'my-id',
                method: 'getAssetsByOwner',
                params: {
                    ownerAddress: ownerAddress,
                    page: page,
                    limit: 1000,
                },
            }),
        });

        const { result } = await response.json();

        allAssets.push(...result.items);

        if (result.items.length < 1000) {
            hasMore = false;
        } else {
            page++;
        }
    }

    return allAssets
}