<script setup lang="ts">
import { reactive } from "vue";
import { listen } from "@tauri-apps/api/event";

const state = reactive({ text: "", output: [] });

const regex = /^(\[.*]) (\[.*]:) (.*)$/gm;

listen("output", (event) => {
  console.log(event.payload);

  for (const match of event.payload.line.matchAll(regex)) {
    // If no matches, log anyway but in a special format?
    state.output.push({
      time: match[1],
      thread: match[2],
      info: match[3],
    });
  }
});
</script>

<template lang="pug">
div
  .bg-slate-800.text-slate-300.rounded-lg.px-5.py-3.m-2
    p(v-for="line of state.output")
      span.text-red-300 {{ line.time }}
      | 
      span.text-green-300 {{ line.thread }}
      | 
      span {{ line.info }}
</template>
