<template>
  <div>
    <p>
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed non risus.
    </p>
    <span>17/05/2023 07:30:29</span>
  </div>
</template>

<style scoped>
div {
  width: fit-content;
  max-width: 80%;
  border-top-left-radius: 12px;
  border-top-right-radius: 12px;
  border-bottom-left-radius: 0px;
  border-bottom-right-radius: 12px;
  background-color: rgba(23, 23, 23, 0.9);
  color: white;
  margin: 6px auto 6px 6px;
  padding: 8px 12px 4px 12px;
  display: flex;
  flex-direction: column;
  align-content: flex-start;
}

div > p {
  font-size: 10px;
  text-align: left;
  line-height: 1.5;
  margin: 0px;
}

div > span {
  font-size: 8px;
  text-align: left;
  margin: 0px;
}
</style>
