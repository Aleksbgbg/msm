<script setup lang="ts">
const props = defineProps<{
  title: string;
}>();
</script>

<template lang="pug">
.grid.grid-cols-6
  h3.font-semibold.self-center.justify-self-center {{ title }}
  .col-span-4
    slot(name="body")
  .self-center.justify-self-center
    slot(name="input")
</template>
