soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
     --user GBJ6XLEJVAWOFON3TDPIUOMJ4JI7OJ7EGRFNJEUJ2WHQFHZWKSLAMIVH     \
    --value 2 \
    --message "savekreatoraddress" \
    --sender  "kreator"


