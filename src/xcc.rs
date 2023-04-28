use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};
pub type AssetId = String;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_xcc)]
pub trait ExternsContract {
    fn get_asset(&self,asset_id: String) -> U128;
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {

}