<template>
  <v-app>
    <v-main>
						<Bar />
      <router-view />
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
  import Bar from "@/components/Bar.vue";
</script>
