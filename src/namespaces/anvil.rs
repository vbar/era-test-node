use jsonrpc_derive::rpc;
use zksync_basic_types::{Address, U256};

use super::RpcResult;

#[rpc]
pub trait AnvilNamespaceT {
    /// Modifies an account's nonce by overwriting it.
    ///
    /// # Arguments
    ///
    /// * `address` - The `Address` whose nonce is to be changed
    /// * `nonce` - The new nonce
    ///
    /// # Returns
    ///
    /// A `BoxFuture` containing a `Result` with a `bool` representing the success of the operation.
    #[rpc(name = "anvil_setNonce")]
    fn set_nonce(&self, address: Address, balance: U256) -> RpcResult<bool>;
}
