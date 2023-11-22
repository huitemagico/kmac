# kmac
# About this doc 
README.md Version November 22, 2023 12:40<br />
This is README.md file of the kmac project, at the main branch at https://github.com/huitemagico/kmac

 *IMPORTANT: This is an overview of the project!*
Please see the updated and detailed documentation in detail at following links:

| Content| Link |
| --- | --- |
| Proposal of the idea (Dashboard SCF)| [KMAC at SCF](https://dashboard.communityfund.stellar.org/scfawards/scf-20/panelreview/suggestion/103) |
| KMAC Documentation main page| [KMAC wiki main page](https://github.com/huitemagico/kmac/wiki) |
| Deliverables main chapter| [Deliverables main chapter ](https://github.com/huitemagico/kmac/wiki#deliverables) |
| Documentation of Development Stage-1 (Deliverable 1) | [Documentation of Deliverable-1](https://github.com/huitemagico/kmac/wiki/KMAC-Deliverable-1) |
| How Setup the KMAC environment  | [KMAC Setup](https://github.com/huitemagico/kmac/wiki/KMAC-Setup) |
| User Manual of KMAC| [KMAC User Manual](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual) |




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
Version (unstable, in progress) of "Stage-2" at https://github.com/huitemagico/kmac/tree/main/contracts/kmac2

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

(6) Understanding the process
At this step, the contract is prepared for receive the transactions included at the "Stage-1" (Deliverables-1).

For details of the transactions, [please see the chapter "Deliverable-1"](https://github.com/huitemagico/kmac/wiki/KMAC-Deliverable-1)
For details of the setup process, [please see the chapter "Setup"](https://github.com/huitemagico/kmac/wiki/KMAC-Setup)

(8) More on playing with the program
Please see "user manual"
https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual

