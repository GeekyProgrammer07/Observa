type Level = "debug" | "info" | "warn" | "error";

const LEVEL_ORDER: Record<Level, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

let minLevel: Level = import.meta.env.DEV ? "debug" : "info";

export function setLogLevel(level: Level): void {
  minLevel = level;
}

export function getLogLevel(): Level {
  return minLevel;
}

function emit(level: Level, scope: string, message: string, ...data: unknown[]): void {
  if (LEVEL_ORDER[level] < LEVEL_ORDER[minLevel]) return;
  const prefix = `[${new Date().toISOString()}] [${scope}]`;
  console[level](prefix, message, ...data);
}

export interface Logger {
  debug: (message: string, ...data: unknown[]) => void;
  info: (message: string, ...data: unknown[]) => void;
  warn: (message: string, ...data: unknown[]) => void;
  error: (message: string, ...data: unknown[]) => void;
}

export function createLogger(scope: string): Logger {
  return {
    debug: (message, ...data) => emit("debug", scope, message, ...data),
    info: (message, ...data) => emit("info", scope, message, ...data),
    warn: (message, ...data) => emit("warn", scope, message, ...data),
    error: (message, ...data) => emit("error", scope, message, ...data),
  };
}
