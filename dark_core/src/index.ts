import { Elysia } from "elysia";
import Log from './utils/logging';
const app = new Elysia().get("/", () => "Hello Elysia").listen(3000);

Log.info("Core // HTTP // Server is running on http://localhost:3000");

