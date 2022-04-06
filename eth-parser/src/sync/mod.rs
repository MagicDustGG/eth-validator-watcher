pub(crate) mod consensus_layer;
pub(crate) mod execution_layer;
pub(crate) mod syncer;
pub(crate) mod validators;

pub(crate) use consensus_layer::*;
pub(crate) use execution_layer::*;
pub(crate) use syncer::*;

#[derive(Debug)]
pub enum SyncError {
	/// Block not found at height
	NothingAtHeight(u64),
	/// The indexed block was pending
	PendingBlock(u64),
	/// The client did not return any validators
	NoValidators,
}
