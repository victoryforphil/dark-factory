export class NotFoundError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'NotFoundError';
  }
}

export class IdCollisionDetectedError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'IdCollisionDetectedError';
  }
}

export const isNotFoundError = (error: unknown): error is NotFoundError => {
  return error instanceof NotFoundError;
};

export const isIdCollisionDetectedError = (error: unknown): error is IdCollisionDetectedError => {
  return error instanceof IdCollisionDetectedError;
};
