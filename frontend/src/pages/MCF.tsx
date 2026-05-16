import { useEffect, useState } from "react";
import { api } from "../api";
import type { ApiError } from "../apiError";
import CreateTopicCard from "../components/mcf/CreateTopicCard";
import Header from "../components/mcf/Header";
import TopicGrid from "../components/mcf/TopicGrid";
import type { TopicData } from "../types";
import "../assets/MCF.scss";

export default function MCF() {
    const [topics, setTopics] = useState<TopicData[]>([]);
    const [error, setError] = useState<ApiError>();

    async function load(cancelled?: () => boolean) {
        try {
            setError(undefined);
            const res = await api.getAllTopics();
            if (cancelled?.()) return;
            setTopics(res.filter(t => !t.deleted));
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
        <div className="mcf-page">
            <div className="mcf-board">
                <Header/>
                <CreateTopicCard onCreated={load} error={error}/>
                <TopicGrid topics={topics}/>
            </div>
        </div>
    );
}