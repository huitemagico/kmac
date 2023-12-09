soroban contract invoke \
  --id $(cat .soroban/holita-id) \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC 
