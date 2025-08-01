use soroban_sdk::{contract, contractimpl, contracttype, Env, U256, Bytes};

#[repr(u32)]
#[derive(Copy, Clone)]
#[contracttype]
pub enum Stage {
    SrcWithdrawal = 0,
    SrcPublicWithdrawal = 1,
    SrcCancellation = 2,
    SrcPublicCancellation = 3,
    DstWithdrawal = 4,
    DstPublicWithdrawal = 5,
    DstCancellation = 6,
}

#[contract]
pub struct Timelocks;

const DEPLOYED_AT_OFFSET: u32 = 224;

// Contract implementation
#[contractimpl]
impl Timelocks {

    pub fn set_deployed_at(env: Env, timelocks: U256, value: U256) -> U256 {
        // Convert `timelocks` (U256) to `Bytes`, then copy into [u8; 32]
        let bytes_dyn = timelocks.to_be_bytes();
        let mut bytes: [u8; 32] = [0; 32];
        for (i, b) in bytes_dyn.iter().enumerate() {
            bytes[i] = b;
        }

        // Convert value to u32 using u128 -> u32 downcast
        let value_u32: u32 = value
            .to_u128()
            .expect("value too large")
            .try_into()
            .expect("value doesn't fit in u32");

        let value_bytes = value_u32.to_be_bytes();
        bytes[0..4].copy_from_slice(&value_bytes); // set bits 224–255

        let bytes_val = Bytes::from_array(&env, &bytes);
        U256::from_be_bytes(&env, &bytes_val)
    }

    pub fn rescue_start(timelocks: U256, rescue_delay: U256) -> U256 {
        rescue_delay.add(&(timelocks.shr(DEPLOYED_AT_OFFSET)))
    }

    pub fn get(env: Env, timelocks: U256, stage: Stage) -> U256 {
        let deployed_at = timelocks.shr(DEPLOYED_AT_OFFSET);
        let stage_offset = (stage as u32) * 32;

        // Extract the stage delta (32 bits) and add to deployed_at
        let stage_delta = timelocks
            .shr(stage_offset)
            .rem_euclid(&U256::from_u32(&env, u32::MAX));
        deployed_at.add(&stage_delta)
    }
}
