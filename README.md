# kmac
# About this doc 
README.md Version November 11, 2023 13:20<br />
This is README.md file of the kmac project, at the main branch at https://github.com/huitemagico/kmac

 *IMPORTANT: This is an overview of the project!*
Please see the updated and detailed documentation in detail at following links:


| Content| Link |
| --- | --- |
| KMAC wiki main page| [wiki main page](https://github.com/huitemagico/kmac/wiki) |
| Deliverables main chapter| [Deliverables main chapter ](https://github.com/huitemagico/kmac/wiki#deliverables) |
| Deliverable 1 | [Page1](https://github.com/huitemagico/kmac/wiki/KMAC-Deliverable-1) |
| Setup | [KMAC Setup](https://github.com/huitemagico/kmac/wiki/KMAC-Setup) |
| User Manual | [KMAC User Manual](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual) |

# Fast Deployment
1. In one terminal open a Stellar Node and run the soroban-preview docker container
```bash
bash src/quickstart.sh standalone #here you can also choose futurenet or testnet
```

2.- In another terminal open the soroban-preview docker container
```bash
bash src/run.sh
```
3.- Open kmac1 contract folder and run
```bash
cd contracts/kmac1
cargo test -- --nocapture #add explanation
soroban contract build --profile release-with-logs #add explanation
soroban config identity generate kreator
soroban config identity generate buyer
KREATOR=$(soroban config identity address kreator)
BUYER=$(soroban config identity address buyer)
```

```bash
soroban contract invoke \
    --source kreator \
    --wasm target/wasm32-unknown-unknown/release-with-logs/kmac1b.wasm \
    --id 1 \
    -- \
    kmac\
     --user $KREATOR    \
    --value 2 \
    --message "savekad" \
    --buyer $BUYER \
    --sender  "kreator"
```

# Introduction
 
## What is KMAC?
`@kmac` is a contract build with SOROBAN-SDK that makes an  implementation of an Finite State Machine template,  using knowed design pattern and providing extensible functions capabilities. <br />
[GitHub url](https://github.com/huitemagico/kmac)

![Kmac architecture](pictures/kmac03.vpd.png)

 
# Deliverables
Note about the deliverables: The following paragraphs (deliverable 1, deliverable 2, etc.) represent the 'deliverables' documented in the offer document. 

These are the 'conditions of satisfaction' for the customer, in this case, the SCF team.

To view the updated documentation that explains 'how' each deliverable meets the 'conditions of satisfaction' (as well as any issues, problems, pending tasks, or new features) for each KMAC version (KMAC1, KMAC2, etc.), please refer to the corresponding chapter in the KMAC wiki
For instance, for "Deliverable 1" please see the chapter at link: https://github.com/huitemagico/kmac/wiki#deliverable-1-proof-of-concept-poc-for-design-pattern-implementation


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

