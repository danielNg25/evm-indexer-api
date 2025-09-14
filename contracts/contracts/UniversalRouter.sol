// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import {Ownable2Step, Ownable} from "@openzeppelin/contracts/access/Ownable2Step.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import {Pool} from "./libraries/Pool.sol";
import {UniswapV2Library} from "./libraries/UniswapV2Library.sol";
import {BytesLib} from "./libraries/BytesLib.sol";

import {IHookConfig, CallbackType} from "./interfaces/IHookConfig.sol";

import {IUniswapV2Pair} from "./interfaces/UniswapV2/IUniswapV2Pair.sol";
import {IUniswapV3PoolActions} from "./interfaces/UniswapV3/IUniswapV3PoolActions.sol";
import {IUniswapV3PoolImmutables} from "./interfaces/UniswapV3/IUniswapV3PoolImmutables.sol";

struct CallbackData {
    bytes path;
    address recipientCached;
    uint256 amountInCached;
}

contract UniversalRouter is IHookConfig, Ownable2Step {
    using BytesLib for bytes;
    using Pool for bytes;
    using UniswapV2Library for IUniswapV2Pair;

    int24 internal constant MIN_POINT = -887_272;

    int24 internal constant MAX_POINT = -MIN_POINT;

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4_295_128_739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO =
        1_461_446_703_485_210_103_287_273_052_203_988_822_378_723_970_342;
    /// @dev Transient storage variable used to check a safety condition in exact output swaps.
    // uint256 private amountInCached;

    mapping(bytes4 => CallbackType) public approvedCallbackSigs;

    constructor(
        bytes4[] memory _sigs,
        CallbackType[] memory _callbackTypes
    ) Ownable(msg.sender) {
        for (uint256 i = 0; i < _sigs.length; i++) {
            approvedCallbackSigs[_sigs[i]] = _callbackTypes[i];
        }
    }

    function setApprovedSig(
        bytes4 sig,
        CallbackType callbackType
    ) external onlyOwner {
        require(
            approvedCallbackSigs[sig] != callbackType,
            "HookConfig: sig already approved"
        );
        approvedCallbackSigs[sig] = callbackType;

        emit SetApprovedSig(sig, callbackType);
    }

    struct SwapCallbackData {
        bool isExactIn;
    }

    function swap(uint256 amountOut, bytes memory path) external {
        exactOutputInternal(
            amountOut,
            address(this),
            CallbackData({
                path: path,
                recipientCached: address(this),
                amountInCached: 0
            })
        );
    }

    /// @dev Performs a single exact output swap
    function exactOutputInternal(
        uint256 amountOut,
        address recipient,
        CallbackData memory data
    ) private returns (uint256 amountIn) {
        (address pool, address tokenIn) = data.path.decodeFirstPool();

        bool zeroForOne = tokenIn < IUniswapV3PoolImmutables(pool).token1();
        try IUniswapV3PoolImmutables(pool).liquidity() {
            (int256 amount0Delta, int256 amount1Delta) = IUniswapV3PoolActions(
                pool
            ).swap(
                    recipient,
                    zeroForOne,
                    -int256(amountOut),
                    zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
                    abi.encode(data)
                );

            uint256 amountOutReceived;
            (amountIn, amountOutReceived) = zeroForOne
                ? (uint256(amount0Delta), uint256(-amount1Delta))
                : (uint256(amount1Delta), uint256(-amount0Delta));
            // it's technically possible to not receive the full output amount,
            // so if no price limit has been specified, require this possibility away
            require(amountOutReceived == amountOut);
        } catch {
            amountIn = IUniswapV2Pair(pool).getAmountIn(tokenIn, amountOut);
            data.recipientCached = recipient;
            data.amountInCached = amountIn;
            // if not v3, try to swap using V2
            IUniswapV2Pair(pool).swap(
                zeroForOne ? 0 : amountOut,
                zeroForOne ? amountOut : 0,
                address(this),
                abi.encode(data)
            );
        }
    }

    function swapV2Callback(
        address,
        uint256 amount0,
        uint256 amount1,
        bytes memory _data
    ) internal {
        CallbackData memory data = abi.decode(_data, (CallbackData));
        (address pool, address tokenIn) = data.path.decodeFirstPool();

        if (data.recipientCached != address(this))
            if (amount0 > 0) {
                IERC20(IUniswapV2Pair(pool).token0()).transfer(
                    data.recipientCached,
                    amount0
                );
            } else {
                IERC20(IUniswapV2Pair(pool).token1()).transfer(
                    data.recipientCached,
                    amount1
                );
            }

        uint256 amountToPay = data.amountInCached;

        _proccessCallback(tokenIn, amountToPay, data);
    }

    function swapV3Callback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes memory _data
    ) internal {
        require(amount0Delta > 0 || amount1Delta > 0); // swaps entirely within 0-liquidity regions are not supported
        CallbackData memory data = abi.decode(_data, (CallbackData));
        (, address tokenIn) = data.path.decodeFirstPool();

        uint256 amountToPay = amount0Delta > 0
            ? uint256(amount0Delta)
            : uint256(amount1Delta);

        _proccessCallback(tokenIn, amountToPay, data);
    }

    function _proccessCallback(
        address token,
        uint256 amountToPay,
        CallbackData memory data
    ) internal {
        // either initiate the next swap or pay
        if (data.path.hasMultiplePools()) {
            data.path = data.path.skipPool();
            exactOutputInternal(amountToPay, msg.sender, data);
        } else {
            IERC20(token).transfer(msg.sender, amountToPay);
            IERC20(token).transfer(
                owner(),
                IERC20(token).balanceOf(address(this))
            );
        }
    }

    // Check if the function call is approve callback from dexes
    // This design allow to add more approve callback in the future
    // since each dexes has different callback function signature
    function _fallback() internal {
        CallbackType _callbackType = approvedCallbackSigs[msg.sig];
        if (_callbackType == CallbackType.SwapV3) {
            (int256 amount0Delta, int256 amount1Delta, bytes memory data) = (
                abi.decode(msg.data[4:], (int256, int256, bytes))
            );
            swapV3Callback(amount0Delta, amount1Delta, data);
        } else if (_callbackType == CallbackType.SwapV2) {
            (
                address sender,
                uint256 amount0,
                uint256 amount1,
                bytes memory data
            ) = (abi.decode(msg.data[4:], (address, uint256, uint256, bytes)));
            swapV2Callback(sender, amount0, amount1, data);
        }
    }

    fallback() external payable virtual {
        _fallback();
    }

    function withdraw(address token, uint256 amount) external onlyOwner {
        IERC20(token).transfer(msg.sender, amount);
    }

    receive() external payable {
        // solhint-disable-previous-line no-empty-blocks
    }
}
