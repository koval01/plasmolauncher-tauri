import { defineStore } from 'pinia'

import { emit, listen } from '@tauri-apps/api/event'

import { ref } from 'vue'

import { SessionEnum } from "@bindings/SessionEnum"
import { invoke } from '@tauri-apps/api'
import { useRouter } from 'vue-router'

export const useSessionStore = defineStore('sessionStore', () => {

    const session = ref<SessionEnum | null>(null)

    const router = useRouter()

    const updateSession = async () => {
        try {
            await invoke('update_session')
        } catch(e) {
            console.error(e)
        }
    }

    const init = async () => {
        session.value = await invoke<SessionEnum>('get_session')
        updateSession()
        const unlisten = await listen<SessionEnum>("synced-state://session-update", async (event) => {
            session.value = event.payload
        })
        return unlisten
    }

    const getNick = (): null | string => {
        if (!session.value) return null
        if (session.value.type == "none") return null
        if (session.value.type == "offline") return session.value.content.nick
        if (session.value.type == "plasmo") return session.value.content.nick
        return null
    }

    // const setOfflineSession

    return { session, init, getNick }
})