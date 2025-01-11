# kmac
# About this doc 
README.md 'wikiVersion' January 05, 2025. New jpg showing the mindmap of the system context of the KMAC program. See wiki doc in this same site. <br />
README.md 'wikiVersion' December 31 2023. Updated the [test module](https://github.com/huitemagico/kmac/wiki/test-module). <br />
README.md 'wikiVersion' Jun 19 2024 20:23. Updated setup(https://github.com/huitemagico/kmac/wiki/KMAC-Setup). <br />
# KMAC VERSION
"Final Production Version" 1.0.0, December 31, 2023
# KMAC wiki
This is the README page of KMAC.<br>
 

## What is KMAC?
KMAC is the project for building a contract, with the [SOROBAN-SDK tools](https://soroban.stellar.org/).<br>
KMAC implements a Finite State Machine [template](https://github.com/huitemagico/kmac/wiki/Design-Patterns#design-the-template-approach) with the purpose of simplifying new developments using this pattern.<br>

![Vending Machine Example](https://github.com/huitemagico/kmac/blob/main/pictures/kmcorefsmdiagram10.vpd.png)

The KMAC program has implemented the template and has provided an example of using this template. 

Really, we could see KMAC as [three different points of view](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#what-is-kmac-the-three-views-of-kmac-), depending of your interest:<br>
(1) The KMAC template program, (2) the KMAC Vending Machine model application example, and (3) the KMAC user extensions feature.

## How do I build KMAC?
The KMAC project intends to utilize [well-known design patterns](https://github.com/huitemagico/kmac/wiki/Design-Patterns).<br>
The primary goal is to practice implementing these features. The intention is to provide easily understandable and reusable components through this approach. This aligns with the final goal of KMAC, which is to be an Educational Project.

## About the architecture of KMAC-
### Extensible
KMAC providing [extensible functions capabilities](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions).<br>
This means that you can implement other examples of systems based on Finite State Machine model,  **without the need to code the logic** of the Finite State Machine flow. You only need to focus on programming the specific 'user' functions for your new model.

See the next image "KMAC architecture diagram"
![kmac Architecture Diagram](https://github.com/huitemagico/kmac/blob/main/pictures/kmac03.vpd.png)

## About the actors involved in KMAC
![Actor use cases](https://github.com/huitemagico/kmac/blob/main/pictures/actorsusecases.vpd.png)
[Actors Use Cases](https://github.com/huitemagico/kmac/wiki/KMAC-actors-use-cases)

## For installing the environment 
Please refer to [kmac setup](https://github.com/huitemagico/kmac/wiki/KMAC-Setup)

## Table of contents 
For easier navigation of the documentation, you can use the table below (alphabetically ordered) or go directly to the [main doc page](https://github.com/huitemagico/kmac/wiki).

| Content| Link |Purpose|
| --- | --- |---|
| context |[Global context] | Global Context of the KMAC program|
| architecture| [KMAC architecture](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#the-kmac-architecture-diagram) |Review of KMAC architecture|
|dashboard| [dashboard](https://github.com/huitemagico/kmac/wiki/Design-Patterns#coding-the-kmac-dashboard-utility-for-inspecting-results) |How understand the output of the vending machine program|
|dashboard| [dashboard](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-version-december-19-2023-output-example-and-explanation) |How understand the output of the vending machine program|
|deliverables (history of)| [deliverables (history)](https://github.com/huitemagico/kmac/wiki/Deliverables-proposal) |Deliverables description|
|deliverables page| [Deliverables main chapter ](https://github.com/huitemagico/kmac/wiki#deliverables) |Deliverables achieved description|
|finite state machine (FSM)| [finite state machine](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#the-complete-fsm-diagram-of-the-vending-machine) |Finite state machine how to|
|flow diagram| [flow diagram](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#the-flow-diagram-of-kmac) |flow diagram|
|functions explained| [functions](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#some-functions-explained) |description of (some) function of KMAC|
|matrix of functions| [matrix](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90extending-feature:-How-add-user-functions#m-a-t-r-i-x-n-u-m) |understand how use the matrix for build a new program |
|modules (of kmac)| [modules](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#modules) |description of KMAC modules|
|network configuration| [modules](https://github.com/huitemagico/kmac/wiki/Configuring-network-and-identity#how-to-configure-network-and-identity) |how configure the network|
|identity configuration| [identity](https://github.com/huitemagico/kmac/wiki/Configuring-network-and-identity#how-to-configure-network-and-identity) |how configure the identities|
|SCF Dashboard | [KMAC at SCF](https://dashboard.communityfund.stellar.org/scfawards/scf-20/panelreview/suggestion/103) |KMAC at SCF|
|shell comands| [shell comands for the vending machine](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-the-sequence-of-command-shell-transactions-takeaways) |shell commands for run the vending machine example|
|storage management| [storage management](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#storage-management-use-cases) |how KMAC use storage|
|security| [security](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#notes-about-security-and-require_auth) |how KMAC deals with|
|soroban rust sdk| [soroban rust sdk instructions](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#soroban-sdk-rust-instructions) |selected RUST instructions |
|sequence diagram | [sequence diagram for Vending Machine example](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#vending-machine-example-sequence-diagram) |the sequence of transactions|
| setup KMAC | [KMAC Setup](https://github.com/huitemagico/kmac/wiki/KMAC-Setup) |how setup run and test the KMAC|
|template| [template](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#c-how-to-extend-the-template) |the template KMAC|
|template| [template design approach](https://github.com/huitemagico/kmac/wiki/Design-Patterns#design-the-template-approach) |the template KMAC|
|user transactions| [user transactions](https://github.com/huitemagico/kmac/wiki/KMAC%E2%80%90technical-description#the-function11-function12-at-kmacusermodrs) |user functions how to|
|use cases-actors|[Actor use cases](https://github.com/huitemagico/kmac/wiki/KMAC-actors-use-cases)|Who-is-who at KMAC approach|
|user Manual of KMAC| [KMAC User Manual](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual) |"user manual"|
|vending machine| [vending machine](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#b-the-example-of-the-vending-machine) |The Vending Machine example|
|vending machine output| [vending machine dashboard output](https://github.com/huitemagico/kmac/wiki/KMAC-User-Manual#the-vending-machine-example-version-december-19-2023-output-example-and-explanation) |the vending machine output|

