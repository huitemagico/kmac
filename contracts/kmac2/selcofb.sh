soroban contract invoke   --id $(cat .soroban/kmac2-id) \
      	--source  alice   \
	--network testnet  \
       	--   kmac    \
	--user GB7PDQICHJOSTEVMIQJJIQVZ762WKPYGG354DI34DBB67DJYKQQ4YNQD         \
	--value 2    \
       	--message "selcofblnd" \
       	--buyer GB7PDQICHJOSTEVMIQJJIQVZ762WKPYGG354DI34DBB67DJYKQQ4YNQD    \
       	--sender  "buyer"
