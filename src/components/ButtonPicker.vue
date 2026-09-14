<script setup lang="ts">
import { computed, ref } from 'vue'
import { ChevronDown } from 'lucide-vue-next'
import { PopoverContent, PopoverPortal, PopoverRoot, PopoverTrigger } from 'reka-ui'
import { activationButtons } from '../device'

const model = defineModel<number>({ required: true })
defineProps<{ disabled: boolean }>()
const open = ref(false)
const label = computed(
  () =>
    activationButtons.find((button) => button.value === model.value)?.label ??
    (model.value === 0 ? 'Button' : `0x${model.value.toString(16).toUpperCase()}`),
)

function choose(value: number) {
  model.value = value
  open.value = false
}
</script>

<template>
  <PopoverRoot v-model:open="open">
    <PopoverTrigger class="trigger" :disabled="disabled" title="Activation button">
      <span>{{ label }}</span
      ><ChevronDown :size="16" />
    </PopoverTrigger>
    <PopoverPortal>
      <PopoverContent as-child align="end" :side-offset="8" :collision-padding="16">
        <div class="picker">
          <button
            v-for="button in activationButtons"
            :key="button.value"
            :class="{ selected: model === button.value }"
            @click="choose(button.value)"
          >
            {{ button.label }}
          </button>
        </div>
      </PopoverContent>
    </PopoverPortal>
  </PopoverRoot>
</template>

<style scoped>
.trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  width: 120px;
  flex: none;
  padding: 0 13px;
  background: var(--control);
  border: 1px solid var(--line);
  border-radius: 7px;
  font-size: 14px;
  span {
    overflow: hidden;
    text-overflow: ellipsis;
  }
  svg {
    flex: none;
    color: var(--muted);
    transition: transform 140ms;
  }
  &:hover,
  &[data-state='open'] {
    border-color: #7c6b98;
  }
  &[data-state='open'] svg {
    transform: rotate(180deg);
  }
}
.picker {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 6px;
  width: 344px;
  padding: 10px;
  border: 1px solid #514859;
  border-radius: 10px;
  background: #2d2933;
  box-shadow: 0 12px 36px #0006;
  button {
    min-height: 36px;
    border-radius: 5px;
    background: #37323e;
    color: var(--muted);
    font-size: 13px;
    &:hover,
    &:focus-visible {
      background: #494050;
      color: var(--text);
    }
    &.selected {
      background: var(--accent);
      color: #292132;
    }
  }
}
</style>
