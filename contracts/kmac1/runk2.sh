soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
     --user GAHRKXCQ345H5SXJXHROXND54CT5DJXHHPBAKRPTOAGV5GZN4VXQJI3O     \
    --value 2 \
    --message "savekreatoraddress" \
    --sender  "kreator"


