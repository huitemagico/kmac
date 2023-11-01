# kmac
KMAC is a program build with SOROBAN-SDK that makes an  implementation of an Finite State Machine template,  using knowed design pattern and providing extensible functions capabilities. <br />
## inspiring codes from:
  https://soroban.stellar.org/docs/basic-tutorials/auth

## news
---------------------<br />
01/10/2023 note BEGIN <br />
important  <br />
--------------------- <br />
new folder contracts <br />
contracts/READ.ME <br />
Here will come every component of kmac. <br />
The directory ../README.md contains log of the updates. <br />
## diagram
+-------+<br />
| kmac  |<br />
+-------+<br />
    |<br />
    |<br />
    +------------contracts<br />
                 |<br />
                 |<br />
                 +------src<br />
01/10/2023 note  END<br />

## 01/10/2023 technical notes.<br />
Version 01/10/2023 notes<br />
-contains a first coding for auth function.<br />
-accept 2 first parameters auth style<br />


 ## fast setup
  Here comes description how setup and run<br />
 1) cargo test <br />
 2) soroban contract build<br /> 
 3) identities <br />
We need to set up some identities to use for testing and get their public keys: <br />

soroban config identity generate acc1 && \ <br />
soroban config identity generate acc2 && \ <br />
soroban config identity address acc1 && \ <br />
soroban config identity address acc2 <br />

 You must copy the string obtained and then insert in the run0.sh script <br />
 (see The shells paragraph) <br />

soroban contract invoke \ <br />
    --source acc1 \ <br />
    --wasm target/wasm32-unknown-unknown/release-with-logs/echo2.wasm \ <br />
    --id 1 \ <br />
    -- \ <br />
    echo2\ <br />
     --user GCK3W6M5Z3W3JIHJ7M4MTKG4TH2TRMZRBFFKN3TTF6VFTFQYAN5RXPIM     \ <br />
    --value 2 \ <br />
    --message "reset" \ <br />
    --trx "ab" <br />

## The shells
-shells: are basic development and test utility shell script.<br />
 compi.sh for compiling<br />
 bld.sh   for build<br />
 run0.sh  for run<br />
 (you must edit run0.sh with the identities obtained in the precedent paragraph)

 

