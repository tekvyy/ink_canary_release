# Canary Release Capability (Ink Smart Contracts)

## Overview

Submission for [Polkadot ink! Hackathon powered by Encode Club](https://www.encode.club/polkadot-ink-hackathon) 


This repository contains the source code and documentation for enhancing the functionality of Ink smart contracts on Substrate to support Canary releases. 

Canary releases are a way to gradually roll out new features or changes to a subset of users to test and validate them before deploying to a wider audience.

![Alt text](docs/canary_release.webp?raw=true "Main Image")


## Features

- **Canary Release Support**: Easily create and manage Canary releases for your smart contracts.
- **Gradual Rollout**: Control the percentage of users or nodes that receive the new features during a Canary release.

## Getting Started

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/tekvyy/ink_canary_release.git


2. Build Project

   ```bash
   cd ./canary_contract
   cargo contract build
   
3. Deploy Smart Contract on Chain

4. Ability To Select 2 Smart Contract on Contracts UI 
- Main Contract
- Canary Contract

![Alt text](docs/select_contracts.png?raw=true "Main Image")

5. Ability to Switch Percentage for Canary Release users via smart contract 


   
