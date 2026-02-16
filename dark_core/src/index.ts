import { Elysia } from 'elysia';
import { openapi } from '@elysiajs/openapi'
import { logger } from '@grotto/logysia';
import { llms } from "@opuu/elysia-llms-txt";

import Log from './utils/logging';

const app = new Elysia()
.use(openapi()) 
.use(
    llms({
      source: {
        type: "url",
        url: "/openapi/json",
      },
    })
  )
.use(logger({ 
            logIP: false,
            writer: {
                write(msg: string) {
                  Log.info(`Core // HTTP // ${msg.trim()}`);
                }
            }
        }))
.get('/', () => 'Hello Elysia').listen(4150);

Log.info('Core // HTTP // Server is running on http://localhost:4150');
