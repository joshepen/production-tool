<template>
  <v-app>
    <v-main>
      <Bar />
      <router-view />
      <v-snackbar-queue
        v-model="messages"
        closable
      /></v-main></v-app>
</template>

<script lang="ts" setup>
  import Bar from '@/components/Bar.vue'
  import { useMessageStore } from '@/stores/message'

  const { messages } = useMessageStore()
</script>
