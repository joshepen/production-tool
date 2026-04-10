<script setup lang="ts">
  import { useBackendStore } from '@/stores/backend'
  import { useMessageStore } from '@/stores/message'
  const backend = useBackendStore()
  const messageStore = useMessageStore()

</script>

<template>
  <span>Users</span>
</template>
