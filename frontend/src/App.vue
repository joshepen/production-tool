<template>
  <v-app>
    <v-main :style="{height: '100vh'}">
      <Bar />
      <router-view />
      <v-snackbar-queue
        v-model="messages"
        closable
      /></v-main></v-app>
</template>

<script lang="ts" setup>
  import Bar from '@/components/Bar.vue'
  import { useStatusMessageStore } from '@/stores/statusMessage'

  const { messages } = useStatusMessageStore()
</script>
