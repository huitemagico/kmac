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

 
# Deliverables

## Deliverable 1: Proof of Concept (POC) for Design Pattern Implementation

Brief Description: Design, coding, and documentation of functional code components using soroban-sdk.   <br />
This includes the following features:<br />

Security – Allow List Implementation & Role-Based Access Control <br />
Modularity – Implementation of internal and external functions (basic structure) <br />
Incorporating parametric components with declarative functionality to enable the use of extensible functions with component reuse. <br />
Testing of the previous modules <br />
How to measure completion: <br />
Code and tests available in https://github.com/huitemagico/kmac <br />
Announcement in Stellar Development Discord <br />
Documentation published in dev.to and on KMAC github repository <br />
Estimated date of completion: November 10 2023 (2 weeks from october 27, 2023) <br />

## Deliverable 2: Functional KMAC Program for Vending Machine Example (Sandbox) <br />
Brief Description: A soroban-sdk smart contract with hard-coded parameters that define states and transactions for the vending machine example. <br />
 The program is designed to accept transactions from a test module and respond with the new state. 
 Exception conditions are not handled in this version.<br />
How to Measure Completion:<br />
Code and tests available in https://github.com/huitemagico/kmac <br />
Announcement in Stellar Development Discord <br />
Documentation published in dev.to and on KMAC github repository <br />
Estimated Date of Completion: December 01 2023 (+3 weeks) <br />


## Deliverable 3: Documentation Manual for KMAC Usage
Brief Description: This document serves as a comprehensive manual that provides detailed instructions on working with the KMAC template. It <br />includes information on the various modules, setting new parameters for building different FSMs, reusing design pattern components, and extending <br />functionality. The document also discusses storage alternatives and considerations for network and stand-alone environments.<br />
How to Measure Completion: This deliverable will be posted in the GitHub repository of the KMAC project and announced in the Stellar discussion <br />chat.
Estimated Date of Completion: December 8 2023 (+1 week)<br />

