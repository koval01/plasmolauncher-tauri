<script setup lang="ts">

import Input from '@components/common/Input.vue';
import Button from '@components/common/Button.vue';
import IconButton from '@components/common/IconButton.vue';
import ArrowBackIcon from '@components/common/icons/ArrowBackIcon.vue';
import { onMounted, ref } from 'vue';
import syncSize from '@utils/syncSize';
import { RouterLink, useRouter } from 'vue-router';


import PasteGoIcon from '@components/common/icons/PasteGoIcon.vue'
import { readText, writeText } from '@tauri-apps/api/clipboard';
import { emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';

const token = ref("")
// const disabled = ref(true)
const loading = ref(false)

const router = useRouter()

// const handleInput = (e: any) => {
//     token.value = e.target.value
//     console.log(e.target.value)
//     disabled.value = Boolean(e.target.value)
// }

const setPlasmoSession = async () => {
    try {
        loading.value = true
        const clipboard = await readText()
        await writeText("")
        await invoke('set_plasmo_session', { token: clipboard })
        router.push('/launcher')
    } catch(e) {
        await emit("global-error", e)
    } finally {
        loading.value = false
    }
}

onMounted(async () => await syncSize())

</script>

<template>

<div id="windowHeight" class="p-8 py-16 flex flex-col items-center text-center">
    <Button @click="setPlasmoSession" type="secondary" :loading="loading" class="w-full mb-5">
        <PasteGoIcon class="fill-pink-50"/>
        Вставить токен и продолжить
    </Button>
    <RouterLink to="/">
        <IconButton type="secondary" class="w-fit">
            <ArrowBackIcon class="fill-pink-50" />
            <p>Назад</p>
        </IconButton>
    </RouterLink>
</div>

</template>