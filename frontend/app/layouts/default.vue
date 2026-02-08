<template>
  <div>
    <ClientOnly>
      <button
        class="fixed top-4 right-4 p-2 rounded-md border border-border hover:bg-accent transition-colors"
        @click="toggleColorMode"
      >
        <Sun v-if="colorMode.value === 'dark'" class="size-5" />
        <Moon v-else class="size-5" />
      </button>
    </ClientOnly>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { Sun, Moon } from 'lucide-vue-next'

const colorMode = useColorMode()

function toggleColorMode() {
  colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'
}
</script>
