<template>
  <transition name="modal-animation">
    <div v-if="modalActive" class="modal">
      <div @click="$emit('close')" class="modal-outside"></div>
      <div class="modal-inner">
        <!-- Modal Content -->
        <slot />
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  props: ["modalActive"],
};
</script>

<style lang="scss" scoped>
.modal-animation-enter-active {
  transition: opacity 0.3s ease;
}
.modal-animation-leave-active {
  transition: opacity 0.3s ease;
}
.modal-animation-enter-from {
  opacity: 0;
}
.modal-animation-leave-to {
  opacity: 0;
}

.modal-inner {
  display: flex;
  justify-content: center;
}

.modal-outside {
  position: fixed;
  width: 100%;
  height: 100vh;
  left: 0;
  top: 0;
  z-index: 99;
}
</style>
