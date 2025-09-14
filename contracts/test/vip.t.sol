// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "forge-std/src/Test.sol";
import {Constant} from "./Constant.t.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {UniversalRouter, CallbackType} from "../contracts/UniversalRouter.sol";
import {UniversalRouterCustomFee} from "../contracts/UniversalRouterCustomFee.sol";
import {IQuoterV2} from "./IQuoterV2.sol";
import {console2} from "forge-std/src/console2.sol";

contract ForkUniversalDexLoopingHookTest is Constant, Test {
    IStakePool public stakePool =
        IStakePool(0xf6701A6A20639f0E765bA7FF66FD4f49815F1a27);
    IQuoterV2 public quoter =
        IQuoterV2(0xe8CabF9d1FFB6CE23cF0a86641849543ec7BD7d5);

    function setUp() public {
        vm.createSelectFork(
            "https://sly-lively-water.story-mainnet.quiknode.pro/2cb2f586bc9ac68d8b0c29e46a6005abd5f0425e",
            7701509
        ); // Fork from mainnet
    }

    function testLoopp() public {
        for (uint256 i = 1; i <= 15; i++) {
            uint256 amountIn = 10 * i * 1e18;
            console2.log("amountIn", amountIn);
            uint256 vip = stakePool.calculateVIPMint(amountIn);
            console2.log("vip", vip);
            (
                uint256 amountOut,
                uint160 sqrtPriceX96After,
                uint32 initializedTicksCrossed,
                uint256 gasEstimate
            ) = quoter.quoteExactInputSingle(
                    IQuoterV2.QuoteExactInputSingleParams({
                        tokenIn: address(
                            0x5267F7eE069CEB3D8F1c760c215569b79d0685aD
                        ),
                        tokenOut: address(
                            0x1514000000000000000000000000000000000000
                        ),
                        amountIn: vip,
                        fee: 500,
                        sqrtPriceLimitX96: 0
                    })
                );
            console2.log("amountOut", amountOut);
            console2.log(amountOut > amountIn);
            if (amountOut > amountIn) {
                console2.log("change", amountOut - amountIn);
            }
        }
    }
}

/// @title Stake Pool Interface
/// @notice Interface for the main staking pool contract that handles IP to vIP conversions
/// @dev Manages staking, unstaking, and configuration of the staking pool
interface IStakePool {
    /// @notice Configuration structure for the stake pool
    /// @dev Contains all configurable parameters
    /// @custom:storage-location erc7201:VerioIP.StakePool.StakePoolState
    struct StakePoolState {
        uint256 singularity;
        uint256 minStake;
        uint256 maxStakeIterations;
        uint256 maxStake;
        uint256 delegationSize;
        uint256 unbondingPeriod;
        uint256 mintFeeBps;
        uint256 burnFeeBps;
        uint256 pendingUnstake;
        uint256 excessUnstake;
        bool mintFeeEnabled;
        bool burnFeeEnabled;
    }

    /// @notice Configuration structure for the stake pool
    /// @dev Contains all configurable parameters
    struct StakePoolConfig {
        uint256 singularity;
        uint256 minStake;
        uint256 maxStakeIterations;
        uint256 maxStake;
        uint256 delegationSize;
        uint256 unbondingPeriod;
        uint256 mintFeeBps;
        uint256 burnFeeBps;
        bool mintFeeEnabled;
        bool burnFeeEnabled;
    }

    /// @notice Emitted when a user stakes IP
    /// @param staker Address of the staking user
    /// @param amount Amount of IP staked
    event Staked(address staker, uint256 amount);

    /// @notice Emitted when a user unstakes vIP
    /// @param staker Address of the unstaking user
    /// @param amount Amount of IP unstaked
    event Unstaked(address staker, uint256 amount);

    /// @notice Emitted when rewards are received
    /// @param amount Amount of rewards received
    event RewardReceived(uint256 amount);

    /// @notice Emitted when the stake pool is updated
    /// @param stakePoolState The new stake pool state
    event StakePoolStateUpdated(StakePoolState stakePoolState);

    /// @notice Error when singularity is invalid
    error InvalidSingularity();

    /// @notice Error when max stake iterations is invalid
    error InvalidMaxStakeIterations();

    /// @notice Error when delegation size is invalid
    error InvalidDelegationSize();

    /// @notice Error when min stake is invalid
    error InvalidMinStake();

    /// @notice Error when mint fee exceeds maximum
    /// @param maxMintFeeBps The maximum mint fee in basis points
    error MaxMintFeeExceeded(uint256 maxMintFeeBps);

    /// @notice Error when burn fee exceeds maximum
    /// @param maxBurnFeeBps The maximum burn fee in basis points
    error MaxBurnFeeExceeded(uint256 maxBurnFeeBps);

    /// @notice Error when max stake is invalid
    error InvalidMaxStake();

    /// @notice Error when stake amount is not a multiple of 1 gwei
    error StakeNotGweiMultiple();

    /// @notice Error when stake is less than minimum
    /// @param minStake The minimum stake amount
    error StakeLessThanMinimum(uint256 minStake);

    /// @notice Error when admin stake is less than delegation size
    /// @param delegationSize The minimum delegation size
    error AdminStakeLessThanDelegationSize(uint256 delegationSize);

    /// @notice Error when unstaking before singularity
    /// @param singularity The singularity block height
    error CannotUnstakeBeforeSingularity(uint256 singularity);

    /// @notice Error when unstaking 0 IP
    error CannotUnstakeZeroIP();

    /// @notice Funds the contract with native currency
    function fund() external payable;

    /// @notice Stakes native currency and receives vIP tokens
    /// @param _evaluate Whether to evaluate the stakePool immediately
    function stake(bool _evaluate) external payable;

    /// @notice Evaluates the stake pool and updates the state
    function evaluateStakePool() external;

    /// @notice Allows admin to stake with specific validator and maturity
    /// @param _validator Validator identification bytes
    /// @param _maturity Staking period enum value
    function adminStake(
        bytes calldata _validator,
        IIPTokenStaking.StakingPeriod _maturity
    ) external payable;

    /// @notice Triggers a reward claim and restaking process
    function restake() external;

    /// @notice Unstakes vIP tokens and initiates withdrawal
    /// @param _vIPAmount Amount of vIP to unstake
    function unstake(uint256 _vIPAmount) external;

    /// @notice Calculates amount of vIP to mint for a given IP amount
    /// @param _ipAmount Amount of IP to calculate vIP for
    /// @return Amount of vIP that would be minted
    function calculateVIPMint(
        uint256 _ipAmount
    ) external view returns (uint256);

    /// @notice Calculates amount of IP to withdraw for a given vIP amount
    /// @param _vIPToBurn Amount of vIP to calculate IP withdrawal for
    /// @return Amount of IP that would be withdrawn
    function calculateIPWithdrawal(
        uint256 _vIPToBurn
    ) external view returns (uint256);

    /// @notice Gets the current balance of the stake pool
    /// @return Current balance in native currency
    function getStakePoolAmount() external view returns (uint256);

    /// @notice Gets the total amount of IP staked
    /// @return Total amount of IP staked across all validators
    function getTotalStake() external view returns (uint256);

    /// @notice Gets the amount of unstaked IP available
    /// @return Amount of excess IP from unstaking operations
    function getUnstaked() external view returns (uint256);

    /// @notice Gets the minimum stake amount required
    /// @return Minimum amount of IP that can be staked
    function getMinStake() external view returns (uint256);

    /// @notice Sets the minimum stake amount
    /// @param _minStake New minimum stake amount
    /// @dev Only callable by admin
    function setMinStake(uint256 _minStake) external;

    /// @notice Gets the current mint fee in basis points
    /// @return Current mint fee (100 = 1%)
    function getMintFeeBps() external view returns (uint256);

    /// @notice Sets the mint fee in basis points
    /// @param _mintFeeBps New mint fee (100 = 1%)
    /// @dev Only callable by admin
    function setMintFeeBps(uint256 _mintFeeBps) external;

    /// @notice Gets the current burn fee in basis points
    /// @return Current burn fee (100 = 1%)
    function getBurnFeeBps() external view returns (uint256);

    /// @notice Sets the burn fee in basis points
    /// @param _burnFeeBps New burn fee (100 = 1%)
    /// @dev Only callable by admin
    function setBurnFeeBps(uint256 _burnFeeBps) external;

    /// @notice Gets the unbonding period
    /// @return Unbonding period in seconds
    function getUnbondingPeriod() external view returns (uint256);

    /// @notice Sets the unbonding period
    /// @param _unbondingPeriod New unbonding period in seconds
    /// @dev Only callable by admin
    function setUnbondingPeriod(uint256 _unbondingPeriod) external;

    /// @notice Gets the pending unstake amount
    /// @return Pending unstake amount
    function getPendingUnstake() external view returns (uint256);

    /// @notice Checks if mint fee is enabled
    /// @return True if mint fee is enabled, false otherwise
    function getMintFeeEnabled() external view returns (bool);

    /// @notice Enables or disables mint fee
    /// @param _mintFeeEnabled True to enable mint fee, false to disable
    /// @dev Only callable by admin
    function setMintFeeEnabled(bool _mintFeeEnabled) external;

    /// @notice Checks if burn fee is enabled
    /// @return True if burn fee is enabled, false otherwise
    function getBurnFeeEnabled() external view returns (bool);

    /// @notice Enables or disables burn fee
    /// @param _burnFeeEnabled True to enable burn fee, false to disable
    /// @dev Only callable by admin
    function setBurnFeeEnabled(bool _burnFeeEnabled) external;

    /// @notice Gets the minimum delegation size
    /// @return Minimum amount required for delegation
    function getDelegationSize() external view returns (uint256);

    /// @notice Sets the minimum delegation size
    /// @param _delegationSize New minimum delegation size
    /// @dev Only callable by admin
    function setDelegationSize(uint256 _delegationSize) external;

    /// @notice Gets the maximum stake iterations
    /// @return Maximum stake iterations
    function getMaxStakeIterations() external view returns (uint256);

    /// @notice Sets the maximum stake iterations
    /// @param _maxStakeIterations New maximum stake iterations
    /// @dev Only callable by admin
    function setMaxStakeIterations(uint256 _maxStakeIterations) external;

    /// @notice Gets the maximum stake amount
    /// @return Maximum stake amount
    function getMaxStake() external view returns (uint256);

    /// @notice Sets the maximum stake amount
    /// @param _maxStake New maximum stake amount
    /// @dev Only callable by admin
    function setMaxStake(uint256 _maxStake) external;
}

/// @title IIPTokenStaking
/// @notice Interface for the IPTokenStaking contract
interface IIPTokenStaking {
    /// @notice Enum representing the different staking periods
    /// @dev FLEXIBLE is used for flexible staking, where the staking period is not fixed and can be changed by the user
    /// SHORT, MEDIUM, and LONG are used for staking with specific periods
    enum StakingPeriod {
        FLEXIBLE,
        SHORT,
        MEDIUM,
        LONG
    }

    /// @notice Returns the rounded stake amount and the remainder.
    /// @param rawAmount The raw stake amount.
    /// @return amount The rounded stake amount.
    /// @return remainder The remainder of the stake amount.
    function roundedStakeAmount(
        uint256 rawAmount
    ) external view returns (uint256 amount, uint256 remainder);

    /// @notice Sets an operator for a delegator.
    /// Calling this method will override any existing operator.
    /// @param operator The operator address to add.
    function setOperator(address operator) external payable;

    /// @notice Removes current operator for a delegator.
    function unsetOperator() external payable;

    /// @notice Set/Update the withdrawal address that receives the withdrawals.
    /// Charges fee (CL spam prevention). Must be exact amount.
    /// @param newWithdrawalAddress EVM address to receive the  withdrawals.
    function setWithdrawalAddress(
        address newWithdrawalAddress
    ) external payable;

    /// @notice Set/Update the withdrawal address that receives the stake and reward withdrawals.
    /// Charges fee (CL spam prevention). Must be exact amount.
    /// @param newRewardsAddress EVM address to receive the stake and reward withdrawals.
    function setRewardsAddress(address newRewardsAddress) external payable;

    /// @notice Entry point to stake (delegate) to the given validator. The consensus client (CL) is notified of
    /// the deposit and manages the stake accounting and validator onboarding. Payer must be the delegator.
    /// @dev Staking burns tokens in Execution Layer (EL). Unstaking (withdrawal) will trigger minting through
    /// withdrawal queue.
    /// @param validatorCmpPubkey 33 bytes compressed secp256k1 public key.
    /// @param stakingPeriod The staking period.
    /// @param data Additional data for the stake.
    /// @return delegationId The delegation ID, always 0 for flexible staking.
    function stake(
        bytes calldata validatorCmpPubkey,
        StakingPeriod stakingPeriod,
        bytes calldata data
    ) external payable returns (uint256 delegationId);

    /// @notice Entry point for redelegating the stake to another validator.
    /// Charges fee (CL spam prevention). Must be exact amount.
    /// @dev For non flexible staking, your staking period will continue as is.
    /// @dev For locked tokens, this will fail in CL if the validator doesn't support unlocked staking.
    /// @param validatorSrcCmpPubkey 33 bytes compressed secp256k1 public key.
    /// @param validatorDstCmpPubkey 33 bytes compressed secp256k1 public key.
    /// @param delegationId The delegation ID, 0 for flexible staking.
    /// @param amount The amount of stake to redelegate.
    function redelegate(
        bytes calldata validatorSrcCmpPubkey,
        bytes calldata validatorDstCmpPubkey,
        uint256 delegationId,
        uint256 amount
    ) external payable;

    /// @notice Entry point for unstaking the previously staked token.
    /// @dev Unstake (withdrawal) will trigger native minting, so token in this contract is considered as burned.
    /// Charges fee (CL spam prevention). Must be exact amount.
    /// @param validatorCmpPubkey 33 bytes compressed secp256k1 public key.
    /// @param delegationId The delegation ID, 0 for flexible staking.
    /// @param amount Token amount to unstake.
    /// @param data Additional data for the unstake.
    function unstake(
        bytes calldata validatorCmpPubkey,
        uint256 delegationId,
        uint256 amount,
        bytes calldata data
    ) external payable;

    /// @notice Returns the minimum stake amount.
    /// @return The minimum stake amount.
    function minStakeAmount() external view returns (uint256);

    /// @notice Returns the fee charged for payable functions such as unstake, redelegate, etc.
    /// @return The fee amount.
    function fee() external view returns (uint256);
}
