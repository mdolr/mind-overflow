<template>
  <div>
    <button class="button active">Transcript</button>
    <button class="button" disabled>Chat</button>
  </div>
</template>

<style scoped>
div {
  background-color: #111;
  border: solid 1px rgba(47, 47, 47, 0.5);
  width: 100%;
  height: 36px;
  display: flex;
  align-items: center;
}

.button {
  width: 50%;
  border: none;
  height: 36px;
  background-color: rgba(47, 47, 47);
  /* All borders except top*/
  border-bottom: solid 1px rgba(229, 231, 235, 0.25);
  transition: 0.15s;
  cursor: pointer;
}

.active,
.button:hover {
  background-color: rgba(47, 47, 47, 0.5);
}
</style>
