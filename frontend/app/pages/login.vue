<template>
  <div class="flex items-center justify-center h-screen">
    <template v-if="clerkEnabled">
      <SignedOut>
        <SignInButton
          class="rounded-lg border border-gray-900 px-10 py-2 hover:bg-gray-900 hover:text-white transition-colors"
        />
      </SignedOut>
      <SignedIn>
        <UserButton />
      </SignedIn>
    </template>
    <div v-else class="text-center space-y-2">
      <p class="text-lg font-medium">Auth is not configured</p>
      <p class="text-sm text-gray-500">
        Set <code class="bg-gray-100 dark:bg-gray-800 px-1 rounded">NUXT_PUBLIC_CLERK_ENABLED=true</code> with your Clerk keys to enable authentication.
      </p>
      <NuxtLink to="/" class="inline-block mt-4 text-sm underline">Go home</NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
const { clerkEnabled } = useRuntimeConfig().public

if (clerkEnabled) {
  const { isSignedIn, getToken } = useAuth()

  if (import.meta.client && isSignedIn.value) {
    await getToken.value()
    navigateTo("/")
  }
}
</script>
