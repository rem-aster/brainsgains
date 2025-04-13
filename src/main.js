import { createApp } from "vue";
import App from "./App.vue";
import './assets/styles.css';

import { listen } from '@tauri-apps/api/event';
import { restoreStateCurrent, saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';

restoreStateCurrent(StateFlags.ALL);

function debounce(fn, delay) {
    let timer;
    return function (...args) {
        clearTimeout(timer);
        timer = setTimeout(() => fn(...args), delay);
    };
}

const saveSize = debounce(async () => {
    saveWindowState(StateFlags.ALL);
}, 500);

listen('tauri://resize', saveSize);
createApp(App).mount("#app");