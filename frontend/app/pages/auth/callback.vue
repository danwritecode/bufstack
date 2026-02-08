<template>
  <div class="flex flex-col items-center justify-center h-screen gap-4">
    <div class="size-8 border-4 border-gray-300 border-t-gray-600 rounded-full animate-spin" />
    <p class="text-sm text-gray-500">Signing you in...</p>
  </div>
</template>

<script setup lang="ts">
const { isSignedIn, getToken } = useAuth()

onMounted(() => {
  watch(isSignedIn, async (signedIn) => {
    if (signedIn) {
      await getToken.value()
      navigateTo("/")
    }
  }, { immediate: true })
})
</script>
