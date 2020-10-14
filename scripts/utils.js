const fs = require('fs');
const mongodb = require('mongodb');

function readFile(fname) {
  return new Promise((res, err) => {
    fs.readFile(fname, (e, dat) => {
      if (e) {
        err(e);
      } else {
        res(dat);
      }
    });
  });
}

async function getConfig() {
  return JSON.parse(await readFile('../be/resources/config.json'));
}

async function getDb(config) {
  const client = await mongodb.MongoClient.connect(config.mongo.uri);

  return [client.db(config.mongo.database), client];
}

module.exports = {
  getConfig,
  getDb,
};
