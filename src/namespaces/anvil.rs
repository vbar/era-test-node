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

    /// Era Test Node allows transactions impersonating specific account and contract addresses.
    /// To impersonate an account use this method, passing the address to impersonate as its parameter.
    /// After calling this method, any transactions with this sender will be executed without verification.
    /// Multiple addresses can be impersonated at once.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to impersonate
    ///
    /// # Returns
    ///
    /// A `BoxFuture` containing a `Result` with a `bool` representing the success of the operation.
    #[rpc(name = "anvil_impersonateAccount")]
    fn impersonate_account(&self, address: Address) -> RpcResult<bool>;

    /// Use this method to stop impersonating an account after having previously used `anvil_impersonateAccount`
    /// The method returns `true` if the account was being impersonated and `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to stop impersonating.
    ///
    /// # Returns
    ///
    /// A `BoxFuture` containing a `Result` with a `bool` representing the success of the operation.
    #[rpc(name = "anvil_stopImpersonatingAccount")]
    fn stop_impersonating_account(&self, address: Address) -> RpcResult<bool>;
}
