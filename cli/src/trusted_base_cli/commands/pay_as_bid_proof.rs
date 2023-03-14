/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at
		http://www.apache.org/licenses/LICENSE-2.0
	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
*/

use crate::{
	trusted_cli::TrustedCli, trusted_command_utils::get_pair_from_str,
	trusted_operation::perform_trusted_operation, Cli,
};

use codec::Decode;
use ita_stf::{MerkleProofWithCodec, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use log::debug;
use sp_core::{Pair, H256};

use codec;

#[derive(Parser)]
pub struct PayAsBidProofCommand {
	/// AccountId in ss58check format
	account: String,
	orders_string: String,
	leaf_index: u8,
}

impl PayAsBidProofCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) {
		println!(
			"{:?}",
			// if we serialize with serde-json we can easily just pass it as
			// an argument in the verify-proof command.
			serde_json::to_string(&pay_as_bid_proof(
				cli,
				trusted_args,
				&self.account,
				self.orders_string.clone(),
				self.leaf_index
			))
			.unwrap()
		);
	}
}

pub(crate) fn pay_as_bid_proof(
	cli: &Cli,
	trusted_args: &TrustedCli,
	arg_who: &str,
	orders_string: String,
	leaf_index: u8,
) -> MerkleProofWithCodec<H256, Vec<u8>> {
	debug!("arg_who = {:?}", arg_who);
	let who = get_pair_from_str(trusted_args, arg_who);

	let top: TrustedOperation =
		TrustedGetter::pay_as_bid_proof(who.public().into(), orders_string, leaf_index)
			.sign(&KeyPair::Sr25519(Box::new(who)))
			.into();

	let res = perform_trusted_operation(cli, trusted_args, &top);

	match res {
		Some(value) => {
			let proof: MerkleProofWithCodec<_, _> =
				MerkleProofWithCodec::decode(&mut &value[..]).unwrap();
			proof
		},
		None => {
			panic!("Proof not found");
		},
	}
}