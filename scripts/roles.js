const { v4: uuid } = require('uuid');
const utils = require('./utils');

const roles = [
  {
    name: 'SUPER_ADMIN',
    permissions: [
      'createAdminUser',
    ],
  },
];

const updateRole = (collection) => async (role) => {
  const roleDb = await collection.findOne({ name: role.name });
  if (roleDb) {
    await collection.updateOne(
      { _id: roleDb.id },
      { $set: role },
    );
  } else {
    await collection.insertOne({
      _id: uuid(),
      ...role,
    });
  }
};

async function main() {
  const config = await utils.getConfig();
  const [db, client] = await utils.getDb(config);

  await Promise.all(
    roles.map(updateRole(db.collection('Roles'))),
  );

  client.close();
}

main().catch(console.error);
