<script setup lang="ts">
import { useUserStore } from "./stores/user";

const userStore = useUserStore();

userStore.updateUser();
</script>

<template>
  <p v-if="userStore.user">
    <span>Hello, </span>
    <strong>{{ userStore.user.username }}</strong>
    <span>!</span>
  </p>
  <pre class="info" v-if="userStore.user">{{ userStore.user }}</pre>
  <p v-else>Please login.</p>

  <div v-if="userStore.user">
    <form action="/api/logout" method="post">
      <button>Logout</button>
      <input type="hidden" name="accessToken" :value="userStore.user.accessToken" />
    </form>
    <button @click="userStore.refresh">Refresh</button>
  </div>
  <form action="/api/login" method="post" v-else>
    <button>Login</button>
    <input type="hidden" name="next" value="http://localhost:5173" />
  </form>
</template>

<style scoped>
pre.info {
  max-width: 80em;
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
