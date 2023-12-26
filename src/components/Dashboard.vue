<script setup lang="ts">
import { ref, onMounted, defineEmits } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import NoteList from "./NoteList.vue";
import NewNote from "./NewNote.vue";

const refCheck = ref("");
const name = ref("");
const emit = defineEmits(['newNote']);
const isDialogVisible = ref(false);
const isNewNote = ref(false);
const notes = ref([
]);

async function search() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function getNotes() {
  try {
    notes.value = await invoke("get_all_notes");
  } catch (error) {
    console.log(error, "error");  
  }
}

const showNewNoteDialog = () => {
  isDialogVisible.value = true;
}

const showEditNoteDialog = () => {
  isDialogVisible.value = true;
  isNewNote.value = true; 
}

const closeNewNoteDialog = () => {
  isDialogVisible.value = false;
}

onMounted(() => {
    getNotes();
})

</script>

<template>
  <form class="row" @submit.prevent="search">
    <input id="greet-input" v-model="name" placeholder="Search for a note.." />
    <button type="submit" class="btn btn-primary">Search</button>
  </form>

  <p>{{ refCheck }}</p>

  <NoteList :notes="notes" @newNote="showNewNoteDialog"/>

  <NewNote :visible="isDialogVisible" :isNewNote="isNewNote" @fetchData="getNotes" @close="closeNewNoteDialog"/>

</template>

<style scoped>

  .btn-primary {
    background-color : blue;
    color : white;
  }
  
</style>
