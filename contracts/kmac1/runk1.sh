soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
     --user GC6NIFTZVU3NFMAXAT5C4ZHSOAVIFZDL7YMWSPOVZUO54SFR5IMMBEHJ     \
    --value 2 \
    --message "savekreatoraddress" \
    --sender  "kreator"


