import { Navigate, Route, Routes } from "react-router-dom";
import Auth from "./pages/Auth.tsx";
import "./assets/variables.scss"
import MCF from "./pages/MCF";
import Post from "./pages/Post";
import Topic from "./pages/Topic";
import "./assets/App.scss"

export default function App() {
    return (
        <Routes>
            <Route path="/" element={<Navigate to="/mcf"/>}/>

            <Route path="/auth" element={<Auth/>}/>

            <Route path="/mcf" element={<MCF/>}/>
            <Route path="/mcf/:topic" element={<Topic/>}/>
            <Route path="/mcf/:topic/nib/:post" element={<Post/>}/>

            <Route path="*" element={<div>404</div>}/>
        </Routes>
    );
}
