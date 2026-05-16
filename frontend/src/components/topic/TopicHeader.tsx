import AuthStatus from "../AuthStatus.tsx";

type Props = {
    name: string;
    description: string;
}

export default function TopicHeader({ name, description }: Props) {
    return (
        <div className="topic-header">
            <div className="topic-header-row">
                <span>
                    <h1>{name}</h1>
                    <p>{description}</p>
                </span>

                <AuthStatus/>
            </div>
        </div>
    );
}