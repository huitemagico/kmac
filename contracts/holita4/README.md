README.md of holita*
Rev December 09, 2023 17:14
https://github.com/huitemagico/kmac/tree/main/contracts/holita4

## This folder contains an zip file of a working "hello world" contract.


## What is inside the folder:
This is a test trying to turn around a problem that I have for deploy 
an contract.
See https://discord.com/channels/897514728459468821/1182199504024375296

## What means that problem.
I dont know which is the origin of the problem. See the discord above.


If you have experienced the error explained at the discord above,  when you have tried to deploy an contract, 
you could try the following:

##Steps:
0. you have to configure identity and network (better is use "global" option).
 For this please see

1. download gzip
2. move zipped to an test folder, for instance $HOME/hola
2. unzipped the file ---> gunzip holita4.gz
3. extract the content ---> tar xvf holita
4. (now you could delete the .gz file)
5. Standing on $HOME/hola, run the test  ---->cargo test
6. build  ---> soroban contract build --profile release-with-logs
7. deploy -->
soroban contract deploy   --wasm target/wasm32-unknown-unknown/release-with-logs/holita.wasm    --source alice   --network testnet


If the deploy is ok, then you follow the steps indicated at wiki page above for storing the address.

Run the contract:

 soroban contract invoke \
  --id $(cat .soroban/holita-id) \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC \


  If running is ok, the you have an development environment.
  For that you could  cp -r to another folder and replace it for your
  lib.rs and test.rs
  

THIS IS A TEST. USE IT UNDER YOUR OWN RESPONSIBILITY. BEFORE USING THIS YOU HAVE BACK UP YOUR FILES.
THE OFFICIAL SOROBAN PAGE IS AT https://soroban.stellar.org/docs/
