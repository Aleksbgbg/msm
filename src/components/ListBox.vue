<script setup lang="ts">
const props = defineProps(["items"]);
</script>

<template lang="pug">
slot(v-for="item of items" :item="item")
</template>
