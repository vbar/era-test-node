use zksync_basic_types::{Address, U256};
use zksync_web3_decl::error::Web3Error;

use crate::{
    fork::ForkSource,
    namespaces::{AnvilNamespaceT, RpcResult},
    node::InMemoryNode,
    utils::{into_jsrpc_error, IntoBoxedFuture},
};

impl<S: ForkSource + std::fmt::Debug + Clone + Send + Sync + 'static> AnvilNamespaceT
    for InMemoryNode<S>
{
    fn set_nonce(&self, address: Address, balance: U256) -> RpcResult<bool> {
        self.set_nonce(address, balance)
            .map_err(|err| {
                tracing::error!("failed setting nonce: {:?}", err);
                into_jsrpc_error(Web3Error::InternalError(err))
            })
            .into_boxed_future()
    }

    fn impersonate_account(&self, address: Address) -> RpcResult<bool> {
        self.impersonate_account(address)
            .map_err(|err| {
                tracing::error!("failed impersonating account: {:?}", err);
                into_jsrpc_error(Web3Error::InternalError(err))
            })
            .into_boxed_future()
    }

    fn stop_impersonating_account(&self, address: Address) -> RpcResult<bool> {
        InMemoryNode::<S>::stop_impersonating_account(self, address)
            .map_err(|err| {
                tracing::error!("failed stopping to impersonate account: {:?}", err);
                into_jsrpc_error(Web3Error::InternalError(err))
            })
            .into_boxed_future()
    }
}
