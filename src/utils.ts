import { config } from "dotenv";
config();

function notFound(): string {
  throw "'MONGO_URL' not found in .env";
}

export const mongoUrl = process.env.MONGO_URL || notFound();

export type Cloak = "glitch" | "forrest" | "";

export interface Player {
  uuid: string;
  cloak: Cloak;
  cloaks: Cloak[];
}
