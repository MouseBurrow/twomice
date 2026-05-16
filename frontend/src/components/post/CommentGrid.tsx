import { motion } from "framer-motion";
import type { CommentData } from "../../types";
import CommentCard from "./CommentCard";

type Props = {
    topic: string;
    post: string;
    comments: CommentData[];
};

export default function CommentGrid({ topic, post, comments }: Props) {
    return (
        <motion.div
            className="comment-list"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.2 }}
        >
            {comments.map(c => (
                <CommentCard
                    key={c.hash}
                    topic={topic}
                    post={post}
                    comment={c}
                />
            ))}
        </motion.div>
    );
}
