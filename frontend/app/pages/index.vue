<template>
  <div class="min-h-screen flex items-center justify-center">
    <div class="text-center space-y-6">
      <h1 class="text-4xl font-bold">Bufstack</h1>
      <p class="text-gray-500">Rust gRPC + Nuxt 4 + Protocol Buffers</p>

      <div class="space-y-4">
        <button
          type="button"
          class="px-4 py-2 border border-gray-900 hover:bg-gray-900 hover:text-white transition-colors"
          @click="sayHello"
        >
          Say Hello
        </button>

        <div v-if="message" class="p-4 bg-green-50 border border-green-200 rounded-lg">
          {{ message }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Greeter } from "~/gen/helloworld_pb";

const message = ref("");

async function sayHello() {
  const client = await useGrpcClient(Greeter);
  const res = await client.sayHello({ name: "World", message: "Hello from Bufstack!" });
  message.value = res.message;
}
</script>
