soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \
    --id 1 \
    -- \
    echo2\
     --user GDI6Q5C2ROTGSHULML5JFNAY6YQCLLSC3673T3M7GKLGSYDZSNDZQHXY     \
    --value 2 \
    --message "savekreatoraddress" \
    --sender  "kreator"


