import { AnimatePresence, motion, useAnimationControls, type Variants } from "framer-motion";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../api.ts";
import type { ApiError } from "../apiError.ts";
import ErrorMessage from "../components/ErrorMessage.tsx";
import { useAuth } from "../contexts/AuthContext.tsx";
import "../assets/Auth.scss";

export default function Auth() {
    const [mode, setMode] = useState<"login" | "signup">("login");
    const [lookingAway, setLookingAway] = useState(false);
    const [peek, setPeek] = useState(false);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [direction, setDirection] = useState<1 | -1>(1);
    const [error, setError] = useState<ApiError>();
    const [loading, setLoading] = useState(false);

    const nav = useNavigate();
    const { auth, refresh } = useAuth();

    async function handleSubmit() {
        try {
            setError(undefined);
            setLoading(true);

            if (mode === "login") {
                await api.login({ username, password });
            } else {
                await api.signup({ username, password });
            }

            await refresh();
            nav("/mcf")
        } catch (e) {
            setError(e as ApiError);
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        if (auth.status === "user" || auth.status === "admin") {
            nav("/mcf");
        }
    }, [auth, nav]);

    async function switchMode(next: "login" | "signup") {
        setMode(next);
        setTimeout(() => {
            setDirection(next === "signup" ? -1 : 1);
        }, 500);
    }

    useEffect(() => {
        setError(undefined);
    }, [mode]);

    const mouseShake = useAnimationControls();

    useEffect(() => {
        if (!error) return;

        mouseShake.start({
            x: [0, -6, 6, -4, 4, 0],
            transition: {
                duration: 0.25,
                ease: "easeOut",
            },
        });
    }, [error, mouseShake]);

    const mouseVariants: Variants = {
        idle: (custom: { right?: boolean }) => ({
            y: [3, -3],
            rotate: custom.right ? 40 : -40,
            transition: {
                y: {
                    duration: 1.5,
                    repeat: Infinity,
                    repeatType: "mirror",
                    ease: "easeInOut",
                    delay: custom.right ? 0.5 : 0
                },
                rotate: { duration: 0.35 },
            },
        }),
        peek: (custom: { right?: boolean }) => ({
            y: [3, -3],
            rotate: custom.right ? 100 : -100,
            transition: {
                y: { duration: 1.5, repeat: Infinity, repeatType: "mirror", ease: "easeInOut" },
                rotate: { duration: 0.3, ease: "easeInOut" },
            },
        }),
        away: (custom: { right?: boolean }) => ({
            y: [3, -3],
            rotate: custom.right ? 160 : -160,
            transition: {
                y: { duration: 1.5, repeat: Infinity, repeatType: "mirror", ease: "easeInOut" },
                rotate: { duration: 0.4, ease: "easeInOut" },
            },
        }),
    };

    const cardVariants: Variants = {
        initial: (dir: number) => ({ y: 200 * dir, opacity: 0 }),
        animate: { y: 0, opacity: 1 },
        exit: (dir: number) => ({ y: -200 * dir, opacity: 0 }),
    };

    return (
        <div className="auth-page">
            <AnimatePresence mode="wait">
                <motion.div
                    key={mode}
                    className="auth-card"
                    custom={direction}
                    variants={cardVariants}
                    initial="initial"
                    animate="animate"
                    exit="exit"
                    transition={{ duration: 0.2, ease: "easeInOut" }}
                >
                    {/* Mascot */}
                    <div className="tm-header">
                        <motion.div className="tm-mascot" animate={mouseShake}>
                            <motion.span
                                className="tm-mouse"
                                variants={mouseVariants}
                                custom={{ right: false }}
                                animate={peek ? "peek" : lookingAway ? "away" : "idle"}
                            >
                                🐭
                            </motion.span>


                            <motion.span
                                className="tm-mouse"
                                variants={mouseVariants}
                                custom={{ right: true }}
                                animate={peek ? "peek" : lookingAway ? "away" : "idle"}
                            >
                                🐭
                            </motion.span>
                        </motion.div>
                        <h1>TwoMice</h1>
                        <p>{mode === "login" ? "Welcome back to the mischief" : "Join the mischief"}</p>
                    </div>

                    {/* Form */}
                    <div className="tm-form">
                        <input placeholder="Mouse name" onChange={e => setUsername(e.target.value)}/>
                        <div className="tm-password">
                            <input
                                type={peek ? "text" : "password"}
                                placeholder="Secret stash"
                                value={password}
                                onFocus={() => !peek && setLookingAway(true)}
                                onBlur={() => setLookingAway(false)}
                                onChange={e => setPassword(e.target.value)}
                            />

                            <button
                                type="button"
                                className="tm-peek"
                                onClick={() => {
                                    setPeek(p => !p);
                                    setLookingAway(false);
                                }}
                                aria-label={peek ? "Hide password" : "Show password"}
                            >
                                {peek ? "👀" : "🙈"}
                            </button>
                        </div>

                        <button className="tm-submit" onClick={handleSubmit}>
                            <span className="tm-submit-text">
                                {mode === "login" ? "Scurry In" : "Create Burrow"}
                            </span>

                            {loading && (
                                <span className="tm-spinner" aria-hidden/>
                            )}
                        </button>

                        {/* Error message */}
                        <ErrorMessage error={error}/>
                    </div>

                    {/* Footer */}
                    <div className="tm-footer">
                        {mode === "login" ? (
                            <>
                                New mouse?{" "}
                                <button type="button" onClick={() => switchMode("signup")}>
                                    Build a burrow
                                </button>
                            </>
                        ) : (
                            <>
                                Already squeaking around?{" "}
                                <button type="button" onClick={() => switchMode("login")}>
                                    Scurry back in
                                </button>
                            </>
                        )}
                    </div>
                </motion.div>
            </AnimatePresence>
        </div>
    );
}
