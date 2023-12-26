<template>
  <div class="dialog-overlay" v-if="visible">
    <div class="dialog">
      <h2>{{ isNewNote ? 'New Note' : 'Edit Note' }}</h2>
      <form @submit.prevent="saveNote">
        <label for="title">Title:</label>
        <input v-model="formData.title" type="text" id="title" />
          <br>
          <br>
          <br>
        <label for="content" >Description:</label>
        <input v-model="formData.description" type="text" id="description" />

        <div class="button-container">
          <button type="submit" class="btn btn-primary">{{ isNewNote ? 'Create Note' : 'Save Changes' }}</button>
          <button @click="closeDialog" class="btn btn-secondary">Cancel</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, defineProps, defineEmits } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps(['visible']);
const emit = defineEmits(['close', 'newNote']);

const dialogVisible = ref(props.visible);
const isNewNote = ref(true);

const formData = ref({
  title: '',
  description: '',
  created_at: 'today',
  updated_at: 'today',
});


const formDataStructure = ref({
  title: '',
  description: '',
  created_at: 'today',
  updated_at: 'today',
});

const openDialog = (isCreatingNewNote = true, existingNote = null) => {
  isNewNote.value = isCreatingNewNote;
  formData.value = { ...existingNote }; 
  dialogVisible.value = true;
};

const closeDialog = () => {
  formData.value = { ...formDataStructure.value };
  emit('close');
};

async function saveNote() {
  console.log("formData.value", formData.value);
  try {
    await invoke("save_note", { note: formData.value});
  } catch (error) {
    console.log(error, "error");
  }
  emit('newNote');
  emit('fetchData');
  closeDialog();
};

</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.dialog {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  max-width: 400px;
  width: 100%;
}

.button-container {
  margin-top: 15px;
  display: flex;
  justify-content: space-between;
}

.btn {
  padding: 10px 15px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary {
  background-color: #3498db;
  color: #fff;
}

.btn-secondary {
  background-color: #95a5a6;
  color: #fff;
}
</style>

