// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
    I make the assumption that the escrow could only handle one transaction at a time,
    but the sender has the freedom to cancel the current transaction and get the Ether back,
    so that the next sender could use the escrow service.
 */
contract Escrow {
    bool public transactionStatus; // true means currently there's an ongoing transaction
    string public passcode;
    address public sender;
    address public recipient;

    receive() external payable {
        require(
            transactionStatus == false,
            "This contract is currently handling a transaction!"
        );
        transactionStatus = true;
        sender = msg.sender;
    }

    // this function should be called by the sender right after it successfully sends Ether to the contract,
    // to correctly set the passcode and recipient address
    function setTerms(string memory _passcode, address _recipient) public {
        require(sender == msg.sender, "Sender doesn't match with the record!");
        passcode = _passcode;
        recipient = _recipient;
    }

    // Send Ether to the recipient and reset the transaction if validation passes
    function validate(string memory _passcode) public {
        require(transactionStatus, "There's no ongoing transaction!");
        require(
            recipient == msg.sender,
            "Recipient doesn't match with the record!"
        );
        require(compare(passcode, _passcode), "Incorrect passcode!");
        send(recipient, getBalance());
        reset();
    }

    // Sender can cancel the ongoing transaction, Ethers are then sent back to the sender and reset the transaction status
    function cancelTransaction() public {
        require(transactionStatus, "There's no ongoing transaction!");
        require(sender == msg.sender, "Sender doesn't match with the record!");
        send(sender, getBalance());
        reset();
    }

    function send(address to, uint256 amount) public payable {
        (bool sent, ) = to.call{value: amount}("");
        require(sent, "Failed to send Ether!");
    }

    // reset the status of the contract for future use
    function reset() public {
        transactionStatus = false;
        passcode = "";
        sender = address(0);
        recipient = address(0);
    }

    // Looks like this is the way of comparing strings in Solidity. First transform to bytes,
    // then hash and compare. Early break if the lengths aren't equal to save gas
    function compare(string memory str1, string memory str2)
        public
        pure
        returns (bool)
    {
        if (bytes(str1).length != bytes(str2).length) {
            return false;
        }
        return
            keccak256(abi.encodePacked(str1)) ==
            keccak256(abi.encodePacked(str2));
    }

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}
