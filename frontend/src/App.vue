<template>
  <v-app>
    <Bar />
    <v-main :style="{ height: '100vh' }">
      <router-view />
    </v-main>
    <v-snackbar-queue v-model="messages" closable />
  </v-app>
</template>

<script lang="ts" setup>
  import { storeToRefs } from 'pinia'
  import Bar from '@/components/Bar.vue'
  import { useStatusMessageStore } from '@/stores/statusMessage'

  const { messages } = storeToRefs(useStatusMessageStore())
</script>
