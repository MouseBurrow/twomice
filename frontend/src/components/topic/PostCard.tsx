import { motion } from "framer-motion";
import { Link } from "react-router-dom";
import type { PostData } from "../../types";

type Props = {
    topic: string;
    post: PostData;
};

export default function PostCard({ topic, post }: Props) {
    return (
        <motion.article
            className="topic-post"
            whileHover={{ y: -3 }}
        >
            <Link to={`/mcf/${topic}/nib/${post.slug}`}>
                <h3>{post.title}</h3>

                <p>
                    {post.content.slice(0, 120)}{post.content.length > 120 ? "..." : ""}
                </p>

                <span>
                    {new Date(post.created_at).toLocaleDateString()}
                </span>
            </Link>
        </motion.article>
    );
}