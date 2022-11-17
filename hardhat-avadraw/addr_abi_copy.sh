cp -f contract-addresses.json ../frontend-avadraw/src/web3
cp -f contract-addresses.json ../crawler/web3/
mkdir -p ../crawler/web3/abi/ ../frontend-avadraw/src/web3/abi/
cp -f artifacts/contracts/Board.sol/Board.json ../crawler/web3/abi/Board.json
cp -f artifacts/contracts/Board.sol/Board.json ../frontend-avadraw/src/web3/abi/Board.json
