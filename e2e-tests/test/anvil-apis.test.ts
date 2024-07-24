import { expect } from "chai";
import { Wallet } from "zksync-web3";
import { RichAccounts } from "../helpers/constants";
import { getTestProvider } from "../helpers/utils";
import { ethers } from "hardhat";

const provider = getTestProvider();

describe("anvil_setBalance", function () {
  it("Should update the balance of an account", async function () {
    // Arrange
    const userWallet = Wallet.createRandom().connect(provider);
    const newBalance = ethers.utils.parseEther("42");

    // Act
    await provider.send("anvil_setBalance", [userWallet.address, newBalance._hex]);

    // Assert
    const balance = await userWallet.getBalance();
    expect(balance.eq(newBalance)).to.true;
  });
});

describe("anvil_setNonce", function () {
  it("Should update the nonce of an account", async function () {
    // Arrange
    const userWallet = Wallet.createRandom().connect(provider);
    const newNonce = 42;

    // Act
    await provider.send("anvil_setNonce", [userWallet.address, ethers.utils.hexlify(newNonce)]);

    // Assert
    const nonce = await userWallet.getNonce();
    expect(nonce).to.equal(newNonce);
  });
});

describe("anvil_mine", function () {
  it("Should mine multiple blocks with a given interval", async function () {
    // Arrange
    const numberOfBlocks = 100;
    const intervalInSeconds = 60;
    const startingBlock = await provider.getBlock("latest");
    const startingTimestamp: number = await provider.send("config_getCurrentTimestamp", []);

    // Act
    await provider.send("anvil_mine", [
      ethers.utils.hexlify(numberOfBlocks),
      ethers.utils.hexlify(intervalInSeconds),
    ]);

    // Assert
    const latestBlock = await provider.getBlock("latest");
    expect(latestBlock.number).to.equal(startingBlock.number + numberOfBlocks, "Block number mismatch");
    expect(latestBlock.timestamp).to.equal(
      startingTimestamp + (numberOfBlocks - 1) * intervalInSeconds * 1000 + 1,
      "Timestamp mismatch"
    );
  });
});

describe("anvil_impersonateAccount & anvil_stopImpersonatingAccount", function () {
  it("Should allow transfers of funds without knowing the Private Key", async function () {
    // Arrange
    const userWallet = Wallet.createRandom().connect(provider);
    const beforeBalance = await provider.getBalance(RichAccounts[5].Account);

    // Act
    await provider.send("anvil_impersonateAccount", [RichAccounts[5].Account]);

    const signer = await ethers.getSigner(RichAccounts[5].Account);
    const tx = {
      to: userWallet.address,
      value: ethers.utils.parseEther("0.42"),
    };

    const recieptTx = await signer.sendTransaction(tx);
    await recieptTx.wait();

    await provider.send("anvil_stopImpersonatingAccount", [RichAccounts[5].Account]);

    // Assert
    expect((await userWallet.getBalance()).eq(ethers.utils.parseEther("0.42"))).to.true;
    expect((await provider.getBalance(RichAccounts[5].Account)).eq(beforeBalance.sub(ethers.utils.parseEther("0.42"))))
      .to.true;
  });
});
