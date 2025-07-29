run_stages() {
  for STAGE in "${STAGES[@]}"; do
    # Set next block timestamp for time-dependent stages
    if [[ "$CHAIN_ID" == "31337" ]]; then
        case "$STAGE" in
        withdrawSrc)
            NEXT_TS=$((ANVIL_DEPLOY_SRC_TIMESTAMP + WITHDRAWAL_SRC_TIMELOCK + TIME_DELTA))
            echo "Setting next block timestamp for withdrawSrc: $NEXT_TS"
            cast rpc anvil_setNextBlockTimestamp "$NEXT_TS" --rpc-url "$RPC_URL"
            ;;
        withdrawDst)
            NEXT_TS=$((ANVIL_DEPLOY_DST_TIMESTAMP + WITHDRAWAL_DST_TIMELOCK + TIME_DELTA))
            echo "Setting next block timestamp for withdrawDst: $NEXT_TS"
            cast rpc anvil_setNextBlockTimestamp "$NEXT_TS" --rpc-url "$RPC_URL"
            ;;
        cancelSrc)
            NEXT_TS=$((ANVIL_DEPLOY_SRC_TIMESTAMP + CANCELLATION_SRC_TIMELOCK + TIME_DELTA))
            echo "Setting next block timestamp for cancelSrc: $NEXT_TS"
            cast rpc anvil_setNextBlockTimestamp "$NEXT_TS" --rpc-url "$RPC_URL"
            ;;
        cancelDst)
            NEXT_TS=$((ANVIL_DEPLOY_DST_TIMESTAMP + CANCELLATION_DST_TIMELOCK + TIME_DELTA))
            echo "Setting next block timestamp for cancelDst: $NEXT_TS"
            cast rpc anvil_setNextBlockTimestamp "$NEXT_TS" --rpc-url "$RPC_URL"
            ;;
        esac
    fi

    MODE=$STAGE forge script "$SCRIPT_PATH" --broadcast --rpc-url "$RPC_URL" --root packages/1inch-ref

    case "$STAGE" in
        deployEscrowSrc)
          ANVIL_DEPLOY_SRC_TIMESTAMP=$(cast block latest --rpc-url "$RPC_URL" | grep timestamp | awk '{print $2}')
          echo "New anvil deploy src timestamp: $ANVIL_DEPLOY_SRC_TIMESTAMP"
          ;;
        deployEscrowDst)
          deployEscrowDstStellar()
    esac

    echo -n "Continue to the next stage? [y/n]:" 
    read answer
    if [[ "$answer" == "n" || "$answer" == "N" ]]; then
        echo "Exiting script."
        break
    fi

    sleep 1
  done
}