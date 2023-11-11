####cold reset scldrst
soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/kmac1b.wasm \
    --id 1 \
    -- \
    kmac\
     --user GB4QRIXKO4K3QTQFPMFLJ7MSIDIWCBOYSK6C7Q43KJZZ7BX5IB4H74LF     \
    --value 2 \
    --message "cldrst" \
    --buyer "GBGC5LMJOTEYRHND7AY3GMDNTQPHJ22WMUMWKVBD7D746MLAN3OGVXRP" \
    --sender  "kreator"
