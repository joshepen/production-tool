<script setup lang="ts">
  import type { Header } from '@/types/ThingTableTypes'
  import { ref } from 'vue'
  import DeleteButton from '@/components/DeleteButton'
  const searchValue = ref('')
  const props = defineProps<{ headers: Header[], query: object, getName: Function }>()
  const headers = ref([...props.headers, { key: 'delete' }])
  const dialogOpen = ref(false)

  defineEmits(['delete'])
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
        @click="dialogOpen= true"
      />
    </div>
    <slot :close="() => dialogOpen = false" name="create-dialog" :open="dialogOpen" />
    <v-data-table-virtual
      fixed-header
      :headers
      height="450"
      :items="query.data.value ?? []"
      :search="searchValue"
    >
      <template
        v-for="header in headers.filter(h => h.isDate || h.isDatetime)"
        :key="header.key"
        #[`item.${header.key}`]="{ item }"
      >
        {{ header.isDate ? new Date(item[header.key]).toDateString() : new Date(item[header.key]).toLocaleString(undefined, {
          year: 'numeric',
          month: 'short',
          day: 'numeric',
          hour: '2-digit',
          minute: '2-digit'
        }) }}
      </template>
      <template #item.delete="thing">
        <DeleteButton :name="getName(thing.item)" @confirm="$emit('delete',thing.item.id)" />
      </template>
    </v-data-table-virtual></div>
</template>
