# ORACLES-NEAR 📄

CONTRACT=dev-1682393090809-48003990518715

echo $CONTRACT

## Switchboard

1- Consultar Feeds

    https://switchboard.xyz/explorer

2- Crear Nuevo Feed

    https://app.switchboard.xyz/

2.1- Conectar wallet de Nigthly

    https://wallet.nightly.app/

2.2- Crear Feed

    https://app.switchboard.xyz/template/b5bfc7b2-4668-4f65-8ac5-8d12505ebab4

3- Consultar Feed

    Bitcoin price
    near call dev-1682393090809-48003990518715 feed_read '{"feed_address": "2fR7X8rUWhBxi4jv22N9Ee8spvPTXUXb9EcqaFXS7bcu"}' --accountId yairnava.testnet --gas=300000000000000

    NEAR PRICE
    near call dev-1682393090809-48003990518715 feed_read '{"feed_address": "5cgn4Q8S7tVkgqBj6GKFmaXwiTEhkuN3RkVEchhUSgU1"}' --accountId yairnava.testnet --gas=300000000000000

## Promixityfi

    near view priceoracle.testnet get_asset '{"asset_id":"wrap.testnet"}' 

    near call dev-1682393090809-48003990518715 get_asset_promixityfi '{"asset_id":"wrap.testnet"}'  --accountId yairnava.testnet --gas=300000000000000

    

Documentación:

https://docs.near.org/develop/relevant-contracts/oracles
https://docs.switchboard.xyz/near/dev/feed-parser
https://github.com/switchboard-xyz/sbv2-near
https://github.com/switchboard-xyz/sbv2-near/tree/main/programs/feed-parser
https://docs.near.org/develop/contracts/crosscontract