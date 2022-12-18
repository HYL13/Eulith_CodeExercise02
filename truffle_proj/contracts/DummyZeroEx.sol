// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface ITransformERC20Feature {
    function transformERC20(
        string calldata inputToken, // I replaced the IERC20TokenV06 with string so I didn't have to copy extra classes
        string calldata outputToken,
        uint256 inputTokenAmount,
        uint256 minOutputTokenAmount
    ) external payable returns (uint256 outputTokenAmount);
}

// This exercice focuses on transformERC20 function, so I let IZeroEx inherit ITransformERC20Feature only
interface IZeroEx is
    ITransformERC20Feature
{
   
}

contract DummyZeroEx is IZeroEx{
     function transformERC20(
        string calldata inputToken,
        string calldata outputToken,
        uint256 inputTokenAmount,
        uint256 minOutputTokenAmount
    ) external payable returns (uint256 outputTokenAmount){
        return 2023;
    }
}
