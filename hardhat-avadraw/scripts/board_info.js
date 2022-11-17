const { getSavedContractAddresses, getSavedContractABI } = require("./utils");
const hre = require("hardhat");

async function main() {
    let network_name = hre.network.name;
    const abi = getSavedContractABI();
    const network = getSavedContractAddresses()[network_name];
    const address = network.Board;
    const provider = new hre.ethers.providers.JsonRpcProvider(network.url);
    const board = new ethers.Contract(address, abi.abi,provider);
    console.log("========== Tile info ==========");
    for (var i = 0; i < 100; i++) {
        for (var j = 0; j < 100; j++) {
            const tile = JSON.parse(JSON.stringify(await board.tiles(i,j)));
            console.log("Tile[" + i + "][" + j + "]" , "Price = " + parseInt(tile[0].hex,16), "Owner = " + tile[1]);
        }
    }
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
