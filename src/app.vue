<script setup lang="ts">
import { reactive } from "vue";
import {
  type FirstTimeSetup,
  requiresFirstTimeSetup,
} from "@/native/first_time_setup";
import FirstTimeSetupView from "@/views/setup/first_time_setup.vue";
import WorldsView from "@/views/worlds.vue";

const setup = reactive({
  fetched: false,
  info: {
    required: false,
    hasJava: false,
  },
});

requiresFirstTimeSetup().then((firstTimeSetup: FirstTimeSetup) => {
  setup.fetched = true;
  setup.info = firstTimeSetup;
});
</script>

<template lang="pug">
.select-none
  div(v-if="setup.fetched")
    first-time-setup-view(
      v-if="setup.info.required" :needs-java-setup="!setup.info.hasJava" @setup-complete="setup.info.required = false")
    worlds-view(v-else)
  p(v-else) Waiting...
</template>
