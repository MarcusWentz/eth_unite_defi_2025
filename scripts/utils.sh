derive_secret_and_hashlock() {
    local config_secret="$1"
    
    SECRET=$(echo -n "$config_secret" | cast keccak)
    HASHLOCK=$(echo -n "$SECRET" | xxd -r -p | cast keccak)
    
    echo "Config secret: $config_secret"
    echo "Derived secret: $SECRET"
    echo "Derived hashlock: $HASHLOCK"
}

timelocks_init_bc() {
    local src_withdrawal="$1"
    local src_public_withdrawal="$2"
    local src_cancellation="$3"
    local src_public_cancellation="$4"
    local dst_withdrawal="$5"
    local dst_public_withdrawal="$6"
    local dst_cancellation="$7"
    local deployed_at="$8"
    
    # Calculate each shifted value using bc
    local deployed_at_shifted=$(echo "$deployed_at * 2^224" | bc)
    local src_withdrawal_shifted=$(echo "$src_withdrawal * 2^0" | bc)  # Stage 0
    local src_public_withdrawal_shifted=$(echo "$src_public_withdrawal * 2^32" | bc)  # Stage 1
    local src_cancellation_shifted=$(echo "$src_cancellation * 2^64" | bc)  # Stage 2
    local src_public_cancellation_shifted=$(echo "$src_public_cancellation * 2^96" | bc)  # Stage 3
    local dst_withdrawal_shifted=$(echo "$dst_withdrawal * 2^128" | bc)  # Stage 4
    local dst_public_withdrawal_shifted=$(echo "$dst_public_withdrawal * 2^160" | bc)  # Stage 5
    local dst_cancellation_shifted=$(echo "$dst_cancellation * 2^192" | bc)  # Stage 6
    
    # Sum all the shifted values
    echo "$deployed_at_shifted + $src_withdrawal_shifted + $src_public_withdrawal_shifted + $src_cancellation_shifted + $src_public_cancellation_shifted + $dst_withdrawal_shifted + $dst_public_withdrawal_shifted + $dst_cancellation_shifted" | bc
}