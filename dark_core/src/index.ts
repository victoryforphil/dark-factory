import { buildApp } from './app';
import Log from './utils/logging';

const PORT = 4150;
const HOST = 'localhost';

const app = buildApp();

app.listen(PORT);

Log.info(`Core // HTTP // Listening (env=${Bun.env.NODE_ENV ?? 'development'},host=${HOST},port=${PORT})`);
