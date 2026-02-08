<template>
  <div class="min-h-screen flex items-center justify-center">
    <Card class="w-full max-w-md">
      <CardHeader class="text-center">
        <CardTitle class="text-4xl font-bold">Bufstack</CardTitle>
        <CardDescription>Rust gRPC + Nuxt 4 + Protocol Buffers</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <Button class="w-full" @click="sayHello">
          Say Hello
        </Button>

        <Alert v-if="message">
          <AlertTitle>Response</AlertTitle>
          <AlertDescription>{{ message }}</AlertDescription>
        </Alert>
      </CardContent>
      <CardFooter class="justify-center gap-2">
        <Badge variant="secondary">Rust</Badge>
        <Badge variant="secondary">gRPC</Badge>
        <Badge variant="secondary">Nuxt 4</Badge>
        <Badge variant="secondary">Protobuf</Badge>
      </CardFooter>
    </Card>
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
