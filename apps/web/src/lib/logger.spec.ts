import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { createLogger, getLogLevel, setLogLevel } from "./logger";

describe("logger", () => {
  const spies = {
    debug: vi.spyOn(console, "debug").mockImplementation(() => undefined),
    info: vi.spyOn(console, "info").mockImplementation(() => undefined),
    warn: vi.spyOn(console, "warn").mockImplementation(() => undefined),
    error: vi.spyOn(console, "error").mockImplementation(() => undefined),
  };
  const initialLevel = getLogLevel();

  beforeEach(() => {
    vi.clearAllMocks();
    setLogLevel("debug");
  });

  afterEach(() => {
    setLogLevel(initialLevel);
  });

  it("routes each level to the matching console method", () => {
    const log = createLogger("test");

    log.debug("d");
    log.info("i");
    log.warn("w");
    log.error("e");

    expect(spies.debug).toHaveBeenCalledTimes(1);
    expect(spies.info).toHaveBeenCalledTimes(1);
    expect(spies.warn).toHaveBeenCalledTimes(1);
    expect(spies.error).toHaveBeenCalledTimes(1);
  });

  it("prefixes messages with an ISO timestamp and the scope", () => {
    const log = createLogger("api");

    log.info("hello");

    const [prefix, message] = spies.info.mock.calls[0];
    expect(prefix).toMatch(/^\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z\] \[api\]$/);
    expect(message).toBe("hello");
  });

  it("passes structured data through to the console", () => {
    const log = createLogger("api");
    const err = new Error("boom");

    log.error("request failed", err, { status: 500 });

    const call = spies.error.mock.calls[0];
    expect(call[2]).toBe(err);
    expect(call[3]).toEqual({ status: 500 });
  });

  it("suppresses messages below the configured level", () => {
    setLogLevel("warn");
    const log = createLogger("test");

    log.debug("d");
    log.info("i");
    log.warn("w");
    log.error("e");

    expect(spies.debug).not.toHaveBeenCalled();
    expect(spies.info).not.toHaveBeenCalled();
    expect(spies.warn).toHaveBeenCalledTimes(1);
    expect(spies.error).toHaveBeenCalledTimes(1);
  });

  it("setLogLevel updates the reported level", () => {
    setLogLevel("error");
    expect(getLogLevel()).toBe("error");
  });
});
