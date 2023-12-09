soroban contract invoke \
  --id $(cat .soroban/holita-id) \
  --source alice \
  --network HUITENET \
  -- \
  hello \
  --to RPC
