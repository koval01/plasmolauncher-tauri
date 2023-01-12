import { defineStore } from 'pinia'

import { emit, listen } from '@tauri-apps/api/event'

import { ref } from 'vue'

import { Preferences } from "@bindings/Preferences"
import { invoke } from '@tauri-apps/api'

export const usePreferencesStore = defineStore('preferencesStore', () => {

    const preferences = ref<Preferences | null>(null)

    const init = async () => {
        preferences.value = await invoke<Preferences>('get_prefs')
        const unlisten = await listen<Preferences>("synced-state://prefs-update", async (event) => {
            preferences.value = event.payload
        })
        return unlisten
    }

    return { preferences, init }
})