<script setup lang="ts">

import { useTaskProgressStore } from '@store/taskProgressStore';
import syncSize from '@utils/syncSize';
import classNames from 'classnames'
import { watch } from 'fs';
import { storeToRefs } from 'pinia';
import { computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';

const taskProgressState = useTaskProgressStore();

const { processing, progress } = storeToRefs(taskProgressState)

// onMounted(async () => await syncSize())
// onUnmounted(async () => await syncSize())

// watch()

const progress_percent = computed(() => {
  const progress_value = progress.value?.progress;
  if (!progress_value) return 0

  if (progress_value.type === 'bool') return progress_value.content ? 1 : 0;

  if (progress_value.type === 'count') return (
    (progress_value.content[0] / progress_value.content[1]) * 100
  );

  if (progress_value.type === 'bytes') return (
    (Number(progress_value.content[0]) / Number(progress_value.content[1])) * 100
  );

  if (progress_value.type === 'percentage') return progress_value.content;

  return 100;
})

const bytes_to_megabytes = (bytes: bigint): string => {
  let number = (Number(bytes) / 1000000).toFixed(1)
  return `${number} МБ`
}

const progress_string = computed(() => {
  const progress_value = progress.value?.progress;

  // console.log(progress_value)

  if (!progress_value) return ""

  if (progress_value.type === 'bool') return "";

  if (progress_value.type === 'count') return `
    ${progress_value.content[0]} / ${progress_value.content[1]}
  `;

  if (progress_value.type === 'bytes') return `
    ${bytes_to_megabytes(progress_value.content[0])} / ${bytes_to_megabytes(progress_value.content[1])}
  `;

  if (progress_value.type === 'percentage') return `
    ${progress_value.content} / 100%
  `;

  return "";
})

const { t } = useI18n()

</script>


<template>

<div class="w-full fixed bottom-0">
    <div class="flex justify-between p-3 px-4 gap-3">
        <p class="text-xs text-ellipsis overflow-hidden whitespace-nowrap align-baseline">
            <span v-if="progress?.title">{{ t(progress.title) }}</span>
            <span v-if="progress?.subtitle">: {{ t(progress.subtitle) }}</span>
            <span>...</span>
        </p>
        <span class="text-xs whitespace-nowrap">{{progress_string}}</span>
    </div>
    <div class="w-full h-2.5 bg-pink-300">
        <div
            :class="classNames(`bg-pink-50 h-2.5`, {'animate-pulse': progress?.progress.type == 'indefinate'})"
            :style="`width: ${progress_percent}%`"/>
    </div>
</div>

</template>