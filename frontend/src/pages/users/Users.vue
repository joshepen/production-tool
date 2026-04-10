<script setup lang="ts">

</script>

<template>
  <span>Users</span>
  <v-data-table
    :headers="[{ title: 'First Name', key: 'first_name' }, { title: 'Last Name', key: 'last_name' }]"
  />
</template>
