near deploy \
    --wasmFile out/main.wasm \
    --initFunction "new" \
    --initArgs '{
        "owner_id": "rental-service.manhng.testnet"
    }' \
    --accountId rental-service.manhng.testnet
