<script setup lang="ts">

const props = defineProps<{
    disabled?: boolean
    placeholder?: string
}>()

</script>

<template>
    <input :placeholder="placeholder"/>
</template>