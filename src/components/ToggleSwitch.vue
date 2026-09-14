<script setup lang="ts">
import { GripVertical } from 'lucide-vue-next'
defineProps<{ modelValue: boolean; disabled?: boolean; label: string }>()
defineEmits<{ 'update:modelValue': [value: boolean] }>()
</script>

<template>
  <button
    class="switch"
    :class="{ on: modelValue }"
    :disabled="disabled"
    :title="label"
    @click="$emit('update:modelValue', !modelValue)"
  >
    <span class="state">{{ modelValue ? 'On' : 'Off' }}</span>
    <span class="lever"><GripVertical :size="14" /></span>
  </button>
</template>

<style scoped>
.switch {
  position: relative;
  width: 72px;
  height: 30px;
  flex: none;
  border: 1px solid #504957;
  border-radius: 6px;
  background: #302c36;
  font-size: 12px;
  font-weight: 500;
  color: var(--muted);
  .state {
    display: block;
    padding-left: 22px;
  }
  .lever {
    position: absolute;
    inset: 3px auto 3px 3px;
    width: 21px;
    display: grid;
    place-items: center;
    border-radius: 3px;
    background: #665e70;
    color: #2b2631;
    transition: transform 150ms;
  }
  &.on {
    color: var(--accent);
    border-color: #7c6b98;
    .state {
      padding: 0 22px 0 0;
    }
    .lever {
      transform: translateX(43px);
      background: var(--accent);
    }
  }
  &:hover:not(:disabled) {
    border-color: var(--accent);
  }
}
</style>
