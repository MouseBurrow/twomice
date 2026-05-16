import { motion } from "framer-motion";
import { useState } from "react";
import { api } from "../../api";
import type { ApiError } from "../../apiError";
import type { CommentData, ReplyData } from "../../types";
import ErrorMessage from "../ErrorMessage";
import CreateReplyCard from "./CreateReplyCard.tsx";

type Props = {
    topic: string;
    post: string;
    comment: CommentData;
};

export default function CommentCard({ topic, post, comment }: Props) {
    const [open, setOpen] = useState(false);
    const [loading, setLoading] = useState(false);
    const [replies, setReplies] = useState<ReplyData[]>([]);
    const [error, setError] = useState<ApiError>();

    async function loadReplies() {
        try {
            setLoading(true);
            setError(undefined);

            const data = await api.getReplies(
                topic,
                post,
                comment.hash
            );

            setReplies(data.filter(r => !r.deleted));
        } catch (e) {
            setError(e as ApiError);
        } finally {
            setLoading(false);
        }
    }

    async function toggleReplies() {
        if (open) {
            setOpen(false);
            return;
        }

        if (replies.length === 0) {
            try {
                setLoading(true);
                setError(undefined);

                const data = await api.getReplies(
                    topic,
                    post,
                    comment.hash
                );

                setReplies(data.filter(r => !r.deleted));
            } catch (e) {
                setError(e as ApiError);
            } finally {
                setLoading(false);
            }
        }

        setOpen(true);
    }

    return (
        <motion.article className="comment-card">
            <p className="comment-content">
                {comment.content}
            </p>

            <div className="comment-meta">
                <span>
                    {new Date(comment.created_at).toLocaleDateString()}
                </span>

                <button
                    className="comment-replies-toggle"
                    onClick={toggleReplies}
                >
                    {open ? "Hide replies" : "View replies"}
                </button>
            </div>

            {loading && (
                <div className="comment-loading">
                    Loading replies…
                </div>
            )}

            {open && (
                <div className="comment-replies">
                    {replies.map(r => (
                        <div key={r.hash} className="comment-reply">
                            <p>{r.content}</p>
                            <span>
                                {new Date(r.created_at).toLocaleDateString()}
                            </span>
                        </div>
                    ))}

                    <CreateReplyCard
                        topic={topic}
                        post={post}
                        commentHash={comment.hash}
                        onCreated={loadReplies}
                    />
                </div>
            )}

            <ErrorMessage error={error}/>
        </motion.article>
    );
}