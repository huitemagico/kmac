soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/kmac1b.wasm \
    --id 1 \
    -- \
    kmac\
     --user GBSWHN23BZS5PGFMTYOZJ5QE77X7E7B4OMENPXKFD3BY6CUOTAW5BMA3     \
    --value 2 \
    --message "savekad" \
    --buyer "GBGC5LMJOTEYRHND7AY3GMDNTQPHJ22WMUMWKVBD7D746MLAN3OGVXRP" \
    --sender  "kreator"
