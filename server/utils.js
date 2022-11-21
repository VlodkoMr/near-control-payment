const { connect, keyStores, utils, Contract } = require("near-api-js");

function convertToTera(amount) {
  return `${amount}000000000000`;
}

const CONTRACT_NAME = process.env.CONTRACT_NAME;
const VIEW_METHODS = ["get_tmp_list"];
const CHANGE_METHODS = ["send_order_payment", "cleanup_tmp_payments"];

function getConfig(env) {
  console.log(env);
  console.log("CONTRACT_NAME", CONTRACT_NAME);

  switch (env) {
    case "production":
    case "mainnet":
      return {
        networkId: "mainnet",
        nodeUrl: "https://rpc.mainnet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.near.org",
        helperUrl: "https://helper.mainnet.near.org",
        explorerUrl: "https://explorer.mainnet.near.org",
      };
    case "development":
    case "testnet":
    case "local":
      return {
        networkId: "testnet",
        nodeUrl: "https://rpc.testnet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
      };
    default:
      throw Error(
        `Unconfigured environment '${env}'. Can be configured in src/config.js.`
      );
  }
}

const nearConfig = getConfig(process.env.NODE_ENV || "testnet");

const initContract = async () => {
  const keyPair = new utils.key_pair.KeyPairEd25519(
    process.env.ACCOUNT_PRIVATE_KEY
  );
  const keyStore = new keyStores.InMemoryKeyStore();
  await keyStore.setKey(nearConfig.networkId, nearConfig.contractName, keyPair);
  const near = await connect(
    Object.assign({ deps: { keyStore: keyStore } }, nearConfig)
  );

  const account = await near.account(nearConfig.contractName);

  return await new Contract(account, nearConfig.contractName, {
    viewMethods: VIEW_METHODS,
    changeMethods: CHANGE_METHODS,
  });
};

module.exports = {
  convertToTera,
  initContract,
};
