use bitcoin::Amount;

pub const DUST_AMOUNT: Amount = Amount::from_sat(1000);
pub const STAKE_VALUE_INDEX: u32 = 0;
pub const STAKE_TIMELOCK_INDEX: u32 = 1;
pub const CONNECTOR_A_INDEX: u32 = 2;
pub const CONNECTOR_B_INDEX: u32 = 3;
pub const CONNECTOR_C_INDEX: u32 = 1;
pub const CHALLENGE_FEE_INDEX: u32 = 4;

pub const STAKE_FEE_AMOUNT: Amount = Amount::from_sat(300);
pub const CHALLENGE_FEE_AMOUNT: Amount = Amount::from_sat(300);
pub const ASSERT_FEE_AMOUNT: Amount = Amount::from_sat(400);
pub const DISPROVE_FEE_AMOUNT: Amount = Amount::from_sat(1_000);

// Regtest
// pub const STAKE_AMOUNT: Amount = Amount::from_sat(10_000_000);
// pub const CHALLENGE_AMOUNT: Amount = Amount::from_sat(5_000_000);
// pub const COMMITTEE_RESERVE_AMOUNT: Amount = Amount::from_sat(5_000_000);

// Signet
pub const STAKE_AMOUNT: Amount = Amount::from_sat(3_000);
pub const CHALLENGE_AMOUNT: Amount = Amount::from_sat(1_000);
pub const COMMITTEE_RESERVE_AMOUNT: Amount = Amount::from_sat(1_000);
