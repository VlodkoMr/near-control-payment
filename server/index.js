const express = require("express");
const {
  initContract,
  convertToTera,
} = require("./utils");
const { Sequelize, QueryTypes } = require("sequelize");

const app = express();
const port = process.env.SERVER_PORT;

const CHECK_CONTRACT_INTERVAL_SECONDS = 10;
const sequelize = new Sequelize(process.env.DATABASE_URL)

const startContractChecks = async () => {
  const contract = await initContract();
  const tmpOrders = await contract.get_tmp_list();
  if (tmpOrders.length) {
    let isList = tmpOrders.map(order => order.order_id);
    await contract.cleanup_tmp_payments({
      orders: isList
    });

    tmpOrders.map(async order => {
      await sequelize.query("UPDATE handmade_orders SET is_paid = TRUE WHERE id = :id", {
        replacements: {
          id: order.order_id,
        },
        type: QueryTypes.SELECT
      });
    });
  }
}

setInterval(() => {
  startContractChecks();
}, CHECK_CONTRACT_INTERVAL_SECONDS * 1000);

app.listen(port, () => console.log(`Listening on port ${port}`));
