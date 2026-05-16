import { useState } from "react";
// import { api } from "../api";
import type { CommentData } from "../types";

type Props = {
    // topic: string;
    // post: string;
    data: CommentData;
};

export default function CommentItem({ /* topic, post, */ data }: Props) {
    const [open, setOpen] = useState(false);
    // const [replies, setReplies] = useState<CommentData[] | null>(null);
    // const [error, setError] = useState<ApiError | undefined>();

    async function toggle() {
        if (open) {
            setOpen(false);
            return;
        }

        setOpen(true);

        // if (replies !== null) return;

        // try {
        //     const res = await api.getReplies(topic, post, comment.hash);
        //     setReplies(res);
        // } catch (e) {
        //     setError(e as ApiError);
        // }
    }

    return (
        <li>
            <div
                style={{ cursor: "pointer", border: "1px solid #444", padding: "8px" }}
                onClick={toggle}
            >
                <p>{data.content}</p>
                <small>{data.created_at}</small>
            </div>

            {/*{error && <p style={{ color: "red" }}>{error.message}</p>}*/}

            {/*{open && replies && (*/}
            {/*    <ul style={{ marginLeft: "20px" }}>*/}
            {/*        {replies.map(r => (*/}
            {/*            <li key={r.hash}>*/}
            {/*                <p>{r.content}</p>*/}
            {/*                <small>{r.created_at}</small>*/}
            {/*            </li>*/}
            {/*        ))}*/}
            {/*    </ul>*/}
            {/*)}*/}
        </li>
    );
}
