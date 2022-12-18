// SPDX-License-Identifier: MIT

pragma solidity ^0.8.13;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/DummyZeroEx.sol";

contract TestDummyZeroEx {
  function testReturnValue() public{
    DummyZeroEx dummyZeroEx = DummyZeroEx(DeployedAddresses.DummyZeroEx());

    uint256 expected = 2023;

    Assert.equal(dummyZeroEx.transformERC20("BTC","ETH",1000, 500), expected, "transformERC20 should return 2023");
  }
}