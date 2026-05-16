import AuthStatus from "../AuthStatus.tsx";

type Props = {
    title: string;
    content: string;
}

export default function PostHeader({ title, content }: Props) {
    return (
        <div className="post-header">
            <div className="post-header-row">
                <span>
                    <h1>{title}</h1>
                    <p>{content}</p>
                </span>

                <AuthStatus/>
            </div>
        </div>
    );
}