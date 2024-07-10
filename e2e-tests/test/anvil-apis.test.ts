import { expect } from "chai";
import { Wallet } from "zksync-web3";
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
