import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../../api";
import type { ApiError } from "../../apiError";
import { useAuth } from "../../contexts/AuthContext.tsx";
import ErrorMessage from "../ErrorMessage";

type Props = {
    topicName: string;
    onCreated: () => Promise<void>;
    error?: ApiError;
};

export default function CreatePostCard({ topicName, onCreated }: Props) {
    const [title, setTitle] = useState("");
    const [content, setContent] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<ApiError>();

    async function submit() {
        try {
            setError(undefined);
            setLoading(true);

            await api.createPost(topicName, { title, content });

            setTitle("");
            setContent("");
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
        <section className="post-create">
            <h2>Create a new Nibble</h2>

            {!unAuthorized && (
                <><input
                    placeholder="Nibble title"
                    value={title}
                    onChange={e => setTitle(e.target.value)}/><textarea
                    placeholder="Talk about the nibble"
                    value={content}
                    onChange={e => setContent(e.target.value)}
                    rows={3}/>
                    <button className="post-submit" onClick={submit}
                            disabled={!title || loading}>
                        <span className="post-submit-text">
                            {loading ? "Scurrying…" : "Create"}
                        </span>

                        {loading && (
                            <span className="post-spinner" aria-hidden/>
                        )}
                    </button>
                </>
            )}


            {unAuthorized && (
                <div className="post-auth-cta">
                    <p>
                        You need to be signed in to post a nibble.
                    </p>

                    <button
                        type="button"
                        className="post-login"
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
