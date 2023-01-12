<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import useUnlisten from '@utils/useUnlisten';
import { onMounted, onUnmounted } from 'vue';

const unlisten = useUnlisten()

onMounted(async () => {
    unlisten.push(await appWindow.listen("tauri://close-requested", async () => {
        await appWindow.hide()
    }))
})

onUnmounted(async () => await unlisten.unlisten())

</script>


<template>

<slot />

</template>