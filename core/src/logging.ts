import { Logestic } from "logestic";

export const requestLogger = Logestic.preset("fancy");

export const logger = {
  info(message: string) {
    console.info(message);
  },
  warn(message: string) {
    console.warn(message);
  },
  error(message: string) {
    console.error(message);
  },
};

export const createLogysiaWriter = () => ({
  write(message: string) {
    logger.info(message);
  },
});
