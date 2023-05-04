# ORACLES-NEAR 📄

CONTRACT=dev-1683178555977-26262511541746

echo $CONTRACT

Compilar y desplegar contrato Contrato:

    ./build.sh

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
    near call $CONTRACT feed_read '{"feed_address": "2fR7X8rUWhBxi4jv22N9Ee8spvPTXUXb9EcqaFXS7bcu"}' --accountId yairnava.testnet --gas=300000000000000

    NEAR PRICE
    near call $CONTRACT feed_read '{"feed_address": "5cgn4Q8S7tVkgqBj6GKFmaXwiTEhkuN3RkVEchhUSgU1"}' --accountId yairnava.testnet --gas=300000000000000

    Consultar cualquier fuente
    near call $CONTRACT feed_read '{"feed_address": "FeedAddress"}' --accountId yairnava.testnet --gas=300000000000000


## Promixityfi

    near view priceoracle.testnet get_asset '{"asset_id":"wrap.testnet"}' 

    near call $CONTRACT get_asset_promixityfi '{"asset_id":"wrap.testnet"}'  --accountId yairnava.testnet --gas=300000000000000