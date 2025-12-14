import { mount } from "svelte";
import App from "./pages/App.svelte";
import "./constants/styles/index.css";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
