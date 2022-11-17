const { saveContractAddress } = require("./utils");

async function main() {
  const Board = await ethers.getContractFactory("Board");
  const board = await Board.deploy();
  await board.deployed();
  console.log("Board deployed at : ", board.address);
  saveContractAddress("Board", board.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
