#soroban contract read --id 1 --key KSTADR
#soroban contract read --id 1 --key COUNTER
#soroban contract read --id 1 --key B1STAD
#soroban contract read --id 1 --key MCSTAT
sleep 2
echo "Read key COUNTER network AIVNET"
echo "Id en .soroban="
cat .soroban/kmac2-id
ls -al .soroban/*
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key COUNTER  --network AIVNET
sleep 2
echo "REad key B1STAD "
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key B1STAD  --network AIVNET
sleep 2
echo "Read key MCSTAT"
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key MCSTAT  --network AIVNET
sleep 2
echo "Read key KSTADR"
soroban contract read --durability persistent --id $(cat .soroban/kmac2-id) --key KSTADR  --network AIVNET
echo "La secuencia es:"
echo "cldrst.sh --> estado A MCSTAT"
echo "rstkadm.sh ---> estado B MCSTAT"
echo "svb1adr.sh ---> estado C MCSTAT key B1STAD"

