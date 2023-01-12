import { invoke } from "@tauri-apps/api";

const syncSize = async () => {
    let height = document.getElementById('windowHeight')?.scrollHeight || 500;
    await invoke('set_height', { height })
}

export default syncSize