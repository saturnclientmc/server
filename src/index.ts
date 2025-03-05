import express from "express";
import { players, validCloaks } from "./database";
import { Cloak } from "./utils";

async function getPlayerData(token: string) {
  try {
    const response = await fetch(
      `https://api.minecraftservices.com/minecraft/profile`,
      {
        headers: {
          Authorization: "Bearer " + token,
        },
      }
    );

    if (!response.ok) {
      throw new Error("Invalid session");
    }

    const { id, name } = await response.json();

    return { uuid: id, name };
  } catch (error) {
    throw new Error("Failed to validate session");
  }
}

const port = 3000;

const app = express();

app.use(express.json());

app.get("/", (_, res) => {
  res.send({
    version: "0.0.1-alpha.0",
  });
});

app.get("/player/:uuid", async (req, res) => {
  const player = await players.findOne({
    uuid: req.params.uuid.replaceAll("-", ""),
  });

  if (!player) {
    res.send({
      saturn: false,
    });
    return;
  }

  const response = await fetch(
    `https://api.minecraftservices.com/minecraft/profile/lookup/${player.uuid}`
  );

  const { name } = await response.json();

  res.send({
    saturn: true,
    cloak: player.cloak,
    name,
  });
});

app.get("/auth", async (req, res) => {
  const { token } = req.query;

  try {
    const { uuid, name } = await getPlayerData(token as string);

    const player = await players.findOne({ uuid });

    if (!player) {
      await players.insertOne({
        uuid,
        cloak: "",
        cloaks: [],
      });

      res.send({
        success: true,
        uuid,
        cloak: "",
        cloaks: [],
      });
      return;
    }

    res.send({
      success: true,
      uuid,
      name,
      cloak: player.cloak,
      cloaks: player.cloaks,
    });
  } catch (error) {
    res.status(500).send({
      success: false,
      error,
    });
  }
});

app.post("/cloak/:cloak", async (req, res) => {
  const { cloak } = req.params;
  const { token } = req.query;

  const { uuid } = await getPlayerData(token as string);

  const player = await players.findOne({ uuid });

  if (!player) {
    res.status(400).send({
      success: false,
      error: "Player not found",
    });
  }

  if (!validCloaks.includes(cloak as Cloak)) {
    res.status(400).send({
      success: false,
      error: "Invalid cloak",
    });
    return;
  }

  if (!player.cloaks.includes(cloak as Cloak)) {
    res.status(400).send({
      success: false,
      error: "You don't have this cloak",
    });
    return;
  }

  await players.updateOne({ uuid }, { $set: { cloak: cloak as Cloak } });

  console.log("Cloak set to " + cloak + " for " + uuid);

  res.send({ success: true });
});

app.post("/cloak/", async (req, res) => {
  const { token } = req.query;

  const { uuid } = await getPlayerData(token as string);

  const player = await players.findOne({ uuid });

  if (!player) {
    res.status(400).send({
      success: false,
      error: "Player not found",
    });
  }

  await players.updateOne({ uuid }, { $set: { cloak: "" } });

  console.log("Cloak set to none for " + uuid);

  res.send({ success: true });
});

app.listen(3000, () => {
  console.log(`listening as http://localhost:${port}/`);
});
