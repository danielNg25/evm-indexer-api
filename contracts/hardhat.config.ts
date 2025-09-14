import "@nomicfoundation/hardhat-chai-matchers";
import "@nomicfoundation/hardhat-verify";
import "@nomiclabs/hardhat-solhint";
import "@openzeppelin/hardhat-upgrades";
import "@typechain/hardhat";
import "@nomicfoundation/hardhat-toolbox";
import "solidity-coverage";
import * as dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/types";
import "hardhat-docgen";
import "hardhat-gas-reporter";
import "hardhat-contract-sizer";
import "hardhat-tracer";
import "hardhat-log-remover";

dotenv.config();

const config: HardhatUserConfig = {
    defaultNetwork: "hardhat",
    networks: {
        hardhat: {
            // chainId: 1,
            forking: {
                // url: "https://public-en.kairos.node.kaia.io",
                // url: "https://rpc.ankr.com/klaytn",
                // blockNumber: 158075311,
                url: `https://eth.llamarpc.com`,
                blockNumber: 19810683,
            },
            accounts: {
                count: 10,
            },
        },
        tenderly: {
            url: "https://rpc.tenderly.co/fork/403aa1fd-52c2-443e-9cb5-5817e2776c3b",
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        sepolia: {
            url: `https://sepolia.infura.io/v3/${process.env.INFURA_API_KEY}`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        goerli: {
            url: `https://goerli.infura.io/v3/${process.env.INFURA_API_KEY}`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        mainnet: {
            url: `https://mainnet.infura.io/v3/${process.env.INFURA_API_KEY}`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        mumbai: {
            url: `https://matic-mumbai.chainstacklabs.com/`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        bsctestnet: {
            url: `https://data-seed-prebsc-1-s1.binance.org:8545/`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        xlayer: {
            url: `https://endpoints.omniatech.io/v1/xlayer/mainnet/public`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        baobab: {
            url: `https://public-en-baobab.klaytn.net`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
        ip: {
            url: `https://testnet.storyrpc.io/`,
            accounts: [`${process.env.PRIVATE_KEY}`],
        },
    },
    etherscan: {
        apiKey: {
            goerli: `${process.env.ETHERSCAN_KEY}`,
            sepolia: `${process.env.ETHERSCAN_KEY}`,
            bscTestnet: `${process.env.BSCSCAN_KEY}`,
            polygonMumbai: `${process.env.POLYGONSCAN_KEY}`,
            mainnet: `${process.env.ETHERSCAN_KEY}`,
            bsctestnet: `${process.env.BSCSCAN_KEY}`,
            polygonMainnet: `${process.env.POLYGONSCAN_KEY}`,
            klaytn: "unnecessary",
        },
        customChains: [
            {
                network: "klaytn",
                chainId: 1001,
                urls: {
                    apiURL: "https://api-baobab.klaytnscope.com/api",
                    browserURL: "https://baobab.klaytnscope.com",
                },
            },
        ],
    },
    solidity: {
        compilers: [
            {
                version: "0.8.25",
                settings: {
                    optimizer: {
                        enabled: true,
                        runs: 200,
                        details: {
                            yul: true,
                        },
                    },
                    viaIR: true,
                },
            },
            {
                version: "0.7.6",
                settings: {
                    optimizer: {
                        enabled: true,
                        runs: 200,
                        details: {
                            yul: true,
                        },
                    },
                },
            },
        ],
    },
    paths: {
        sources: "./contracts",
        tests: "./tests",
        cache: "./cache",
        artifacts: "./artifacts",
    },
    mocha: {
        timeout: 200000,
        reporter: "mocha-multi-reporters",
        reporterOptions: {
            configFile: "./mocha-report.json",
        },
    },
    docgen: {
        path: "./docs",
        clear: true,
        runOnCompile: false,
    },
    contractSizer: {
        alphaSort: true,
        runOnCompile: true,
        disambiguatePaths: false,
    },
    gasReporter: {
        currency: "ETH",
        gasPrice: 18,
        enabled: true,
        excludeContracts: [],
    },
    typechain: {
        outDir: "typechain-types",
        target: "ethers-v6",
    },
};

module.exports = config;
