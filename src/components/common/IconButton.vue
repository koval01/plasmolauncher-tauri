<script setup lang="ts">import classNames from 'classnames';
import { computed } from 'vue';


type IconButtonType = 'main' | 'secondary'

const props = defineProps<{
    type?: IconButtonType
}>()

const classes = computed(() => {
    const base = "p-3 rounded-lg transition flex items-center gap-2 font-semibold"

    if (!props.type || props.type === 'main')
    return classNames(base, "bg-pink-300 text-pink-500 hover:bg-pink-200 active:bg-pink-300")

    if (props.type === 'secondary')
    return classNames(base, "hover:bg-neutral-800 active:bg-neutral-700 text-neutral-200")
})

</script>



<template>

<div :class="classes">
    <slot />
</div>

</template>