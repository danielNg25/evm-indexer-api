// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

error ExactOutput(uint256 amountIn);
error ExactInput(uint256 amountOut);

interface IUniversalQuoter {
    struct QuoteExactOutputParams {
        uint256 expectAmount;
        bytes path;
    }

    function quoteExactOutput(
        QuoteExactOutputParams memory params
    ) external returns (uint256 amountIn);

    struct QuoteExactInputParams {
        uint256 inputAmount;
        bytes path;
    }

    function quoteExactInput(
        QuoteExactInputParams memory params
    ) external returns (uint256 amountOut);
}
