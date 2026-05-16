import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../../api";
import type { ApiError } from "../../apiError";
import { useAuth } from "../../contexts/AuthContext";
import ErrorMessage from "../ErrorMessage";

type Props = {
    topic: string;
    post: string;
    onCreated: () => Promise<void>;
};

export default function CreateCommentCard({ topic, post, onCreated }: Props) {
    const [content, setContent] = useState("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<ApiError>();

    const { auth } = useAuth();
    const navigate = useNavigate();

    const unAuthorized =
        auth.status === "unknown" || auth.status === "guest";

    async function submit() {
        try {
            setError(undefined);
            setLoading(true);

            await api.createComment(topic, post, { content });

            setContent("");
            await onCreated();
        } catch (e) {
            setError(e as ApiError);
        } finally {
            setLoading(false);
        }
    }

    return (
        <section className="comment-create">
            <h2>Add a comment</h2>

            {!unAuthorized && (
                <>
                    <textarea
                        placeholder="Share your thoughts"
                        value={content}
                        onChange={e => setContent(e.target.value)}
                        rows={3}
                    />

                    <button
                        className="comment-submit"
                        onClick={submit}
                        disabled={!content || loading}
                    >
                        <span className="comment-submit-text">
                            {loading ? "Scurrying…" : "Comment"}
                        </span>

                        {loading && (
                            <span
                                className="comment-spinner"
                                aria-hidden
                            />
                        )}
                    </button>
                </>
            )}

            {unAuthorized && (
                <div className="comment-auth-cta">
                    <p>
                        You need to be signed in to comment.
                    </p>

                    <button
                        type="button"
                        className="comment-login"
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
