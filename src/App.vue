<script setup lang="ts">

import { invoke, dialog } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event'
import { appWindow, LogicalSize } from '@tauri-apps/api/window'
import useUnlisten from '@utils/useUnlisten'
import { onMounted, onUnmounted, ref } from 'vue'
import { RouterView, useRoute, useRouter } from 'vue-router'
import { useSessionStore } from './store/sessionStore'
import { useTaskProgressStore } from './store/taskProgressStore'

import './assets/fonts.css'
import { useMetaInstanceStore } from '@store/metaInstanceStore';
import { usePreferencesStore } from '@store/preferencesStore';

const taskProgressState = useTaskProgressStore();
const sessionStore = useSessionStore();
const metaInstanceStore = useMetaInstanceStore();
const preferencesStore = usePreferencesStore();

const unlisten = useUnlisten()

const route = useRoute()

onMounted(async () => {
  unlisten.push(await sessionStore.init())
  unlisten.push(await preferencesStore.init())
  unlisten.push(await taskProgressState.init())
  await metaInstanceStore.init()
})

onUnmounted(async () => await unlisten.unlisten())

// const route = useRoute()

// const test = ref<any>(null)

// const handleTest = async () => {
//   test.value = await invoke('test')
// }

</script>

<template>

  <component :is="route.meta.window">

    <!-- <button @click="handleTest">Test</button>

    <p>{{ test }}</p> -->

    <RouterView/>

  </component>

</template>
