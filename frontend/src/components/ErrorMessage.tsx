import "../assets/ErrorMessage.scss";
import type { ApiError } from "../apiError.ts";

const ERROR_MESSAGES: Record<string, Record<string, string>> = {
    "/login": {
        user_not_found: "That user does not exist",
        invalid_password: "Invalid password",
    },
    "/signup": {
        username_exists: "That username is already taken",
    },
    "/mcf": {
        invalid_name: "Topic name may contain only letters, digits, and underscores",
        topic_already_exists: "That topic already exists",
    }
};

type Props = {
    error?: ApiError;
};

export default function ErrorMessage({ error }: Props) {
    if (!error) return null;

    if (error.code && !ERROR_MESSAGES[error.route]?.[error.code]) {
        console.warn("Unhandled error: ", error.toJSON());
    }

    const message =
        ERROR_MESSAGES[error.route]?.[error.code ?? ""] ??
        error.payload?.message ??
        "Something went wrong";

    return (
        <div className="error-message" role="alert">
            <span className="error-icon">⚠</span>
            <span className="error-text">{message}</span>
        </div>
    );
}
