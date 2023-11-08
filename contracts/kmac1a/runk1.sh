soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
     --user GB4QRIXKO4K3QTQFPMFLJ7MSIDIWCBOYSK6C7Q43KJZZ7BX5IB4H74LF     \
    --value 2 \
    --message "savekad" \
    --sender  "kreator"


