// ToastContext.tsx
import { createContext, type ReactNode, useContext, useState } from "react";
import { ToastContainer } from "../components/ToastContainer.tsx";

export type Toast = {
    id: string;
    message: string;
    type?: "success" | "error" | "info";
};

type ToastContextType = {
    toasts: Toast[];
    addToast: (message: string, type?: Toast["type"]) => void;
    removeToast: (id: string) => void;
};

const ToastContext = createContext<ToastContextType | undefined>(undefined);

// eslint-disable-next-line react-refresh/only-export-components
export const useToast = () => {
    const ctx = useContext(ToastContext);
    if (!ctx) throw new Error("useToast must be used inside ToastProvider");
    return ctx;
};

export const ToastProvider = ({ children }: { children: ReactNode }) => {
    const [toasts, setToasts] = useState<Toast[]>([]);

    const addToast = (message: string, type: Toast["type"] = "info") => {
        const id = crypto.randomUUID();
        setToasts(prev => [...prev, { id, message, type }]);
        // auto-remove after 3s
        setTimeout(() => removeToast(id), 3000);
    };

    const removeToast = (id: string) => {
        setToasts(prev => prev.filter(t => t.id !== id));
    };

    return (
        <ToastContext.Provider value={{ toasts, addToast, removeToast }}>
            {children}
            <ToastContainer toasts={toasts} removeToast={removeToast}/>
        </ToastContext.Provider>
    );
};
