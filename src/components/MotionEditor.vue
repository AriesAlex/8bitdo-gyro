<script setup lang="ts">
import { computed } from 'vue'
import { MousePointer2 } from 'lucide-vue-next'
import type { MotionSettings } from '../device'
import ButtonPicker from './ButtonPicker.vue'

const model = defineModel<MotionSettings>({ required: true })
defineProps<{ disabled: boolean }>()
const percent = computed(() => Math.floor((model.value.deadzone * 100) / 255))
</script>

<template>
  <fieldset class="editor" :disabled="disabled">
    <div class="setting">
      <label>Output</label>
      <div class="segments output">
        <button
          v-for="(label, index) in ['Left stick', 'Right stick', 'Mouse']"
          :key="index"
          :class="{ selected: model.mapping === index }"
          @click="model.mapping = index"
        >
          <MousePointer2 v-if="index === 2" :size="17" />
          <svg v-else viewBox="0 0 20 20" fill="none">
            <ellipse cx="10" cy="15" rx="7" ry="3" stroke="currentColor" stroke-width="1.5" />
            <path d="M10 13V7" stroke="currentColor" stroke-width="1.5" />
            <circle cx="10" cy="5" r="3" fill="currentColor" />
          </svg>
          {{ label }}
        </button>
      </div>
    </div>

    <div class="setting">
      <label>Activation</label>
      <div class="activation">
        <div class="segments">
          <button
            :class="{ selected: model.activationMode === 1 }"
            @click="model.activationMode = 1"
          >
            Hold
          </button>
          <button
            :class="{ selected: model.activationMode === 2 }"
            @click="model.activationMode = 2"
          >
            Toggle
          </button>
        </div>
        <ButtonPicker v-model="model.activationKey" :disabled="disabled" />
      </div>
    </div>

    <div class="setting slider-setting">
      <label for="sensitivity">Sensitivity</label>
      <div class="slider-control">
        <div class="slider-track">
          <input
            id="sensitivity"
            v-model.number="model.sensitivity"
            type="range"
            min="1"
            max="6"
            step="1"
            :style="{ '--fill': `${(model.sensitivity - 1) * 20}%` }"
          />
          <div class="steps">
            <span v-for="step in 6" :key="step">{{ step }}</span>
          </div>
        </div>
        <output>{{ model.sensitivity }}<small>/ 6</small></output>
      </div>
    </div>

    <div class="setting slider-setting">
      <label for="deadzone">Deadzone compensation</label>
      <div class="slider-control">
        <div class="slider-track">
          <input
            id="deadzone"
            v-model.number="model.deadzone"
            type="range"
            min="0"
            max="255"
            step="1"
            :style="{ '--fill': `${(model.deadzone * 100) / 255}%` }"
          />
          <div class="steps"><span>0%</span><span>100%</span></div>
        </div>
        <output>{{ percent }}<small>%</small></output>
      </div>
    </div>
  </fieldset>
</template>

<style scoped>
.editor {
  border: 0;
  padding: 0;
  margin: 0;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 25px;
}
.setting {
  display: flex;
  align-items: center;
  gap: 24px;
  min-width: 0;
  > label {
    width: 188px;
    flex: none;
    color: var(--muted);
    font-size: 15px;
  }
}
.segments {
  display: flex;
  padding: 3px;
  border-radius: 9px;
  background: var(--control);
  gap: 3px;
  button {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 8px;
    padding: 10px 15px;
    border-radius: 6px;
    color: var(--muted);
    white-space: nowrap;
    font-size: 14px;
  }
  button:hover {
    color: var(--text);
  }
  .selected {
    background: #44404e;
    color: var(--text);
    box-shadow: 0 1px 3px #0002;
  }
}
.output {
  flex: 1;
  button {
    flex: 1;
    padding-inline: 9px;
  }
  svg {
    width: 17px;
    height: 17px;
    flex: none;
  }
}
.activation {
  display: flex;
  flex: 1;
  gap: 14px;
  .segments {
    flex: 1;
  }
  .segments button {
    flex: 1;
  }
}
.slider-control {
  display: flex;
  flex: 1;
  align-items: flex-start;
  gap: 20px;
  min-width: 0;
}
.slider-track {
  flex: 1;
  min-width: 0;
}
input[type='range'] {
  appearance: none;
  width: 100%;
  height: 5px;
  border-radius: 4px;
  margin: 10px 0 0;
  cursor: pointer;
  background: linear-gradient(to right, var(--accent) var(--fill), #45414e var(--fill));
  &::-webkit-slider-thumb {
    appearance: none;
    width: 15px;
    height: 15px;
    background: var(--accent);
    border: 3px solid #211f27;
    border-radius: 50%;
    box-shadow: 0 0 0 1px var(--accent);
  }
}
.steps {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: var(--dim);
  margin-top: 12px;
  padding-inline: 2px;
  font-variant-numeric: tabular-nums;
}
output {
  width: 54px;
  text-align: right;
  font-size: 25px;
  font-weight: 500;
  line-height: 27px;
  font-variant-numeric: tabular-nums;
  small {
    font-size: 11px;
    color: var(--muted);
    margin-left: 4px;
  }
}
.editor:disabled {
  opacity: 0.4;
}
</style>
