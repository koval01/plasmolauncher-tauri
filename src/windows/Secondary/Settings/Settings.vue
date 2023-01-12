<script setup lang="ts">

import { Listbox, ListboxButton, ListboxOption, ListboxOptions, Switch, SwitchGroup, SwitchDescription, SwitchLabel } from "@headlessui/vue"
import { useMetaInstanceStore } from "@store/metaInstanceStore";
import { usePreferencesStore } from "@store/preferencesStore";
import { invoke } from "@tauri-apps/api";
import classNames from "classnames";
import { storeToRefs } from "pinia";
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import memoryVariants from "./memoryVarians";
import { MemoryVariant } from "./memoryVarians";

import SmallArrowDown from "@components/common/icons/SmallArrowDown.vue"
import { MetaMod } from "../../../types/rust-common/MetaMod";

const selectedMemory = ref<MemoryVariant | null>(null)

const metaInstanceStore = useMetaInstanceStore()
const preferencesStore = usePreferencesStore()

const { metaInstance } = storeToRefs(metaInstanceStore)
const { preferences } = storeToRefs(preferencesStore)

watch(preferences, () => {
    let new_value = preferences.value?.java.maximum_memory_allocation
    let variant = memoryVariants.find(variant => variant.value == new_value)
    if (variant) {
        selectedMemory.value = variant
    }
})

watch(selectedMemory, async () => {
    let value = selectedMemory.value?.value
    if (!value) return
    await invoke('set_memory_alloc', { value })
})

const optionalMods = computed(() => {
    const mods = metaInstance.value?.mods
    if (!mods) return []
    return mods
        .filter(mod => mod.optional)
        .map(mod => {
            const value = preferences.value?.optional_mods[mod.id] || false
            return {
                data: mod,
                value: value
            }
        })
})

const modName = (mod: MetaMod): string => {
    return mod.locale.ru?.title || mod.id
}

const setOptinalMod = async (value: boolean, id: string) => {
    await invoke('set_optional_mod', {
        value,
        id
    })
}

const log = (e: any) => {
    console.log(e)
}

const { t, te } = useI18n()

</script>

<template>
    <div class="p-8">
        <p class="font-semibold mb-5">Выделенная память</p>
        <Listbox v-model="selectedMemory" as="div" class="relative w-full" v-slot="{ open }">
            <ListboxButton class="w-full bg-pink-400 rounded-lg text-left p-4 px-5 font-medium relative ring-1 ring-inset ring-pink-200 flex justify-between items-center" slot="{ open }">
                <p>{{ selectedMemory?.label || "Стандартное значение" }}</p>
                <SmallArrowDown :class="classNames('fill-pink-50 transition-transform', {
                    'rotate-180': open
                })"/>
            </ListboxButton>
            <ListboxOptions class="bg-pink-400 rounded-lg absolute border border-pink-200 my-2 py-2 w-full z-10">
                <ListboxOption
                    v-for="variant in memoryVariants"
                    class="py-2 px-5 hover:bg-pink-300 transition"
                    :key="variant.value"
                    :value="variant"
                >
                    {{ variant.label }}
                </ListboxOption>
            </ListboxOptions>
        </Listbox>
        <p class="font-semibold mt-8 mb-5">Опциональные моды</p>
        <div class="flex flex-col divide-y border-t border-b border-pink-300">
            <SwitchGroup v-for="mod in optionalMods" as="div" class="flex justify-between items-center gap-4 w-full py-5 border-pink-300">
                <SwitchLabel as="div" class="font-medium select-none w-full h-min cursor-pointer">
                    <p>{{ modName(mod.data) }}</p>
                    <SwitchDescription
                    class="text-xs text-pink-100 font-medium mt-1"
                    v-if="mod.data.locale.ru?.description">
                        {{ mod.data.locale.ru.description }}
                    </SwitchDescription>
                </SwitchLabel>
                <Switch
                value="mod.value"
                @update:model-value="(value) => setOptinalMod(value, mod.data.id)"
                :class="classNames('relative inline-flex h-8 w-16 items-center rounded-full transition', {
                    'bg-pink-50': mod.value,
                    'bg-pink-300': !mod.value
                })">
                    <span
                    :class="mod.value ? 'translate-x-6' : 'translate-x-1'"
                    class="inline-block h-6 w-6 transform rounded-full bg-pink-400 transition"
                    />
                </Switch>
            </SwitchGroup>
        </div>
    </div>
</template>