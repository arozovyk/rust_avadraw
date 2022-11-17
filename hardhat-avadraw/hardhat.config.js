require("@nomiclabs/hardhat-waffle");

module.exports = {
  defaultNetwork: "local",
  networks: {
    local: {
      url: "http://localhost:9650/ext/bc/C/rpc",
      gasPrice: 225000000000,
      chainId: 43112,
      accounts: [
        "0x56289e99c94b6912bfc12adc093c9b51124f0dc54ac7a766b2bc5ccf558d8027", // 0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC
        "0x3742165f277cd191962d7206fd390dc55dcca25a9dcc63ec4e30635944fe42df", // 0x3d337D74851fdE1965e797521f2eE299549c2349
        "0xa93074611ed221aae523d3c4ef5e1860c7a78e3172b73e7290060fbb3838167b", // 0x363A10ac202AA7fA4dc2F2867D73d8E3643699dc
        "0x32a7403e5a991773ad7b2a60163f6274b019d98c547a4f58391666dbc1d2c25c", // 0xE27AbB5392c3163798ce5e0B52dc927F318F825D
        "0x2cafcb5ba0e525d17f494df2d2587b3afc56d4fc8e6cc404d85eb8a6c15c79fa",
      ],
    },
    fuji: {
      url: "https://api.avax-test.network/ext/bc/C/rpc",
      gasPrice: 225000000000,
      chainId: 43113,
      accounts: [
        "0x32a7403e5a991773ad7b2a60163f6274b019d98c547a4f58391666dbc1d2c25c", // 0xE27AbB5392c3163798ce5e0B52dc927F318F825D
        "0x2cafcb5ba0e525d17f494df2d2587b3afc56d4fc8e6cc404d85eb8a6c15c79fa",
      ],
    },
  },
  solidity: {
    compilers: [{ version: "0.8.16" }],
    overrides: {
      "contracts/Board.sol": {
        version: "0.8.16",
        settings: {
          optimizer: {
            enabled: true,
            runs: 180,
          },
        },
      },
    },
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
    },
  },
};
