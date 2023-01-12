import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router"

import Auth from "./windows/Main/Auth.vue"
import Launcher from "./windows/Main/Launcher/Launcher.vue"
import PlasmoOAuth from "./windows/Main/PlasmoOAuth.vue"
import Settings from "./windows/Secondary/Settings/Settings.vue"
import Main from "./windows/Main/Main.vue"
import Secondary from "./windows/Secondary/Secondary.vue"

const routes = [
    { path: '/', component: Auth, meta: { window: Main } },
    { path: '/oauth/plasmo', component: PlasmoOAuth, meta: { window: Main } },
    { path: '/launcher', component: Launcher, meta: { window: Main } },
    { path: '/settings', component: Settings, meta: { window: Secondary } },
]

export const router = createRouter({
    history: createWebHistory(),
    routes,
})   