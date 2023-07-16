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
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 0px;
  background-color: rgba(65, 74, 169, 0.9);
  color: white;
  margin: 6px 6px 6px auto;
  padding: 8px 12px 4px 12px;
  display: flex;
  flex-direction: column;
  align-content: flex-end;
}

div > p {
  font-size: 10px;
  text-align: right;
  line-height: 1.5;
  margin: 0px;
}

div > span {
  font-size: 8px;
  text-align: right;
  margin: 0px;
}
</style>
