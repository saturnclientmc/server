import { MongoClient } from "mongodb";
import { Cloak, Player, mongoUrl } from "./utils";

export const validCloaks: Cloak[] = ["glitch"];

const mongoClient = new MongoClient(mongoUrl);

const db = mongoClient.db("saturnclient");

export const players = db.collection<Player>("players");
