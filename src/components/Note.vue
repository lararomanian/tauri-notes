<template>
  <div class="note">
    <div class="note-header">
      <h3>{{ note.title }}</h3>
      <div class="note-actions">
        <button @click="editNote" class="action-button">Edit</button>
        <button @click="deleteNote" class="action-button delete-button">Delete</button>
      </div>
    </div>
    <div class="note-content">
      <p>{{ note.content }}</p>
      <p class="note-metadata">Created: {{ note.created_at }}, Updated: {{ note.updated_at  }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps(['note']);

const editNote = () => {
  emit('editNote', props.note.id);
};

const deleteNote = () => {
  emit('deleteNote', props.note.id);
};

</script>

<style scoped>
.note {
  display: flex;
  flex-direction: column;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 15px;
  background-color: #fff;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.3s ease;

  &:hover {
    box-shadow: 0 8px 12px rgba(0, 0, 0, 0.2);
  }
}

.note-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.note-actions {
  display: flex;
  gap: 10px;
}

.action-button {
  padding: 8px 12px;
  border: none;
  border-radius: 4px;
  background-color: #3498db;
  color: #fff;
  cursor: pointer;
  transition: background-color 0.3s ease;

  &:hover {
    background-color: #2b7dbd;
  }
}

.delete-button {
  background-color: #e74c3c;

  &:hover {
    background-color: #c0392b;
  }
}

.note-content {
  margin-top: 10px;
}

.note-metadata {
  font-size: 0.8em;
  color: #777;
}

/* Add more styles as needed */
</style>

