# requirements for bridge bank contract

bridge bank contract works as the interface for any cosmos based chain to import/export tokens from other blockchains.

## bridgebank

the controller contract. 

### admin
### relayers
### lock native token
### lock cw20 token
### burn pegged token
### token import
### token burn back

### network
### fungible token identifier (could be address in ethereum). unique
### decimal 
### name

## bridgetoken

the contract for single token, it will map to any fungible token in other chain (evm, cosmos or polkadot)

### bridgebank address 
### counter part

## validators
### validator for each network
### voting power for each network
### threshold for each network
