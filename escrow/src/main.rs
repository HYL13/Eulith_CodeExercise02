use clap::{Parser, ValueEnum};
use clap_num::maybe_hex;
use std::str::FromStr;
use std::time;

use web3::{
    contract::{Contract, Options},
    types::{Address, TransactionRequest},
};

async fn deploy() -> web3::contract::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().await?;

    // Get the contract bytecode for instance from Solidity compiler
    let bytecode = include_str!("./contracts/Escrow_sol_Escrow.bin");

    println!("Start to deploy!");

    // Deploying a contract
    let contract = Contract::deploy(
        web3.eth(),
        include_bytes!("./contracts/Escrow_sol_Escrow.abi"),
    )?
    .confirmations(1)
    .poll_interval(time::Duration::from_secs(10))
    .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
    .execute(bytecode, (), accounts[0])
    .await?;

    println!("Deployed at: {:#x}", contract.address());

    Ok(())
}

async fn deposit(sender: String, contract: String, amount: u128) -> web3::Result {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    // Insert the 20-byte "from" address in hex format (prefix with 0x)
    let from = Address::from_str(&sender).unwrap();

    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let to = Address::from_str(&contract).unwrap();

    // Build the tx object
    let tx_object = TransactionRequest {
        from,
        to: Some(to),
        value: Some(amount.into()),
        ..Default::default()
    };

    // Send the tx to localhost
    let result = web3.eth().send_transaction(tx_object).await?;
    println!("Tx succeeded with hash: {}", result);
    Ok(())
}

async fn cancel(sender: String, contract: String) -> web3::Result {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let sender: Address = Address::from_str(&*sender).unwrap();
    let contract: Address = Address::from_str(&*contract).unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        contract,
        include_bytes!("./contracts/Escrow_sol_Escrow.abi"),
    )
    .unwrap();

    let tx = contract
        .call("cancelTransaction", (), sender, Options::default())
        .await
        .unwrap();
    println!("TxHash: {}", tx);
    Ok(())
}

async fn balance(account: String) -> web3::Result<()> {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let balance = web3.eth().balance(account.parse().unwrap(), None).await?;
    println!("Balance of {:?}: {}", account, balance);
    Ok(())
}

async fn withdraw(sender: String, contract: String, passcode: String) -> web3::Result {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let sender: Address = Address::from_str(&*sender).unwrap();
    let contract: Address = Address::from_str(&*contract).unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        contract,
        include_bytes!("./contracts/Escrow_sol_Escrow.abi"),
    )
    .unwrap();

    let tx = contract
        .call("validate", (passcode,), sender, Options::default())
        .await
        .unwrap();
    println!("TxHash: {}", tx);
    Ok(())
}

async fn configure(
    sender: String,
    contract: String,
    recipient: String,
    passcode: String,
) -> web3::Result {
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);
    let sender: Address = Address::from_str(&*sender).unwrap();
    let contract: Address = Address::from_str(&*contract).unwrap();
    let recipient: Address = Address::from_str(&*recipient).unwrap();

    let contract = Contract::from_json(
        web3.eth(),
        contract,
        include_bytes!("./contracts/Escrow_sol_Escrow.abi"),
    )
    .unwrap();

    let tx = contract
        .call(
            "setTerms",
            (passcode, recipient),
            sender,
            Options::default(),
        )
        .await
        .unwrap();
    println!("TxHash: {}", tx);
    Ok(())
}

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "This is CLI tool for CodeExercise02"
)]
struct Cli {
    /// What mode to run the program in
    #[arg(short, long, value_enum)]
    mode: Mode,

    /// sender address or caller of the functions in the contract
    #[arg(long)]
    sender: Option<String>,

    /// recipient address
    #[arg(long)]
    recipient: Option<String>,

    /// recipient passcode (necessary when recipient tries to retrieve Ether from escrow)
    #[arg(long)]
    passcode: Option<String>,

    /// Ethr amount
    #[arg(long, value_parser = maybe_hex::< u128 >)]
    amount: Option<u128>,

    /// contract address
    #[arg(long)]
    contract: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Deploy escrow contract to local blockchain and returns
    Deploy,

    /// Check the balance of a give address
    Balance,

    /// Deposit Ether into escrow/contract
    Deposit,

    /// Set passcode and recipient address
    Configure,

    /// Cancel the current transaction at escrow/contract
    Cancel,

    /// Receiver withdraws Ether from escrow/contract if passcode validation is successful
    Withdraw,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Deposit => {
            assert!(
                cli.sender.is_some() && cli.contract.is_some() && cli.amount.is_some(),
                "Please make sure sender, contract and amount is not empty!"
            );
            deposit(
                cli.sender.unwrap(),
                cli.contract.unwrap(),
                cli.amount.unwrap(),
            )
            .await
            .expect("Deposit panic!");
        }
        Mode::Deploy => {
            deploy().await.expect("Deploy panic!");
        }
        Mode::Balance => {
            assert!(
                cli.sender.is_some(),
                "Please make sure sender (the actual address) is not empty!"
            );
            balance(cli.sender.unwrap()).await.expect("Balance failed!");
        }
        Mode::Configure => {
            assert!(
                cli.sender.is_some()
                    && cli.contract.is_some()
                    && cli.passcode.is_some()
                    && cli.recipient.is_some(),
                "Please make sure sender, contract, recipient and passcode is not empty!"
            );
            configure(
                cli.sender.unwrap(),
                cli.contract.unwrap(),
                cli.recipient.unwrap(),
                cli.passcode.unwrap(),
            )
            .await
            .expect("Configure panic!");
        }
        Mode::Cancel => {
            assert!(
                cli.sender.is_some() && cli.contract.is_some(),
                "Please make sure sender, contract is not empty!"
            );
            cancel(cli.sender.unwrap(), cli.contract.unwrap())
                .await
                .expect("Cancel failed!");
        }
        Mode::Withdraw => {
            assert!(
                cli.sender.is_some() && cli.contract.is_some() && cli.passcode.is_some(),
                "Please make sure sender, contract and passcode is not empty!"
            );
            withdraw(
                cli.sender.unwrap(),
                cli.contract.unwrap(),
                cli.passcode.unwrap(),
            )
            .await
            .expect("Withdraw panic!");
        }
    }
}
