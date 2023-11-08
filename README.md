# kmac
# About this doc 
README.md Version November 08, 2023 12:14<br />
This is README.md file of the kmac project, at the main branch at https://github.com/huitemagico/kmac
IMPORTANT: This is an overview of the project!
You have to see the updated and detailed documentation in detail at 
https://github.com/huitemagico/kmac/wiki

# Introduction
 
## What is KMAC?
`@kmac` is a contract build with SOROBAN-SDK that makes an  implementation of an Finite State Machine template,  using knowed design pattern and providing extensible functions capabilities. <br />
[GitHub url](https://github.com/huitemagico/kmac)

![Kmac architecture](pictures/kmac03.vpd.png)


# Usage:
## Environment setup 
  If you want to access a comprehensive documentation for installing all the necessary tools to run the program, 
  please refer to the KMAC wiki, chapter "For installing the environment from Zero",  <br />
  at the following link: <br />
  [KMAC wiki] (https://github.com/huitemagico/kmac/wiki)

 ## Fast setup  
   The basic steps for setup and run kmac are the following: <br />

 1) cargo test <br />
 2) soroban contract build<br /> 
 3) identities setup:<br />
We need to set up some identities to use for testing and get their public keys: <br />

soroban config identity generate acc1 && \ <br />
soroban config identity generate acc2 && \ <br />
soroban config identity address acc1 && \ <br />
soroban config identity address acc2 <br />

 4) You must copy the string obtained and then insert in the runk1.sh script <br />

 Note about the shells: each kmac version (kmac1, kmac2, etc) could have updated shells for the respective kmac version.
 These are explained at the corresponding chapter at the
 KMAC wiki. Please refer to chapter title "Shells", at the kmac version.
 For example, for the kmac1 version (first deliverable)please see the chapter https://github.com/huitemagico/kmac/wiki#deliverable-1-about-the-shells<br />

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

 5) run the run0.sh (or the runk1.sh etc. see the explanation above)

## About the shells
 The following shells are basic development and test utility shell scripts.<br />
 compi.sh for compiling<br />
 bld.sh   for build<br />
 run0.sh  for run<br />
 (you must edit run0.sh with the identities obtained in the precedent paragraph)
 (see the above paragraph "Note about the shells")
 
# Deliverables
Note about the deliverables: The following paragraphs (deliverable 1, deliverable 2, etc.) represent the 'deliverables' documented in the offer document. In other words, these are the 'conditions of satisfaction' for the customer, in this case, the CSF team.

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

