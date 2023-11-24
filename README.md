# kmac
# About this doc 
README.md Version November 24, 2023 14:48<br />
This is README.md file of the kmac project, at the main branch at https://github.com/huitemagico/kmac

 *IMPORTANT: *
Please see the updated and detailed documentation in detail at following links:

| Content| Link |
| --- | --- |
| Proposal of the idea (Dashboard SCF)| [KMAC at SCF](https://dashboard.communityfund.stellar.org/scfawards/scf-20/panelreview/suggestion/103) |
| KMAC Documentation main page| [KMAC wiki main page](https://github.com/huitemagico/kmac/wiki) |
| Deliverables main chapter| [Deliverables main chapter ](https://github.com/huitemagico/kmac/wiki#deliverables) |
| Documentation of Development Stage-1 (Deliverable 1) | [Documentation of Deliverable-1](https://github.com/huitemagico/kmac/wiki/KMAC-Deliverable-1) |
| How Setup the KMAC environment  | [KMAC Setup](https://github.com/huitemagico/kmac/wiki/KMAC-Setup) |
| User Manual of KMAC| [KMAC User Manual](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual) |
| kmac2 | [Stage 2 -Deliverable-2] (https://github.com/huitemagico/kmac/wiki/kmac-Stage-2-%E2%80%90-Deliverable-2-Documentation) |



# What is KMAC?
`@kmac` is a contract build with SOROBAN-SDK that makes an  implementation of an Finite State Machine template. <br />
[GitHub url](https://github.com/huitemagico/kmac)

![Kmac architecture](pictures/kmac03.vpd.png)

 
# How clone, run and test 

## Soroban version
The setup use soroban version: 
```bash
$ soroban -V
soroban 20.0.0-rc2 (1c49aed3badc374d46b340b5212a745703c524b2)
```
 
Important: These instructions are for using the 'kmac2' version on the Testnet updated as of November 22, 2023. Keep in mind that the instructions are for an unstable version and need to be updated accordingly :https://soroban.stellar.org/docs/getting-started/deploy-to-testnet

## (1) Environment setup 

 Please refer to official soroban setup instructions: https://soroban.stellar.org/docs/getting-started/setup
 The RUSTC version should be rustc 1.72 or newer.

## (2) git clone from github 
Version of "Stage-1" (Deliverable-1") at https://github.com/huitemagico/kmac/tree/main/contracts/kmac1
Version (in progress) of "Stage-2" at https://github.com/huitemagico/kmac/tree/main/contracts/kmac2

## cargo, build and run.

Once the program is downloaded on the local disk, see the steps below:

(1)
 ```bash
cd kmac2

cargo test -- --nocapture
 ```
Note: the --nocapture  explanation:
 [Display Options](https://doc.rust-lang.org/cargo/commands/cargo-test.html#display-options)
 By default the Rust test harness hides output from test execution to keep results readable. Test output can be recovered (e.g., for debugging) by passing --nocapture to the test binaries:

(2) soroban contract build --profile release-with-logs

(3) identities setup:

We need to set up some identities to use for testing and get their public keys: <br />
For this, run the following shell commands:
```bash
soroban config identity generate kreator && \

soroban config identity generate buyer && \

soroban config identity address kreator && \

soroban config identity address buyer
```

(4) The output of the precedent instruction, gives the 'kreator address'.
 

(5) To view the stored values:
Use readcont.sh 

 
(6) Overview of the steps

(I)
Configuring the CLI for Testnet
 https://soroban.stellar.org/docs/getting-started/setup

Configuring the net AIVNET:
   soroban config network add  AIVNET   --rpc-url https://soroban-testnet.stellar.org:443   
   --network-passphrase "Test SDF Network ; September 2015"

 (II)
cargo test -- --nocapture

soroban contract build --profile release-with-logs

soroban contract deploy   --wasm target/wasm32-unknown-unknown/release-with-logs/kmac2.wasm     --source alice   --network AIVNET

This command produce an address
This address insert here:
echo "CAVGBSIM2EVCQJKPZMMF4X5PFQZ6MDIZCJBC4AKN2VK335H4ZW4K3THT" > .soroban/kmac2-id

(III) Invoke
soroban contract invoke   --id $(cat .soroban/kmac2-id) \
        --source  alice   \
  --network AIVNET  \
        --   kmac    \
  --user GB4QRIXKO4K3QTQFPMFLJ7MSIDIWCBOYSK6C7Q43KJZZ7BX5IB4H74LF         \
  --value 2    \
        --message "cldrst"    \
        --buyer "GBGC5LMJOTEYRHND7AY3GMDNTQPHJ22WMUMWKVBD7D746MLAN3OGVXRP"    \
        --sender  "kreator"


For details of the transactions, 
[please see the chapter "Stage 1- Deliverable-1"] (https://github.com/huitemagico/kmac/wiki/KMAC-Deliverable-1)
[please see the chapter "Stage 2 -Deliverable-2"] (https://github.com/huitemagico/kmac/wiki/kmac-Stage-2-%E2%80%90-Deliverable-2-Documentation)
For details of the setup process, [please see the chapter "Setup"](https://github.com/huitemagico/kmac/wiki/KMAC-Setup)

(7) More on playing with the program
Please see "user manual"
https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual

