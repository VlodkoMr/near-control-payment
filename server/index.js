const express = require("express");
const { prisma } = require("./utils/prisma");
const {
  initContract,
  convertToTera,
} = require("./utils/utils");

const app = express();
const port = process.env.SERVER_PORT;

const corsConfig = {
  origin: [
    "http://localhost:1234",
  ],
  methods: "GET,OPTION,HEAD,PUT,PATCH,POST,DELETE",
  optionsSuccessStatus: 200,
};

const CHECK_CONTRACT_INTERVAL_SECONDS = 10;
const contractMain = await initContract();
console.log(`contractMain`, contractMain);

const startContractChecks = async () => {
  console.log(`+`);

// let users = await prisma.Users.findMany();
}

setInterval(() => {
  startContractChecks();
}, CHECK_CONTRACT_INTERVAL_SECONDS * 1000);

app.listen(port, () => console.log(`Listening on port ${port}`));
