<script setup lang="ts">
import CheckboxSvg from "@/assets/check.svg?component";

const props = defineProps<{
  isActive: boolean;
}>();
</script>

<template lang="pug">
.flex-none.flex.justify-center.items-center.border.rounded-md.w-6.h-6
  checkbox-svg.fill-blue-400.w-4.h-4(:class="{ 'invisible': !isActive }")
</template>
