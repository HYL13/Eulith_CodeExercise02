geth --datadir node01 init genesis.json

geth --nousb --identity "node01" --networkid 109 --datadir node01 --http --http.addr 0.0.0.0 --http.port 23456 --http.api admin,debug,web3,eth,txpool,personal,ethash,miner,net --http.corsdomain "*" --port 30303 --authrpc.port 8545 --allow-insecure-unlock

personal.unlockAccount(eth.accounts[0], "123", 0)

DummyZeroEx.deployed().then(function(instance) {return instance.transformERC20.call("BTC","ETH",1000, 500)})