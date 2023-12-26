<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Dashboard from "./components/Dashboard.vue";
import NoteList from "./components/NoteList.vue"
</script>

<template>
  <div class="container">
    <h1>Notes App - Tauri/Vue(ts)</h1>

    <div class="row">
    </div>

    <p>This is a notes app using vue js as a CLI Application</p>

    <Dashboard />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
