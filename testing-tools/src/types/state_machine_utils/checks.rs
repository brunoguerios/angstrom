use reth_chainspec::Hardforks;
use reth_network::Peers;
use reth_provider::{BlockReader, ChainSpecProvider, HeaderProvider, ReceiptProvider};

use crate::{
    controllers::enviroments::{AngstromTestnet, DevnetStateMachine},
    providers::WalletProvider,
    types::{StateMachineCheckHookFn, config::DevnetConfig}
};

pub trait WithCheck<C>
where
    C: BlockReader<Block = reth_primitives::Block>
        + ReceiptProvider<Receipt = reth_primitives::Receipt>
        + HeaderProvider<Header = reth_primitives::Header>
        + ChainSpecProvider<ChainSpec: Hardforks>
        + Unpin
        + Clone
        + 'static
{
    type FunctionOutput;

    fn check_block(&mut self, block_number: u64);
}

impl<C, P: Peers + Unpin + 'static> WithCheck<C> for DevnetStateMachine<'_, C, P>
where
    C: BlockReader<Block = reth_primitives::Block>
        + ReceiptProvider<Receipt = reth_primitives::Receipt>
        + HeaderProvider<Header = reth_primitives::Header>
        + ChainSpecProvider<ChainSpec: Hardforks>
        + Unpin
        + Clone
        + 'static
{
    type FunctionOutput = StateMachineCheckHookFn<C, P>;

    fn check_block(&mut self, block_number: u64) {
        let f = move |testnet: &mut AngstromTestnet<C, DevnetConfig, WalletProvider, P>| {
            testnet.check_block_numbers(block_number)
        };
        self.add_check("check block", f);
    }
}
