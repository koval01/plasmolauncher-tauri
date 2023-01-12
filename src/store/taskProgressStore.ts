import { defineStore } from 'pinia'

import { listen } from '@tauri-apps/api/event'

import { computed, ref } from 'vue'

import { NamedProgress } from '@bindings/NamedProgress'
import { invoke } from '@tauri-apps/api'

export const useTaskProgressStore = defineStore('taskProgressStore', () => {

    const progress = ref<NamedProgress | null>(null)

    const init = async () => {
        progress.value = await invoke<NamedProgress>('get_progress')
        const unlisten = await listen<NamedProgress>("synced-state://progress-update", async (event) => {
            progress.value = event.payload
        })
        return unlisten
    }

    const processing = computed(() => {
        return Boolean(progress.value?.progress.type != 'none')
    })

    return { processing, progress, init }
})