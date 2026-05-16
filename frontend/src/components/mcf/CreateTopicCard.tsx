import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../../api";
import type { ApiError } from "../../apiError";
import { useAuth } from "../../contexts/AuthContext.tsx";
import ErrorMessage from "../ErrorMessage";

type Props = {
    onCreated: () => Promise<void>;
    error?: ApiError;
};

export default function CreateTopicCard({ onCreated }: Props) {
    const [name, setName] = useState("");
    const [description, setDescription] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<ApiError>();

    async function submit() {
        try {
            setError(undefined);
            setLoading(true);

            await api.createTopic({ name, description });
            setName("");
            setDescription("");
            await onCreated();
        } catch (e) {
            setError(e as ApiError);
        } finally {
            setLoading(false);
        }
    }

    const navigate = useNavigate();
    const { auth } = useAuth();
    const unAuthorized = auth.status === "unknown" || auth.status === "guest";

    return (
        <section className="mcf-create">
            <h2>Start new mischief</h2>

            {!unAuthorized && (
                <><input
                    placeholder="Mischief name"
                    value={name}
                    onChange={e => setName(e.target.value)}/><textarea
                    placeholder="Describe the chaos"
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                    rows={3}/>
                    <button className="mcf-submit" onClick={submit}
                            disabled={!name || loading}>
                        <span className="mcf-submit-text">
                            {loading ? "Scurrying…" : "Create"}
                        </span>

                        {loading && (
                            <span className="mcf-spinner" aria-hidden/>
                        )}
                    </button>
                </>
            )}


            {unAuthorized && (
                <div className="mcf-auth-cta">
                    <p>
                        You need to be signed in to start a new mischief.
                    </p>

                    <button
                        type="button"
                        className="mcf-login"
                        onClick={() => navigate("/auth")}
                    >
                        Sign in
                    </button>
                </div>
            )}

            <ErrorMessage error={error}/>
        </section>
    );
}
