export interface ApiSuccess<TData> {
  ok: true;
  data: TData;
}

export interface ApiFailure {
  ok: false;
  error: {
    code: string;
    message: string;
  };
}

export const success = <TData>(data: TData): ApiSuccess<TData> => ({
  ok: true,
  data,
});

export const failure = (code: string, message: string): ApiFailure => ({
  ok: false,
  error: {
    code,
    message,
  },
});

export const toErrorMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }

  return 'Unknown error';
};
