<template>
  <div class="note-editor">
    <button @click="saveNote">Save</button>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps(['note']);

const saveNote = () => {
};
</script>

<style scoped>
.note-editor {
}
</style>

