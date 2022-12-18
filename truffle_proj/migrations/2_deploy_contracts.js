var DummyZeroEx = artifacts.require("DummyZeroEx.sol");
module.exports = function(deployer) {
   deployer.deploy(DummyZeroEx);
};