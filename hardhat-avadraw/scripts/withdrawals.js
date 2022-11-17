const { getSavedContractAddresses, getSavedContractABI } = require("./utils");
const hre = require("hardhat");

async function main() {
    let network_name = hre.network.name;
    const abi = getSavedContractABI();
    const network = getSavedContractAddresses()[network_name];
    const address = network.Board;
    const provider = new hre.ethers.providers.JsonRpcProvider(network.url);
    const board = new ethers.Contract(address, abi.abi,provider);
    console.log("========== Withdrawals ==========");
    let account = "0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC";
    let price = await board.pendingWithdrawals(account);
    console.log("Account " + account + " has " + price + " WEI in withdrawals");
    account = "0xE27AbB5392c3163798ce5e0B52dc927F318F825D";
    price = await board.pendingWithdrawals(account);
    console.log("Account " + account + " has " + price + " WEI in withdrawals");
    account = "0x363A10ac202AA7fA4dc2F2867D73d8E3643699dc";
    price = await board.pendingWithdrawals(account);
    console.log("Account " + account + " has " + price + " WEI in withdrawals");
    account = "0x3d337D74851fdE1965e797521f2eE299549c2349";
    price = await board.pendingWithdrawals(account);
    console.log("Account " + account + " has " + price + " WEI in withdrawals");
    account = "0x0000000000000000000000000000000000000000";
    price = await board.pendingWithdrawals(account);
    console.log("Account " + account + " has " + price + " WEI in withdrawals");
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
