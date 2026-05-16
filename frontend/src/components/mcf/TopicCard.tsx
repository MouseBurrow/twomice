import { motion } from "framer-motion";
import { Link } from "react-router-dom";
import type { TopicData } from "../../types";

export default function TopicCard({ topic }: { topic: TopicData }) {
    return (
        <motion.div
            className="mcf-topic"
            whileHover={{ y: -3 }}
        >
            <Link to={`/mcf/${topic.name}`}>
                <h3>{topic.name}</h3>
                <p>{topic.description}</p>
                <span>
                    {new Date(topic.created_at).toLocaleDateString()}
                </span>
            </Link>
        </motion.div>
    );
}