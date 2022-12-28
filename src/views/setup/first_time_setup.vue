<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import Checkbox from "@/components/checkbox.vue";
import SettingsView from "@/views/settings/settings.vue";
import SetupStage from "@/views/setup/setup_stage.vue";
import { open } from "@tauri-apps/api/dialog";

const emit = defineEmits(["setupComplete"]);

const props = defineProps<{
  needsJavaSetup: boolean;
}>();

const javaSetupComplete = ref(false);
const activeIndex = ref(1);

const hasJava = computed(
  () => !props.needsJavaSetup || javaSetupComplete.value
);

if (!props.needsJavaSetup) {
  ++activeIndex.value;
}

function switchActiveIndex(newIndex: number) {
  activeIndex.value = newIndex;

  if (newIndex == 2) {
    javaSetupComplete.value = true;
  }

  if (newIndex == 3) {
    emit("setupComplete");
  }
}

async function selectServerDirectory() {
  const selected = await open({
    directory: true,
  });

  console.log(selected);
}
</script>

<template lang="pug">
.m-5
  h1.text-center.text-2xl.font-bold.mb-5 First Time Setup
  setup-stage(:index="1" title="Install Java" :active-index="activeIndex" @switch="switchActiveIndex")
    .flex
      checkbox(:is-active="hasJava")
      .ml-2
        p(v-if="hasJava") Java is installed.
        p(v-else)
          | You do not have Java installed, which is required to run Minecraft servers.
          | Visit
          |
          a.font-bold.text-blue-600.hover_underline.visited_text-purple-600(
            href="https://www.oracle.com/java/technologies/downloads/" target="_blank") Oracle
          |
          | to download and install Java.
  setup-stage(:index="2" title="Choose Settings" :active-index="activeIndex" @switch="switchActiveIndex")
    p.mb-4
      | Choose settings based on your personal preferences.
      | Practical defaults have already been selected for you, so you can skip this step if you're in a hurry.
    settings-view
</template>
