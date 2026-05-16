import AuthStatus from "../AuthStatus.tsx";

export default function Header() {
    return (
        <div className="mcf-header">
            <div className="mcf-header-row">
                <span>
                    <h1>🐭 Mischief Board</h1>
                    <p>Plots, schemes, and small disasters</p>
                </span>

                <AuthStatus/>
            </div>
        </div>
    );
}