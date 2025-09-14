// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.25;

enum CallbackType {
    None,
    SwapV2,
    SwapV3
}

interface IHookConfig {
    event SetApprovedSig(bytes4 sig, CallbackType callbackType);

    function approvedCallbackSigs(bytes4 sig) external view returns (CallbackType);

    function setApprovedSig(bytes4 sig, CallbackType callbackType) external;
}
