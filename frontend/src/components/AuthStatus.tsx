import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext.tsx";
import "../assets/AuthStatus.scss"
import { useToast } from "../contexts/ToastContext.tsx";

export default function AuthStatus() {
    const navigate = useNavigate();
    const [loading, setLoading] = useState(false);

    const { auth, logout } = useAuth();
    const { addToast } = useToast();

    if (auth.status === "unknown") {
        return null; // app booting
    }

    async function handleLogout() {
        try {
            setLoading(true);
            await logout();
            setLoading(false);
            addToast("🐭 You have logged out successfully!", "success");
        } catch {
            addToast("⚠️ Logout failed. Please try again.", "error");
        }
    }

    if (auth.status === "guest") {
        return (
            <div className="auth-status guest">
                <span>🐭 Guest</span>
                <button onClick={() => navigate("/auth")}>
                    Sign in
                </button>
            </div>
        );
    }

    return (
        <div className="auth-status user">
            <span>
                🐭 {auth.info.username}
                {auth.status === "admin" && " </>"}
            </span>

            <button onClick={handleLogout} disabled={loading}>
                {loading ? "Logging out…" : "Log out"}
            </button>
        </div>
    );
}
