<template>
  <v-app>
    <v-main>
      <Bar />
      <router-view />
      <v-snackbar
        v-model="backend.snackbar.show"
        :color="backend.snackbar.color"
        timeout="10000"
      >
        {{ backend.snackbar.text }}
        <template #actions>
          <v-btn variant="text" @click="backend.snackbar.show = false">Close</v-btn>
        </template>
      </v-snackbar>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
  import Bar from '@/components/Bar.vue'
  import { useBackendStore } from '@/stores/backend'

  const backend = useBackendStore()
</script>
