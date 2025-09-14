// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.25;

import { BytesLib } from "./BytesLib.sol";

/// @title Functions for manipulating pools data for multihop swaps
library Pool {
    using BytesLib for bytes;

    /// @dev The length of the bytes encoded address
    uint256 private constant ADDR_SIZE = 20;

    /// @dev The offset of an encoded pool key
    uint256 private constant POP_OFFSET = ADDR_SIZE + ADDR_SIZE; // Token in + pool address

    /// @dev The minimum length of an encoding that contains 2 or more pools
    uint256 private constant MULTIPLE_POOLS_MIN_LENGTH = 2 * POP_OFFSET;

    /// @notice Returns true iff the pools contains two or more pools
    /// @param pools The encoded swap pools
    /// @return True if pools contains two or more pools, otherwise false
    function hasMultiplePools(bytes memory pools) internal pure returns (bool) {
        return pools.length >= MULTIPLE_POOLS_MIN_LENGTH;
    }

    /// @notice Returns the number of pools in the pools
    /// @param pools The encoded swap pools
    /// @return The number of pools in the pools
    function numPools(bytes memory pools) internal pure returns (uint256) {
        return pools.length / POP_OFFSET;
    }

    /// @notice Decodes the first pool in pools
    /// @param pools The bytes encoded swap pools
    /// @return pool The pool address
    function decodeFirstPool(bytes memory pools) internal pure returns (address pool, address tokenIn) {
        tokenIn = pools.toAddress(0);
        pool = pools.toAddress(ADDR_SIZE);
    }

    /// @notice Gets the segment corresponding to the first pool in the path
    /// @param path The bytes encoded swap path
    /// @return The segment containing all data necessary to target the first pool in the path
    function getFirstPool(bytes memory path) internal pure returns (bytes memory) {
        return path.slice(0, POP_OFFSET);
    }

    /// @notice Skips a token + fee element from the buffer and returns the remainder
    /// @param pools The swap pools
    /// @return The remaining pools
    function skipPool(bytes memory pools) internal pure returns (bytes memory) {
        return pools.slice(POP_OFFSET, pools.length - POP_OFFSET);
    }
}
