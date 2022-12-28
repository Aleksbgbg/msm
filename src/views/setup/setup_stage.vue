<script setup lang="ts">
import { computed } from "vue";
import CButton from "@/components/button.vue";

const props = defineProps<{
  index: number;
  title: string;
  activeIndex: number;
}>();

const isActive = computed(() => props.index === props.activeIndex);
</script>

<template lang="pug">
.flex.transition-opacity.duration-300.delay-300.mb-2(:class="isActive ? [] : ['opacity-50', 'pointer-events-none']")
  h2.flex-none.text-center.text-xl.font-bold.rounded-md.text-white.bg-violet-300.h-8.w-8 {{ index }}
  .ml-2
    h2.inline-block.text-xl.font-bold {{ title }}
    .my-4
      slot
    c-button.transition-all.duration-150(
      color="violet" :class="{ 'scale-y-0': !isActive }" @click="$emit('switch', index + 1)") Continue
</template>
