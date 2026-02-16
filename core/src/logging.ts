import adze from "adze";

export const logger = adze;

export const createLogysiaWriter = () => ({
  write(message: string) {
    logger.info(message);
  },
});
