const { expect } = require("chai");

describe("Board", function () {
  const INIT_BASE_PRICE = ethers.utils.parseUnits("10000000", "gwei");
  const INIT_PURCHASE_PRICE = INIT_BASE_PRICE.mul(130).div(100);

  let Board;
  let deployer, alice, bob, charly;
  beforeEach(async function () {
    const accounts = await ethers.getSigners();
    deployer = accounts[0];
    alice = accounts[1];
    bob = accounts[2];
    charly = accounts[3];
    const BoardFactory = await ethers.getContractFactory("Board");
    Board = await BoardFactory.deploy();
    console.log("Board is deployed at: ", Board.address);
  });

  async function buyOneTile() {
    const value = INIT_PURCHASE_PRICE;
    const tx = await Board.connect(alice).buy(
      [0, 0, 1, 1],
      [false, 130],
      "Dogs",
      charly.address,
      { value }
    );

    return tx.wait();
  }

  context("Buy", async function () {
    it("The tile has to have correct owner and price", async function () {
      await buyOneTile();
      const res = await Board.tiles(0, 0);
      expect(res.owner).to.equal(alice.address);
      expect(res.price).to.equal(INIT_PURCHASE_PRICE);
    });

    it("Pending withdrawals contain correct amounts after the first buy.", async function () {
      await buyOneTile();
      const aliceReward = await Board.pendingWithdrawals(alice.address);
      const bobReward = await Board.pendingWithdrawals(bob.address);
      const charlyReward = await Board.pendingWithdrawals(charly.address);
      const deployerReward = await Board.pendingWithdrawals(deployer.address);
      console.log("dep rew", deployerReward);
      //
      expect(aliceReward).to.equal(ethers.BigNumber.from(0));
      expect(bobReward).to.equal(ethers.BigNumber.from(0));
      // Referer gets 10 % of 30% of added value
      const thirdOfAddedPart = INIT_PURCHASE_PRICE.sub(INIT_BASE_PRICE).div(3);
      expect(charlyReward).to.equal(thirdOfAddedPart);
      expect(deployerReward).to.equal(
        INIT_BASE_PRICE.add(thirdOfAddedPart.mul(2))
      );
    });
  });
});
