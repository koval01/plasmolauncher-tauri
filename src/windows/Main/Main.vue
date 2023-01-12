<script setup lang="ts">
import { dialog } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { appWindow, WebviewWindow, WindowManager } from '@tauri-apps/api/window';
import { onMounted, onUnmounted } from 'vue';
import { RouterView } from 'vue-router';

import useUnlisten from '@utils/useUnlisten'
import syncSize from '@utils/syncSize';

const unlisten = useUnlisten()

onMounted(async () => {

  try {
    await syncSize()

    unlisten.push(await appWindow.onCloseRequested(() => {
      WebviewWindow.getByLabel("settings")?.close()
      appWindow.close()
    }))

    unlisten.push(await listen<string>('global-error', (event) => {
      dialog.message(event.payload, {
        title: "Ошибка",
        type: "error"
      })
    }))
  } catch (e) {
    console.error(e)
  } finally {
    await appWindow.show()
    await appWindow.setFocus()
  }
})

onUnmounted(async () => await unlisten.unlisten())

</script>


<template>
<slot />

</template>