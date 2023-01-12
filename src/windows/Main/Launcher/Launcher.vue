<script setup lang="ts">

import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { appDataDir, join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { readBinaryFile, BaseDirectory, readTextFile } from '@tauri-apps/api/fs';
import { storeToRefs } from 'pinia';
import { RouterLink } from 'vue-router';
import { computed, onMounted, onUnmounted, ref } from 'vue';

import { useSessionStore } from '@store/sessionStore';
import { useTaskProgressStore } from '@store/taskProgressStore';

import Progress from "./Progress.vue"
import Button from "@components/common/Button.vue"
import FolderIcon from "@components/common/icons/FolderIcon.vue"
import LogoutIcon from "@components/common/icons/LogoutIcon.vue"
import SettingsIcon from "@components/common/icons/SettingsIcon.vue"
import { appWindow, WebviewWindow } from '@tauri-apps/api/window';
import { useI18n } from 'vue-i18n';
import syncSize from '@utils/syncSize';
import IconButton from '@components/common/IconButton.vue';
import { SessionEnum } from '@bindings/SessionEnum';
import useUnlisten from '@utils/useUnlisten';

const taskProgressState = useTaskProgressStore();
const sessionStore = useSessionStore();

const { session } = storeToRefs(sessionStore)
const { processing, progress } = storeToRefs(taskProgressState)

const buttonCooldown = ref(false)

const withCooldown = (fn: Function) => async () => {
  buttonCooldown.value = true
  setTimeout(() => buttonCooldown.value = false, 500)
  await fn()
}

const startTask = withCooldown( async () => await invoke('start_task') )

const killGame = withCooldown( async () => await emit('kill_game'))

const cancelQueue = withCooldown( async () => await emit('cancel_task_queue') )

const openGameFodler = async () => await invoke('open_instances_folder')


const openSettings = async () => {
  const settingsWindow = WebviewWindow.getByLabel('settings')
  await settingsWindow?.show()
  await settingsWindow?.setFocus()
}

const avatarUrl = ref<string | null>(null);

const unlisten = useUnlisten()

onMounted(async () => {

  unlisten.push(await listen("synced-state://session-update", async () => {
    const check = await invoke("check_skin_update")
    if (check) await loadAvatar()
  }))

  await syncSize()

  await loadAvatar()
  const check = await invoke("check_skin_update")
  if (check) await loadAvatar()
})

onUnmounted(async () => {
  await unlisten.unlisten()
})

// Can't use tauri's asset protocol because it does not update when the asset is changed
const loadAvatar = async () => {
  // let dir = await appDataDir()
  // let path = await join('cache', 'avatar.png')
  const file = (
    await readBinaryFile('cache/avatar.png', { dir: BaseDirectory.AppData })
  ) as unknown as number[];
  let string = String.fromCharCode.apply(null, file)
  avatarUrl.value = `data:image/png;base64, ${window.btoa(string)}`
}

</script>

<template>

<div id="windowHeight" class="p-8 py-16">
<!-- <div class="p-8 flex flex-col align-center h-screen justify-center"> -->
  
  <div :v-if="session && (session.type !== 'none')" class="flex items-center justify-between gap-4">
    <div class="flex gap-4 items-center">
      <div class="relative w-fit">
        <img
        class="w-16 h-16 rounded-lg"
        v-if="avatarUrl"
        :src="avatarUrl"
        style="image-rendering: pixelated;"/>
        <div v-else class="w-16 h-16 rounded-lg bg-neutral-800 animate-pulse"/>
        <div class="block absolute w-full h-full bg-neutral-800 top-0 rounded-lg -z-10 animate-pulse"/>
      </div>
      <div v-if="session?.type === 'offline'">
        <p class="font-semibold">{{ session.content.nick }}</p>
        <!-- <p class="text-neutral-400 text-sm font-semibold mt-0.5">Не авторизован</p> -->
      </div>
      <!-- <div v-else-if="session?.type === 'plasmo'">
        <p class="font-semibold">{{ session.content.nick }}</p>
        <p class="text-green-600 text-sm font-semibold mt-0.5">Авторизован</p>
      </div> -->
    </div>
    <RouterLink to="/">
      <IconButton>
        <LogoutIcon class="fill-pink-50"/>
      </IconButton>
    </RouterLink>
  </div>

  <div class="mt-6 mb-3">
    <Button v-if="buttonCooldown" class="w-full" :loading="true"/>
    <template v-else>
      <Button
        @click="startTask"
        v-if="!processing"
        class="w-full">
        Играть
      </Button>
      <Button
        @click="killGame"
        type="main_danger"
        v-else-if="progress?.progress.type === 'gamelaunched'"
        class="w-full">
        Закрыть игру
      </Button>
      <Button
        v-else
        @click="cancelQueue"
        class="w-full"
        type="main_warn">
        Отменить запуск
      </Button>
    </template>
  </div>

  <div style="height: 175px" className="w-full flex justify-center">
    <img src="/lomaka.png" className="w-full h-full object-contain" alt="" />
  </div>

  <div class="flex gap-3 mt-3">
    <Button @click="openGameFodler" type="secondary" class="w-full">
      <FolderIcon class="fill-pink-50"/>
      <p>Папка</p>
    </Button>
    <Button @click="openSettings" type="secondary" class="w-full">
      <SettingsIcon class="fill-pink-50"/>
      <p>Настройки</p>
    </Button>
  </div>
  
</div>

<Progress v-if="processing && progress?.progress.type !== 'gamelaunched'" />

</template>
