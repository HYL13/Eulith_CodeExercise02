# Eulith_CodeExercise02













I added contract because I don't want to cause confusion

no testing in solidity, manually check. willing to add more test coverage

rust not good, module system? put everything into one file

I don't know how to catch error messages from output of local blockchain

return Result(ok()) issues
tools I use:


### CLI tool usage:
Compile the contract:
```
solcjs --abi --bin Escrow.sol
```

Local blockchain setup and deploy the contract escrow:
```
// ganache-cli generates 10 accounts, each has 1000 ETH
ganache-cli -b 3 -m "hamster coin cup brief quote trick stove draft hobby strong caught unable"

cargo run --  --mode deploy 

// accounts[0]:             "0xd028d24f16a8893bd078259d413372ac01580769"
// accounts[1]:             "0x75df5695686338883675bb27bd06fc7578aa01b7"
// Contract is deployed at: "0x4d470146215d085c75767b717dbb8d3b4468f893"
```
Check the balance at a given address (use sender param):
```
cargo run --  --mode balance --sender "0x75df5695686338883675bb27bd06fc7578aa01b7" 
```

Deposit 5 ETH (5e18) from accounts[0] to contract:
```
cargo run --  --mode deposit --sender "0xd028d24f16a8893bd078259d413372ac01580769" --contract  "0x4d470146215d085c75767b717dbb8d3b4468f893"  --amount 5000000000000000000
```

Remember to configure the passcode and recipient after deposit is done:
```
cargo run --  --mode configure --sender "0xd028d24f16a8893bd078259d413372ac01580769" --recipient  "0x75df5695686338883675bb27bd06fc7578aa01b7" --contract  "0x4d470146215d085c75767b717dbb8d3b4468f893" --passcode poqwj
```

Cancel the ongoing transaction at escrow so that the next transaction could continue, notice it should be requested by the same sender:
```
cargo run --  --mode cancel --sender "0xd028d24f16a8893bd078259d413372ac01580769" --contract  "0x4d470146215d085c75767b717dbb8d3b4468f893" 
```


Recipient tries to withdraw Ether from escrow contract, and attach passcode for validation, notice recipient is the caller so I treat it as the sender:
```
cargo run --  --mode withdraw --sender "0x75df5695686338883675bb27bd06fc7578aa01b7"  --contract  "0x4d470146215d085c75767b717dbb8d3b4468f893" --passcode poqwj

```
