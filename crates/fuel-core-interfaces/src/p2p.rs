use fuel_core_types::{
    blockchain::{
        block::Block,
        consensus::{
            Consensus,
            ConsensusVote,
        },
        primitives::BlockHeight,
        SealedBlock,
    },
    fuel_tx::Transaction,
};
use std::{
    fmt::Debug,
    sync::Arc,
};
use tokio::sync::oneshot;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GossipsubMessageAcceptance {
    Accept,
    Reject,
    Ignore,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GossipsubMessageInfo {
    pub message_id: Vec<u8>,
    pub peer_id: Vec<u8>,
}

impl<T> From<&GossipData<T>> for GossipsubMessageInfo {
    fn from(gossip_data: &GossipData<T>) -> Self {
        Self {
            message_id: gossip_data.message_id.clone(),
            peer_id: gossip_data.peer_id.clone(),
        }
    }
}

#[derive(Debug)]
pub enum P2pRequestEvent {
    RequestBlock {
        height: BlockHeight,
        response: oneshot::Sender<SealedBlock>,
    },
    BroadcastNewTransaction {
        transaction: Arc<Transaction>,
    },
    BroadcastNewBlock {
        block: Arc<Block>,
    },
    BroadcastConsensusVote {
        vote: Arc<ConsensusVote>,
    },
    GossipsubMessageReport {
        message: GossipsubMessageInfo,
        acceptance: GossipsubMessageAcceptance,
    },
    Stop,
}

#[derive(Debug, Clone)]
pub struct GossipData<T> {
    pub data: Option<T>,
    pub peer_id: Vec<u8>,
    pub message_id: Vec<u8>,
}

pub type ConsensusGossipData = GossipData<Consensus>;
pub type TransactionGossipData = GossipData<Transaction>;
pub type BlockGossipData = GossipData<Block>;

impl<T> GossipData<T> {
    pub fn new(
        data: T,
        peer_id: impl Into<Vec<u8>>,
        message_id: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            data: Some(data),
            peer_id: peer_id.into(),
            message_id: message_id.into(),
        }
    }
}

pub trait NetworkData<T>: Debug + Send {
    fn take_data(&mut self) -> Option<T>;
}

impl<T: Debug + Send + 'static> NetworkData<T> for GossipData<T> {
    fn take_data(&mut self) -> Option<T> {
        self.data.take()
    }
}
