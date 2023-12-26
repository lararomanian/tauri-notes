<template>
  <div class="note-list">
    <button @click="addNewNote" class="btn btn-block">+</button>
    <Note v-for="note in notes" :key="note.id" :note="note" @editNote="editNote" @delete="deleteNote" @reorder="updateNoteOrder" />
  </div>
</template>

<script setup>
import { ref, onMounted, defineProps} from 'vue';
import Note from './Note.vue';
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps(['notes']);

const emit = defineEmits(['newNote'])

const addNewNote = () => {
  emit('newNote');
};

const editNote = (id) => {
  emit('editNote', id);
};

const deleteNote = (id) => {
  emit('deleteNote', id);
};

onMounted(() => {
})

</script>

<style scoped>
.note-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
}

.btn-primary {
  background-color: #3498db;
  color: #fff;
  padding: 10px 15px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s ease;

  &:hover {
    background-color: #2b7dbd;
  }
}

.btn-block {
  background-color : lightblue;
  color: white;
  font-size: 42px;
}
</style>

