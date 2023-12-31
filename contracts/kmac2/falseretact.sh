soroban contract invoke   --id $(cat .soroban/kmac2-id) \
      	--source  kreator   \
	--network testnet  \
       	--   kmac    \
	--user GBB34D6XSHK6TSCE62Z43SWJ4RO22GHN5XKBASMPEXLDDF5QFOHE4TW2         \
	--value 2    \
       	--message "retactiv" \
       --buyer "GBB34D6XSHK6TSCE62Z43SWJ4RO22GHN5XKBASMPEXLDDF5QFOHE4TW2"    \
       	--sender  "kreator"
##false retactiv: the buyer try to send tis trx that is an tyoe of "admin flow".
### IT IS SUPPOSED that the person "buyer" doesnt know the key () of the kreator...
### if this is public key ... then this not functions as security----
###GDLTFWZTH3JXPJX4LHRJU5L7WDBZ2CQKTRA3Z57HUKP25H35ICQZRMFH
###GBB34D6XSHK6TSCE62Z43SWJ4RO22GHN5XKBASMPEXLDDF5QFOHE4TW2

