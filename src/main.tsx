import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import TitleBar from "./TitleBar";
import "./styles.css";
import { Preface } from "./Preface";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <TitleBar />
        <App />
    </React.StrictMode>
);
