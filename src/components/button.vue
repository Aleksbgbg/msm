<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  color: string;
}>();

// Not string formatted to enable Tailwind detection
const colorClasses = {
  blue: ["bg-blue-400", "hover_bg-blue-500"],
  violet: ["bg-violet-400", "hover_bg-violet-500"],
};

const selectedColorClasses = computed(() => colorClasses[props.color]);
</script>

<template lang="pug">
button.rounded-md.text-white.px-2.py-1(:class="selectedColorClasses")
  slot
</template>
