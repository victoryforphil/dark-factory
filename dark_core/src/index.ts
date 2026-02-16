import { Elysia } from 'elysia';
import { openapi } from '@elysiajs/openapi'
import { logger } from '@grotto/logysia';

import Log from './utils/logging';

const app = new Elysia()
.use(openapi()) 
.use(logger({ 
            logIP: false,
            writer: {
                write(msg: string) {
                  logger.info(msg.trim())
                }
            }
        }))
.get('/', () => 'Hello Elysia').listen(3000);

Log.info('Core // HTTP // Server is running on http://localhost:3000');
