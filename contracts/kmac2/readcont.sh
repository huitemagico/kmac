sleep 2
echo "Read key COUNTER network testnet"
echo "Id en .soroban="
cat .soroban/kmac2-id
ls -al .soroban/*
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key COUNTER  --network testnet
sleep 2
echo "REad key B1STAD "
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key B1STAD  --network  testnet
sleep 2
echo "Read key MCSTAT"
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key MCSTAT  --network  testnet
sleep 4
echo "Read key KSTADR"
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key KSTADR  --network  testnet
echo "The sequence is:"
echo "cldrst.sh --> cold reset, state ----->A"
echo "rstkadm.sh ---> reset adm key, state ---->B"
echo "svb1adr.sh ---> save buyer address, state ---->C"
echo "selcofb.sh ---> select coffee blend, state -->E"
echo "buypay.sh ---> buyer paymente, state ---> F"
echo "retact.sh ---> return to active, state ---->C"
