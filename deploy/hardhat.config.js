require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */

const NEXT_PUBLIC_POLYGON_MUMBAI_RPC = "https://rpc.ankr.com/polygon_mumbai";
const NEXT_PUBLIC_PRIVATE_KEY =
  "e4886c1c6c6304808c07b218a84ae298c6601994ed24fe74e019fa5347bd0786";
module.exports = {
  solidity: "0.8.0",
  defaultNetwork: "matic",
  networks: {
    hardhat: {},
    polygon_mumbai: {
      url: NEXT_PUBLIC_POLYGON_MUMBAI_RPC,
      accounts: [`0x${NEXT_PUBLIC_PRIVATE_KEY}`],
    },
  },
};
