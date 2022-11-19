![image](logos/256x256.png)
# FIL Peggy: $\mathbb{\color{#1e46a2}{Manage \ your \ a \ miner \ in \ with \ FVM}}$
### ${\color{blue}Let \space your \space friend \space invest \space your \space miner \space through \space smart \space contract \space directly}$

## 1 Overview

When user want to run a Filecoin miner node, they need to have an account or multisig account, then they can create miner. Account which is used to create the miner is the owner of the miner. The owner can send a request to network to change miner's owner to a new account. All management of miner can only be executed by owner. Mining reward will be withdrawed to owner, or some beneficiary account set by owner.

## 2 Problem

We have three problems if miner is managed by an account whose private key is hold by one or several users.
* If the owner account is hold by the user who own the miner node, when the private key is lost, or stolen by someone else, then user cannot control miner anymore;
* if the miner node is operated by more than 1 users, they may use a multisig account as the owner. But as we know, if we want to keep the private key safe (not lost), the power of each private key is not affair, then some operator may union to do evil;
* When miner node operator want to let investor to invest their miner, then pay benefit to investors with feature mining reward, token holder won't trust miner node operator so easy if owner private key is hold by somebody.

## 3 Solution

With FVM, we can manage miner with smart contract. The smart contract implements all ability to manage miner. All strategies will be implemented and opened to all users who take part in to review. So all user can invoke the smart contract to trigger something, but what's triggered by them is always the same workflow. Currently FVM do not support to use smart contract as miner's worker address, but it may be supported in future, then we can also use smart contract as worker

```mermaid
flowchart TD
  A[Miner Node Operator] -- CreateMiner/ChangeOwner --> B[Miner Node]
  A[Miner Node Operator] -- InstallActor/CreateActor/TakeOwner --> C[Native Smart Contract]
  C[Native Smart Contract] -- Withdraw/ExtendSectorExpiration --> B[Miner Node]
  D[Token Holder] -- Deposit/Governance/Withdraw --> C[Native Smart Contract]
```

### Miner Node Operator
* Create miner
* Create native smart contract, set smart contract parameter
  * Set operator fee
  * Set dataset fee
  * Set device holder fee
  * Etc
* Change miner's owner to smart contract
* Publish miner smart contract and investment term

### Native Actor
* Play as miner's owner
* Accept deposit from token holder and record deposit accounts and amount
* Distribute mining reward to deposited accounts according to amount ratio
* Mint nft of the amount
* Process nft transfer then transfer beneficiary account
* Transfer deposited FIL to pledge account or pledge smart contract
* Provide liquidity to user who want to withdraw before their collateral expired
* Upgrade work flow with governance of all deposited accounts

We'll benefit from using smart contract as miner's owner:
* The code of owner smart contract is open source, everybody can review the work flow;
* Smart contract implement the execution logic. Miner node operator create miner, then set miner's owner to smart contract. Smart contract won't accept change owner request again, so the miner will be managed by smart contract in its lifetime. Nobody can do anything which is not defined in smart contract. If the miner is owned by sereval partner, partner won't be worry about lose private key or some partners union to do evil anymore;
* Execution process of smart contract is transparency. It could be autited by everybody. Anybody can invoke the smart contract, but any invocation can only execute predefined process in the smart contract. With smart contract, all parties can know the details of the miner better;
* Mining reward distribution process is implemented in the smart contract. It'll be run with a cron process, or triggerd by somebody. We do not need someone to calculate the reward distribution anymore;
* Miner with smart contract owner can face to more token holders. Token holder can invest any amount to smart contract, then just wait for reward;
* Amount of the smart contract can be minted to NFT, then the NFT could be transfered. New owner of the NFT can change the old amount beneficiary to theiry new acount.

## 4 FVM Validation
### 4.1 CreateMiner with FVM

Native smart contract cannot create miner directly. Only account or multisig account can create miner. We don't know if it'll be supported by FVM in future.

### 4.2 Use smart contract address as miner's owner

Smart contract address could be used as miner's owner and could implement all owner functionality in the smart contract.

### 4.2 Record deposit (invoke smart contract with method 0 to send FIL to smart contract address) account inside smart contract

Actually currently we cannot know about who deposit FIL to smart contract address inside smart contract. Issue https://github.com/filecoin-project/ref-fvm/issues/835 let us know about that FVM team is considering about that. But even it cannot be supported, we still can use some half-decentralized method to implement this requirement.

### 4.3 Use smart contract address as miner's worker

Smart contract address cannot be used as miner's worker.

### 5 Peggy Smart Contract Functionalities
* All funtionalities to manage miner
* Detect deposit to smart contract address inside smart contract
* Mining reward distribution according to amount ratio
* Upgrade governance process
