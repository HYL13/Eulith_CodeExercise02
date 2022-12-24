# Eulith_CodeExercise02

## BackGround
Hi I'm Elay/Yilei, I'm a passionate software engineer. I previously worked at Meta on VRShell (render/window management runtime of Quest headsets) until I was impacted by recent layoff. To do this coding assignment, I studied blockchain (I knew a little about it before), solidity, rust from scratch. I'm really glad to be able to complete this assignment just before holidays. All the commands below are supposed to work and I will write some of the issues with my implementation in a later section. But overall it's a pretty fun assignment to work on (after the update), and I really appreciate this opportunity to interview with you! Feel free to reach out to me through kinghyl13@gmail.com

## Design
### Solidity Contract (Escrow.sol)
I created the contract called Escrow, which is supposed to handle at most one transaction at a time. Escrow is a trusted third party to provide safety guarantee for two trading parties (sender & recipient). The trading process would be as follows:

1. Sender sends ETH to Escrow
2. Sender sets the passcode & recipient to Escrow.
3. (not necessary) Before recipient claims the ETH in Escrow, sender could cancel the transaction and get the ETH back. Escrow does not withhold any ETH.
4. Recipient tries to claim the ETH in Escrow. If it's the matching recipient and passcode is correct, then Escrow releases ETH to the recipient. Transaction is over and Escrow states are reset.

I also put some require() functions in Escrow to do authentication.

### Rust CLI (main.rs)
I came across this beautiful tool called [rust-web3](https://github.com/tomusdrw/rust-web3). It's a rust implementation of Web3.js. I used a lot of example code in my assignment and it worked pretty well. Another tool that I used is called [clap](https://docs.rs/clap/latest/clap/) which is a command tool parser for rust. It made parsing arguments so much easier and significantly reduced my time spent on implementing the rust CLI tool for this assignment.

## CLI tool usage:
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

Deposit 7 ETH (7e18) from accounts[0] to contract:
```
cargo run --  --mode deposit --sender "0xd028d24f16a8893bd078259d413372ac01580769" --contract  "0x4d470146215d085c75767b717dbb8d3b4468f893"  --amount 7000000000000000000
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

## Issues
There are some issues with my implementation here, and I can certainly work on it if I'm asked.
1. No testing for the solidity contract. I manually eye-checked the correctness of the transaction. Besides, I could definitely optimize the contract like using a more appropriate function/variable modifiers.

2. Rust definitely has some steep learning curve. I just put all the functions into one file, the code could certainly be organized better, like putting parser into main.rs and all the other web3 related call into another file. Currently my CLI tool doesn't do well with error handling. Error messages are output to standard output of local blockchain (ganache-cli process) instead of being caught in CLI tool and printed to user terminal. What's more, I'm still learning async/await, since the semantics are a bit different from other languages like JS and C#.