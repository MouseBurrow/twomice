import { motion } from "framer-motion";
import type { PostData } from "../../types";
import PostCard from "./PostCard";

type Props = {
    topic: string;
    posts: PostData[];
};

export default function PostGrid({ topic, posts }: Props) {
    return (
        <motion.div
            className="topic-posts"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.2 }}
        >
            {posts.map(post => (
                <PostCard topic={topic} key={post.slug} post={post}/>
            ))}
        </motion.div>
    );
}
