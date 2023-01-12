<script setup lang="ts">
import classNames from 'classnames';
import { computed } from 'vue';

import LoadingIcon from '@components/common/icons/LoadingIcon.vue'

type ButtonType = 'main' | 'main_danger' | 'main_warn' | 'secondary'

const props = defineProps<{
    disabled?: boolean
    loading?: boolean
    type?: ButtonType
}>()

const buttonClasses = computed(() => {
    const base = "btn btn-primary btn-lg btn-shadow-hide"
    return classNames(base, "")

    // if (!props.type || props.type === 'main')
    // return classNames(base, "text-pink-600 bg-pink-50 hover:bg-pink-100 active:bg-pink-200")
    //
    // if (props.type === 'main_danger')
    // return classNames(base, "text-rose-600 bg-rose-300 hover:bg-rose-200 active:bg-rose-400")
    //
    // if (props.type === 'main_warn')
    // return classNames(base, "text-amber-600 bg-amber-200 hover:bg-amber-100 active:bg-amber-300")
    //
    // if (props.type === 'secondary')
    // return classNames(base, "text-white bg-pink-300 hover:bg-pink-200 active:bg-pink-300")
})

</script>

<template>
    <button :disabled="disabled || loading" :class="buttonClasses">
        <LoadingIcon class="fill-pink-50 animate-spin" v-if="loading"/>
        <slot v-else />
    </button>
</template>