use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Args {
	/// Height from where the consensus layer will be synced
	#[clap(long)]
	from_slot: Option<u64>,

	/// Height from where the execution layer will be synced
	#[clap(long)]
	from_block: Option<u64>,

	/// Height of the consensus slot at which the database will be fronzen
	#[clap(long)]
	freeze_at: u64,
}

impl Args {
	pub fn from_slot(&self) -> Option<u64> {
		self.from_slot
	}

	pub fn from_block(&self) -> Option<u64> {
		self.from_block
	}

	pub fn freeze_at(&self) -> u64 {
		self.freeze_at
	}
}
