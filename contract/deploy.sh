#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --accountId depository.andrii_yer.testnet --wasmFile ./target/wasm32-unknown-unknown/release/depository.wasm
# --initFunction new --initArgs '{"deposit_account": "balance.andrii_yer.testne"}'
