<template>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup lang="ts">
// Global app setup
useHead({
  titleTemplate: (title) => title ? `${title} - IT-Admin` : 'IT-Admin',
})
</script>
