// ToastContainer.tsx
import { AnimatePresence, motion } from "framer-motion";
import type { Toast } from "../contexts/ToastContext";
import "../assets/ToastContainer.scss"

export const ToastContainer = ({
                                   toasts,
                                   removeToast,
                               }: {
    toasts: Toast[];
    removeToast: (id: string) => void;
}) => {
    return (
        <div className="toast-container">
            <AnimatePresence>
                {toasts.map(t => (
                    <motion.div
                        key={t.id}
                        initial={{ opacity: 0, y: -20 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -20 }}
                        className={`toast toast-${t.type}`}
                        onClick={() => removeToast(t.id)}
                    >
                        {t.message}
                    </motion.div>
                ))}
            </AnimatePresence>
        </div>
    );
};
