use soroban_sdk::{contract, contractimpl, contracttype, Env, U256};

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
        // Create mask for upper 32 bits (0xffff_ffff << 224)
        let mask = U256::from_parts(&env, 0xffff_ffff, 0, 0, 0);

        // Clear the upper 32 bits by subtracting the mask if it's set
        let cleared = if timelocks >= mask {
            timelocks.sub(&mask)
        } else {
            timelocks
        };

        let shifted_value = value.shl(DEPLOYED_AT_OFFSET);
        cleared.add(&shifted_value)
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
