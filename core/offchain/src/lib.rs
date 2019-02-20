// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate service. Starts a thread that spins up the network, client, and extrinsic pool.
//! Manages communication between them.

#![warn(missing_docs)]

use std::{
	marker::PhantomData,
	sync::Arc,
};

use client::runtime_api::{ApiExt, OffchainWorker};
use runtime_primitives::{
	ExecutionContext,
	generic::BlockId,
	traits::{self, ProvideRuntimeApi},
};

use futures::Future;

// TODO [ToDr] move the declaration to separate primitives crate with std/no-std options.
// decl_runtime_apis! {
// 	/// The offchain worker api.
// 	pub trait OffchainWorkerApi {
// 		/// Starts the off-chain task for given block number.
// 		fn generate_extrinsics(number: <<Block as BlockT>::Header as HeaderT>::Number);
// 	}
// }

#[derive(Debug)]
pub struct OffchainWorkers<C, Block> {
	client: Arc<C>,
	_block: PhantomData<Block>,
}

impl<C, Block> OffchainWorkers<C, Block> where
	C: ProvideRuntimeApi,
	C::Api: OffchainWorker<Block>,
	Block: traits::Block,
{
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_block: PhantomData,
		}
	}

	pub fn on_block_imported(&self, number: &<Block::Header as traits::Header>::Number) -> impl Future<Item = (), Error = ()> {
		let runtime = self.client.runtime_api();
		let at = BlockId::number(*number);
		if let Ok(true) = runtime.has_api::<OffchainWorker<Block>>(&at) {
			runtime.generate_extrinsics_with_context(&at, ExecutionContext::OffchainWorker, *number).unwrap();
		}
		return futures::future::ok(())
	}
}
