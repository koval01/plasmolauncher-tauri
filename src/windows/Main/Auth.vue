<script setup lang="ts">

import { onMounted, ref } from 'vue';
import Button from '../../components/common/Button.vue';
import Input from '../../components/common/Input.vue';
import { appWindow, LogicalSize, PhysicalSize } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api';
import { useRouter } from 'vue-router';
import syncSize from '@utils/syncSize';

const disabled = ref(true)

const handleInput = (e: any) => {
    let regex = new RegExp('^[a-zA-Z0-9_]{3,16}$')
    // console.log(regex.test(e.target.value))
    nick.value = e.target.value
    disabled.value = !regex.test(e.target.value)
}

const router = useRouter();

const nick = ref("")

const setOfflineSession = async () => {
    loading.value = true
    await invoke('set_offline_session', { nick: nick.value })
    loading.value = false
    router.push('/launcher')
}

// const handlePlasmoOAuth = async () => {
//     await invoke('open_plasmo_oauth')
//     router.push('/oauth/plasmo')
// }

const loading = ref(false)

// let unlisten: Array<Function> = [];

onMounted(async () => {
    await syncSize()
})

// onUnmounted(async () => {
//   unlisten.forEach(async (fn) => { await fn() })
// })

</script>

<template>
    <div id="windowHeight" class="p-8 py-16 text-center">
    <!-- <div class="p-8 text-center flex flex-col align-center h-screen justify-center"> -->
        <!-- <Button @click="handlePlasmoOAuth" class="w-full">Авторизоваться</Button>
        <div class="my-6 relative">
            <span class="z-50 bg-neutral-900 p-4 text-neutral-500 font-medium">или</span>
            <hr class="border-neutral-700 w-full absolute top-1/2 -z-10"/>
        </div> -->
        <Input :value="nick" @input="handleInput" @keyup.enter="setOfflineSession" placeholder="Введите ник"/>
        <p class="text-left text-xs mt-2 text-white font-medium" >Латиница, от 3 до 16 символов</p>
        <Button @click="setOfflineSession" :loading="loading" :disabled="disabled" class="w-full mt-5">Продолжить</Button>
    </div>
</template>