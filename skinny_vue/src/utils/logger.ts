export type LogLevel = "info" | "warn" | "error" | "debug";

export interface LogPayload {
    message: string;
    data?: unknown;
    timestamp?: string;
}

class Logger {
    private format(level: LogLevel, payload: LogPayload) {
        return {
            level,
            timestamp: payload.timestamp ?? new Date().toISOString(),
            message: payload.message,
            data: payload.data
        };
    }

    info(message: string, data?: unknown) {
        console.info(this.format("info", { message, data }));
    }

    warn(message: string, data?: unknown) {
        console.warn(this.format("warn", { message, data }));
    }

    error(message: string, data?: unknown) {
        console.error(this.format("error", { message, data }));
    }

    debug(message: string, data?: unknown) {
        console.debug(this.format("debug", { message, data }));
    }
}

export const logger = new Logger();
