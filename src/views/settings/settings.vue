<script setup lang="ts">
import { ref } from "vue";
import CButton from "@/components/button.vue";
import Setting from "@/views/settings/setting.vue";

const serverDirectory = ref("");

async function selectServerDirectory() {}
</script>

<template lang="pug">
.border-2.border-violet-300
  setting(title="Server Directory")
    template(v-slot:body)
      p
        | The working directory of the Minecraft server.
        | If left empty, the directory will be created in a temporary location on server startup and deleted on server shutdown.
        | Otherwise, select a custom directory in which to keep the last world that was run on the server.
    template(v-slot:input)
      c-button(color="blue" @click="selectServerDirectory") Select
      p Current: {{ serverDirectory }}
</template>
