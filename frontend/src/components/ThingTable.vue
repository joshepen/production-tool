<script setup lang="ts">
  import type { Header } from '@/types/ThingTableTypes'
  import { ref } from 'vue'
  const searchValue = ref('')
  const props = defineProps<{ headers: Header[], query: object }>()
</script>
<template>
  <div :style="{display: 'flex', flexDirection: 'column', gap: '20px', alignItems: 'center'}">
    <div :style="{display: 'flex', alignItems: 'center', gap: '10px'}">
      <v-text-field
        v-model="searchValue"
        append-inner-icon="mdi-magnify"
        clearable
        hide-details="auto"
        :style="{minWidth: '400px', maxWidth: '450px'}"
      />
      <v-btn
        color="indigo-lighten-1"
        density="default"
        icon="mdi-plus-thick"
        rounded="lg"
        size="large"
      />
      <v-btn
        color="red-lighten-1"
        density="default"
        icon="mdi-delete"
        rounded="lg"
        size="large"
      />
    </div>
    <v-data-table
      :headers
      :items="query.data.value ?? []"
      :search="searchValue"
    >
      <template
        v-for="header in headers.filter(h => h.isDate || h.isDatetime)"
        :key="header.key"
        #[`item.${header.key}`]="{ item }"
      >
        {{ header.isDate ? new Date(item[header.key]).toDateString() : new Date(item[header.key]).toString() }}
      </template>
    </v-data-table>
  </div>
</template>
