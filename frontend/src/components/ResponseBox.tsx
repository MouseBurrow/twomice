import type { ApiError } from "../apiError.ts";

type Props = {
    operation: string;
    request?: unknown;
    response: unknown;
    error?: ApiError;
};

export default function ResponseBox({ operation, request, response, error }: Props) {
    return (
        <div
            style={{
                marginTop: 16,
                padding: 12,
                border: "1px solid #444",
                background: "#111",
                color: "#0f0",
                fontFamily: "monospace",
                whiteSpace: "pre-wrap",
            }}
        >
            <strong>{operation}</strong>
            {!!request && (
                <pre>
                    <strong>REQUEST:</strong><br/>
                    {JSON.stringify(request, null, 2)}
                </pre>
            )}
            {!!response && (
                <pre>
                    <strong>RESPONSE:</strong><br/>
                    {JSON.stringify(response, null, 2)}
                </pre>
            )}
            {!!error && (
                <pre>
                    <strong>ERROR:</strong><br/>
                    {JSON.stringify(error, null, 2)}`
                </pre>
            )}
        </div>
    );
}
