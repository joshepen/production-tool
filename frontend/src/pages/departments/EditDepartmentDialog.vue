<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useQueryClient } from '@tanstack/vue-query'
import { useBackendApiStore } from '@/stores/backendApi'
import { useStatusMessageStore } from '@/stores/statusMessage'

const props = defineProps<{ open: boolean; id: number }>()
const emit = defineEmits(['update:open'])

const backendApiStore = useBackendApiStore()
const messageStore = useStatusMessageStore()
const queryClient = useQueryClient()

const name = ref<string>('')

async function loadDepartment() {
  if (!props.id) return
  const res = await backendApiStore.get(`/department/${props.id}`)
  name.value = res.data.name
}

onMounted(() => {
  loadDepartment()
})

function close() {
  emit('update:open', false)
}

function saveDepartment() {
  backendApiStore.put(`/department/${props.id}`, { name: name.value }).then(() => {
    messageStore.displayMessage('Successfully updated department', 'success')
    queryClient.invalidateQueries({ queryKey: ['departments'] })
    close()
  })
}

watch(() => props.id, () => {
  if (props.open) {
    loadDepartment()
  }
})
</script>

<template>
  <v-dialog :model-value="open" width="auto" @update:model-value="$emit('update:open', $event)">
    <v-card>
      <v-card-item><v-card-title>Edit Department</v-card-title></v-card-item>
      <div
        :style="{
          display: 'flex',
          flexDirection: 'column',
          gap: '20px',
          alignItems: 'start',
          marginInline: '15px',
        }"
      >
        <v-text-field
          v-model="name"
          density="compact"
          hide-details="auto"
          label="Department Name"
          :style="{ minWidth: '250px' }"
        />
      </div>
      <template #actions>
        <v-btn color="indigo-lighten-1" @click="saveDepartment">Confirm</v-btn>
        <v-btn color="red-lighten-1" @click="close">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>