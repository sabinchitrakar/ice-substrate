use frost_runtime::{
	currency::ICY, opaque::SessionKeys, AccountId, AirdropConfig, AuraConfig, BalancesConfig,
	CouncilConfig, CouncilMembershipConfig, DemocracyConfig, EVMConfig, EthereumConfig,
	GenesisConfig, GrandpaConfig, IndicesConfig, SessionConfig, Signature, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, TechnicalMembershipConfig, TreasuryPalletId,
	WASM_BINARY,
};
use hex_literal::hex;
use sc_chain_spec::Properties;
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{AccountIdConversion, IdentifyAccount, Verify};
use std::collections::BTreeMap;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type FrostChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn frost_properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("tokenSymbol".into(), "ICY".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	properties
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

const AIRDROP_MERKLE_ROOT: [u8; 32] =
	hex_literal::hex!("990e01e3959627d2ddd94927e1c605a422b62dc3b8c8b98d713ae6833c3ef122");

/// Initialize frost testnet configuration
pub fn testnet_config() -> Result<FrostChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities = vec![
		(
			hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"]
				.unchecked_into(),
			hex!["27c6da25d03bb6b3c751da3e8c5265b0bb357c15240602443cc286c0658b47f9"]
				.unchecked_into(),
		),
		(
			hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"]
				.unchecked_into(),
			hex!["85ec524aeacb6e558619a10da82cdf787026209211d1b7462cb176d58f2add86"]
				.unchecked_into(),
		),
	];

	let council_members =
		vec![hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into()];

	let technical_committee_membership =
		vec![hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into()];

	let root_key: AccountId =
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into();

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	let endowed_accounts = vec![
		TreasuryPalletId::get().into_account_truncating(),
		hex!["62687296bffd79f12178c4278b9439d5eeb8ed7cc0b1f2ae29307e806a019659"].into(),
		hex!["d893ef775b5689473b2e9fa32c1f15c72a7c4c86f05f03ee32b8aca6ce61b92c"].into(),
		hex!["98003761bff94c8c44af38b8a92c1d5992d061d41f700c76255c810d447d613f"].into(),
	];

	Ok(FrostChainSpec::from_genesis(
		"Frost Testnet",
		"frost-testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				initial_authorities.clone(),
				council_members.clone(),
				technical_committee_membership.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				endowed_accounts.clone(),
				true,
			)
		},
		vec![],
		None,
		None,
		None,
		frost_properties().into(),
		None,
	))
}

/// Initialize frost development configuration
pub fn development_config() -> Result<FrostChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities = vec![authority_keys_from_seed("Alice")];

	let council_members = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let technical_committee_membership = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	let endowed_accounts = vec![
		TreasuryPalletId::get().into_account_truncating(),
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
	];

	Ok(FrostChainSpec::from_genesis(
		"Frost Development",
		"frost-dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				initial_authorities.clone(),
				council_members.clone(),
				technical_committee_membership.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				endowed_accounts.clone(),
				true,
			)
		},
		vec![],
		None,
		None,
		None,
		frost_properties().into(),
		None,
	))
}

/// Initialize frost local testnet configuration
pub fn local_testnet_config() -> Result<FrostChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let initial_authorities = vec![
		authority_keys_from_seed("Alice"),
		authority_keys_from_seed("Bob"),
	];

	let council_members = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let technical_committee_membership = vec![get_account_id_from_seed::<sr25519::Public>("Alice")];

	let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

	let airdrop_creditor_account: AccountId =
		hex!["10b3ae7ebb7d722c8e8d0d6bf421f6d5dbde8d329f7c905a201539c635d61872"].into();

	let endowed_accounts = vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	];

	Ok(FrostChainSpec::from_genesis(
		"Frost Local Testnet",
		"frost-local-testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				initial_authorities.clone(),
				council_members.clone(),
				technical_committee_membership.clone(),
				root_key.clone(),
				airdrop_creditor_account.clone(),
				endowed_accounts.clone(),
				true,
			)
		},
		vec![],
		None,
		None,
		None,
		None,
		None,
	))
}

/// Helper for session keys to map aura id
fn session_keys(aura: AuraId, grandpa: GrandpaId) -> SessionKeys {
	SessionKeys { aura, grandpa }
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	_initial_authorities: Vec<(AuraId, GrandpaId)>,
	council_members: Vec<AccountId>,
	technical_committee_membership: Vec<AccountId>,
	root_key: AccountId,
	airdrop_creditor_account: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	let authorities = vec![
		(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			authority_keys_from_seed("Alice").0,
			authority_keys_from_seed("Alice").1,
		),
		(
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			authority_keys_from_seed("Bob").0,
			authority_keys_from_seed("Bob").1,
		),
	];

	GenesisConfig {
		system: SystemConfig {
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, ICY * 40_000))
				.collect(),
		},
		aura: AuraConfig {
			authorities: vec![],
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		sudo: SudoConfig {
			key: Some(root_key),
		},
		session: SessionConfig {
			keys: authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.1.clone(), x.2.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		evm: EVMConfig {
			accounts: { BTreeMap::new() },
		},
		ethereum: EthereumConfig {},
		dynamic_fee: Default::default(),
		base_fee: Default::default(),
		vesting: Default::default(),
		assets: Default::default(),
		council_membership: CouncilMembershipConfig {
			members: council_members,
			phantom: Default::default(),
		},
		council: CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		},
		treasury: Default::default(),
		simple_inflation: Default::default(),
		fees_split: Default::default(),
		airdrop: AirdropConfig {
			creditor_account: airdrop_creditor_account,
			merkle_root: AIRDROP_MERKLE_ROOT,
		},
		technical_membership: TechnicalMembershipConfig {
			members: technical_committee_membership,
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		},
		phragmen_election: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		democracy: DemocracyConfig::default(),
	}
}
