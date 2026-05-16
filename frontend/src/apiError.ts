export type ApiErrorPayload = {
    error?: string;
    message?: string;
    [key: string]: unknown;
};

export class ApiError extends Error {
    readonly route: string;
    readonly status: number;
    readonly code?: string;
    readonly payload?: ApiErrorPayload;

    constructor(
        route: string,
        status: number,
        message: string,
        code?: string,
        payload?: ApiErrorPayload
    ) {
        super(message);

        this.route = route;
        this.status = status;
        this.code = code;
        this.payload = payload;

        Object.setPrototypeOf(this, ApiError.prototype);
    }

    get userMessage(): string {
        return (
            this.payload?.message ??
            this.message ??
            "Something went wrong"
        );
    }

    toJSON() {
        return {
            route: this.route,
            status: this.status,
            code: this.code,
            message: this.message,
            payload: this.payload,
        };
    }
}