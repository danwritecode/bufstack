<template>
  <div class="flex items-center justify-center h-screen">
    <SignedOut>
      <SignInButton
        class="rounded-lg border border-gray-900 px-10 py-2 hover:bg-gray-900 hover:text-white transition-colors"
      />
    </SignedOut>
    <SignedIn>
      <UserButton />
    </SignedIn>
  </div>
</template>

<script setup lang="ts">
const { isSignedIn, getToken } = useAuth()

if (import.meta.client && isSignedIn.value) {
  await getToken.value()
  navigateTo("/")
}
</script>
