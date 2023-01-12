// import { defineStore } from 'pinia'

import { invoke } from "@tauri-apps/api"
import { defineStore } from "pinia"
import { ref } from "vue"

// import { emit, listen } from '@tauri-apps/api/event'

// import { ref } from 'vue'

// import { SessionEnum } from "@bindings/SessionEnum"
// import { invoke } from '@tauri-apps/api'
// import { useRouter } from 'vue-router'

import { MetaInstance } from '../types/rust-common/MetaInstance'

export const useMetaInstanceStore = defineStore('metaInstanceStore', () => {

    const metaInstance = ref<MetaInstance | null>(null)

    const init = async () => {
        metaInstance.value = await invoke<MetaInstance | null>('get_meta_instance_cache')
    }

    return { metaInstance, init }
})