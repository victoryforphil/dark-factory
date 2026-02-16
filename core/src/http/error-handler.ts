const toMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }

  return "Unexpected error";
};

export const handleAppError = ({ code, error, set }: any) => {
  if (code === "PROTO_REQUEST_ERROR") {
    set.status = 400;
    return {
      message: toMessage(error),
    };
  }

  if (code === "PROTO_RESPONSE_ERROR") {
    set.status = 500;
    return {
      message: toMessage(error),
    };
  }

  return;
};
