soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/kmac1b.wasm \
    --id 1 \
    -- \
    kmac\
     --user GA2YUQ7TWIAA6SBUP6TRN5NVQXCP2VMBJY3FYX4NSWS4PGTEBNOJNE5L     \
    --value 2 \
    --message "svb1adr" \
    --buyer "GBGC5LMJOTEYRHND7AY3GMDNTQPHJ22WMUMWKVBD7D746MLAN3OGVXRP" \
    --sender  "kreator"


