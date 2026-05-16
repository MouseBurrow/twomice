import { motion } from "framer-motion";
import type { TopicData } from "../../types";
import TopicCard from "./TopicCard";

type Props = {
    topics: TopicData[];
};

export default function TopicGrid({ topics }: Props) {
    return (
        <motion.div
            className="mcf-topics"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ duration: 0.2 }}
        >
            {topics.map(t => (
                <TopicCard key={t.name} topic={t}/>
            ))}
        </motion.div>
    );
}