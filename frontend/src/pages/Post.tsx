import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { api } from "../api";
import type { ApiError } from "../apiError";
import CommentGrid from "../components/post/CommentGrid";
import CreateCommentCard from "../components/post/CreateCommentCard";
import PostHeader from "../components/post/PostHeader.tsx";
import type { CommentData, PostData } from "../types";
import "../assets/Post.scss";

export default function Post() {
    const { topic, post } = useParams();

    const [postData, setPostData] = useState<PostData>();
    const [comments, setComments] = useState<CommentData[]>([]);
    const [error, setError] = useState<ApiError>();

    async function load(cancelled?: () => boolean) {
        try {
            setError(undefined);

            const [p, c] = await Promise.all([
                api.getPost(topic!, post!),
                api.getAllComments(topic!, post!)
            ]);

            if (cancelled?.()) return;

            setPostData(p);
            setComments(c.filter(x => !x.deleted));
        } catch (e) {
            if (cancelled?.()) return;
            setError(e as ApiError);
        }
    }

    useEffect(() => {
        let cancelled = false;
        load(() => cancelled);
        return () => {
            cancelled = true;
        };
    }, []);

    return (
        <div className="post-page">
            <div className="post-board">
                <PostHeader title={postData?.title ?? "Loading..."} content={postData?.content ?? "Loading..."}/>

                {postData && (
                    <>
                        <CreateCommentCard
                            topic={topic!}
                            post={post!}
                            onCreated={load}
                        />

                        <CommentGrid
                            topic={topic!}
                            post={post!}
                            comments={comments}
                        />
                    </>
                )}

                {error && (
                    <p className="post-error">
                        Failed to load comments.
                    </p>
                )}
            </div>
        </div>
    );
}
