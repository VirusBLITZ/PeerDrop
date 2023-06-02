<template>
  <main>
    <h1>Peer-Drop</h1>
    <h3>clients</h3>
    <ul>
      <li v-for="client in clients">{{ client }}</li>
    </ul>
  </main>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

const clients = ref([]);

const scan_peers = async () => {
  clients.value = await invoke('scan_peers')
}
scan_peers()
</script>

<style>
body {
  background-color: #313030;
  font-family: sans-serif;
  color: white;
}
</style>