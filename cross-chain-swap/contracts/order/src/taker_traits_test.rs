#![cfg(test)]

use core::fmt::Debug;

use soroban_sdk::{log, testutils::Address as _, Address, Env, U256};

// use crate::taker_trait::{MakerTraitsBuilder, MakerTraitsLib};
// use crate::taker_traits;


use crate::taker_traits::{TakerTraitsBuilder,TakerTraitsLib};

fn create_test_env() -> Env {
    Env::default()
}

fn create_test_address(env: &Env) -> Address {
    Address::generate(env)
}

// #[test]
// fn test_has_extension_flag_taker() {
//     let env = create_test_env();

//     // Test with extension flag set
//     let traits_with_extension = MakerTraitsBuilder::new(env.clone())
//         .with_extension()
//         .build();

//     assert!(MakerTraitsLib::has_extension(
//         env.clone(),
//         traits_with_extension
//     ));

//     // Test without extension flag
//     let traits_without_extension = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::has_extension(
//         env.clone(),
//         traits_without_extension
//     ));
// }

// #[test]
// fn test_is_allowed_sender() {
//     let env = create_test_env();
//     let sender = create_test_address(&env);

//     // Test with any sender allowed (default)
//     let traits_any_sender = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(MakerTraitsLib::is_allowed_sender(
//         &env,
//         traits_any_sender,
//         sender.clone()
//     ));

//     // Test with specific sender bits
//     let sender_bits = 0x123456789abcdef0u128;
//     let traits_specific_sender = MakerTraitsBuilder::new(env.clone())
//         .with_allowed_sender(sender_bits)
//         .build();

//     // This test will fail because we're not using the exact sender bits from the address
//     // But it tests the logic flow
//     let result = MakerTraitsLib::is_allowed_sender(&env, traits_specific_sender, sender);
//     // The result depends on the actual address bytes
//     assert!(result == true || result == false); // Just ensure it doesn't panic
// }

// #[test]
// fn test_nonce_or_epoch() {
//     let env = create_test_env();
//     let test_nonce = 12345u64;

//     let traits = MakerTraitsBuilder::new(env.clone())
//         .with_nonce_or_epoch(test_nonce)
//         .build();

//     assert_eq!(MakerTraitsLib::nonce_or_epoch(&env, traits), test_nonce);
// }

// #[test]
// fn test_series() {
//     let env = create_test_env();
//     let test_series = 67890u64;

//     let traits = MakerTraitsBuilder::new(env.clone())
//         .with_series(test_series)
//         .build();

//     assert_eq!(MakerTraitsLib::series(&env, traits), test_series);
// }

// #[test]
// fn test_allow_partial_fills() {
//     let env = create_test_env();

//     // Test default (allows partial fills)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(MakerTraitsLib::allow_partial_fills(&env, traits_default));

//     // Test with no partial fills flag set
//     let traits_no_partial = MakerTraitsBuilder::new(env.clone())
//         .no_partial_fills()
//         .build();

//     assert!(!MakerTraitsLib::allow_partial_fills(
//         &env,
//         traits_no_partial
//     ));
// }

// #[test]
// fn test_need_pre_interaction_call() {
//     let env = create_test_env();

//     // Test default (no pre-interaction)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::need_pre_interaction_call(
//         &env,
//         traits_default
//     ));

//     // Test with pre-interaction flag set
//     let traits_pre_interaction = MakerTraitsBuilder::new(env.clone())
//         .with_pre_interaction_call()
//         .build();

//     assert!(MakerTraitsLib::need_pre_interaction_call(
//         &env,
//         traits_pre_interaction
//     ));
// }

// #[test]
// fn test_need_post_interaction_call() {
//     let env = create_test_env();

//     // Test default (no post-interaction)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::need_post_interaction_call(
//         &env,
//         traits_default
//     ));

//     // Test with post-interaction flag set
//     let traits_post_interaction = MakerTraitsBuilder::new(env.clone())
//         .with_post_interaction_call()
//         .build();

//     assert!(MakerTraitsLib::need_post_interaction_call(
//         &env,
//         traits_post_interaction
//     ));
// }

// #[test]
// fn test_allow_multiple_fills() {
//     let env = create_test_env();

//     // Test default (no multiple fills)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::allow_multiple_fills(&env, traits_default));

//     // Test with multiple fills flag set
//     let traits_multiple_fills = MakerTraitsBuilder::new(env.clone())
//         .allow_multiple_fills()
//         .build();

//     assert!(MakerTraitsLib::allow_multiple_fills(
//         &env,
//         traits_multiple_fills
//     ));
// }

// #[test]
// fn test_use_bit_invalidator() {
//     let env = create_test_env();

//     // Test default (no partial fills, no multiple fills) - should use bit invalidator
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(MakerTraitsLib::use_bit_invalidator(&env, traits_default));

//     // Test with both partial and multiple fills allowed - should not use bit invalidator
//     let traits_both_fills = MakerTraitsBuilder::new(env.clone())
//         .allow_multiple_fills()
//         // Note: allow_partial_fills is default behavior (not setting no_partial_fills)
//         .build();

//     assert!(!MakerTraitsLib::use_bit_invalidator(
//         &env,
//         traits_both_fills
//     ));

//     // Test with no partial fills but allow multiple fills - should use bit invalidator
//     let traits_no_partial_multi = MakerTraitsBuilder::new(env.clone())
//         .no_partial_fills()
//         .allow_multiple_fills()
//         .build();

//     assert!(MakerTraitsLib::use_bit_invalidator(
//         &env,
//         traits_no_partial_multi
//     ));
// }

// #[test]
// fn test_need_check_epoch_manager() {
//     let env = create_test_env();

//     // Test default (no epoch check)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::need_check_epoch_manager(
//         &env,
//         traits_default
//     ));

//     // Test with epoch manager flag set
//     let traits_epoch_check = MakerTraitsBuilder::new(env.clone())
//         .need_check_epoch_manager()
//         .build();

//     assert!(MakerTraitsLib::need_check_epoch_manager(
//         &env,
//         traits_epoch_check
//     ));
// }

// #[test]
// fn test_use_permit2() {
//     let env = create_test_env();

//     // Test default (no permit2)
//     let traits_default = MakerTraitsBuilder::new(env.clone()).build();

//     assert!(!MakerTraitsLib::use_permit2(env.clone(), traits_default));

//     // Test with permit2 flag set
//     let traits_permit2 = MakerTraitsBuilder::new(env.clone()).use_permit2().build();

//     assert!(MakerTraitsLib::use_permit2(env.clone(), traits_permit2));
// }

#[test]
fn test_unwrap_weth() {

    let env = create_test_env();

    // Test default (no unwrap)
    let traits_default = TakerTraitsBuilder::new(env.clone()).build();

    // assert!(!TakerTraitsLib::unwrap_weth(env.clone(), traits_default));

    //     // Test with unwrap WETH flag set
    //     let traits_unwrap = TakerTraitsBuilder::new(env.clone()).unwrap_weth().build();

    //     assert!(TakerTraitsLib::unwrap_weth(env.clone(), traits_unwrap));
}

// #[test]
// fn test_maker_traits_builder_default() {
//     let env = create_test_env();

//     // Test default builder
//     let default_traits = MakerTraitsBuilder::new(env.clone()).build();

//     // Verify all defaults
//     assert!(!MakerTraitsLib::has_extension(
//         env.clone(),
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::is_expired(&env, default_traits.clone()));
//     assert_eq!(
//         MakerTraitsLib::nonce_or_epoch(&env, default_traits.clone()),
//         0
//     );
//     assert_eq!(MakerTraitsLib::series(&env, default_traits.clone()), 0);
//     assert!(MakerTraitsLib::allow_partial_fills(
//         &env,
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::need_pre_interaction_call(
//         &env,
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::need_post_interaction_call(
//         &env,
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::allow_multiple_fills(
//         &env,
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::need_check_epoch_manager(
//         &env,
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::use_permit2(
//         env.clone(),
//         default_traits.clone()
//     ));
//     assert!(!MakerTraitsLib::unwrap_weth(env.clone(), default_traits));
// }

// #[test]
// fn test_edge_cases_zero_values() {
//     let env = create_test_env();

//     // Test with zero values
//     let zero_traits = MakerTraitsBuilder::new(env.clone())
//         .with_allowed_sender(0)
//         .with_expiration(0)
//         .with_nonce_or_epoch(0)
//         .with_series(0)
//         .build();

//     // Zero allowed sender should allow any sender
//     let any_address = create_test_address(&env);
//     assert!(MakerTraitsLib::is_allowed_sender(
//         &env,
//         zero_traits.clone(),
//         any_address
//     ));

//     // Zero expiration should never expire
//     assert!(!MakerTraitsLib::is_expired(&env, zero_traits.clone()));

//     // Zero values should return zero
//     assert_eq!(MakerTraitsLib::nonce_or_epoch(&env, zero_traits.clone()), 0);
//     assert_eq!(MakerTraitsLib::series(&env, zero_traits), 0);
// }

// #[test]
// fn test_bit_invalidator_logic_combinations() {
//     let env = create_test_env();

//     // Case 1: No partial, no multiple - should use bit invalidator
//     let case1 = MakerTraitsBuilder::new(env.clone())
//         .no_partial_fills()
//         .build();
//     assert!(MakerTraitsLib::use_bit_invalidator(&env, case1));

//     // Case 2: Allow partial, no multiple - should use bit invalidator
//     let case2 = MakerTraitsBuilder::new(env.clone()).build(); // default allows partial
//     assert!(MakerTraitsLib::use_bit_invalidator(&env, case2));

//     // Case 3: No partial, allow multiple - should use bit invalidator
//     let case3 = MakerTraitsBuilder::new(env.clone())
//         .no_partial_fills()
//         .allow_multiple_fills()
//         .build();
//     assert!(MakerTraitsLib::use_bit_invalidator(&env, case3));

//     // Case 4: Allow partial, allow multiple - should NOT use bit invalidator
//     let case4 = MakerTraitsBuilder::new(env.clone())
//         .allow_multiple_fills()
//         .build(); // default allows partial
//     assert!(!MakerTraitsLib::use_bit_invalidator(&env, case4));
// }
