/*
	Copyright 2019 Supercomputing Systems AG

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

pub static ENCLAVE_TOKEN:		&str = "./bin/enclave.token";
pub static ENCLAVE_FILE:		&str = "./bin/enclave.signed.so";
pub static RSA_PUB_KEY:			&str = "./bin/rsa_pubkey.txt";
pub static ECC_PUB_KEY:			&str = "./bin/ecc_pubkey.txt";

pub static ATTN_REPORT_FILE:    &str = "./bin/attestation_report.json";

pub static RA_SPID:				&str = "./bin/spid.txt";
pub static RA_CERT:				&str = "./bin/client.crt";
pub static RA_KEY:				&str = "./bin/client.key";
