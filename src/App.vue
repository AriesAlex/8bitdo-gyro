<script setup lang="ts">
import { useTemplateRef, watch } from 'vue'
import { RefreshCw, Crosshair, Check, ArrowRight } from 'lucide-vue-next'
import GyroMark from './components/GyroMark.vue'
import ToggleSwitch from './components/ToggleSwitch.vue'
import MotionEditor from './components/MotionEditor.vue'
import { useController } from './useController'

const {
  saved,
  drafts,
  profile,
  motion,
  pending,
  error,
  notice,
  dirty,
  dirtyProfiles,
  reload,
  save,
  toggleTournament,
  startCalibration,
  discard,
} = useController()
const calibrationDialog = useTemplateRef<HTMLDialogElement>('calibrationDialog')
watch(profile, () => {
  notice.value = ''
})

function openCalibration() {
  error.value = ''
  calibrationDialog.value?.showModal()
}

function cancelCalibration(event: Event) {
  if (pending.value) event.preventDefault()
}

async function calibrate() {
  await startCalibration()
  if (!error.value) calibrationDialog.value?.close()
}
</script>

<template>
  <main>
    <header class="app-header">
      <div class="brand">
        <GyroMark /><span>8bitdo <b>gyro</b></span>
      </div>
      <button
        class="icon-button"
        :disabled="!!pending"
        title="Reload from controller"
        @click="reload"
      >
        <RefreshCw :size="17" :class="{ spinning: pending === 'load' }" />
      </button>
    </header>

    <template v-if="saved">
      <div class="device-row">
        <span>Ultimate 3 <span class="subtle">for Xbox</span></span>
        <label class="tournament"
          >Tournament mode
          <ToggleSwitch
            :model-value="saved.tournament"
            :disabled="!!pending"
            label="Tournament mode"
            @update:model-value="toggleTournament"
        /></label>
      </div>

      <div class="workspace">
        <nav class="profiles">
          <button
            v-for="index in 3"
            :key="index"
            :class="{ current: profile === index - 1 }"
            :disabled="!!pending"
            @click="profile = index - 1"
          >
            Profile {{ index }}{{ dirtyProfiles[index - 1] ? ' *' : '' }}
          </button>
        </nav>

        <div class="motion-heading">
          <h1>Gyroscope</h1>
          <ToggleSwitch
            :model-value="motion.enabled"
            :disabled="!!pending"
            label="Gyroscope"
            @update:model-value="motion.enabled = $event"
          />
        </div>
        <p v-if="saved.reconnectRequired" class="mode-note">
          Mode saved. Reconnect the controller and receiver, then refresh.
        </p>
        <button
          v-else-if="!saved.tournament"
          class="mode-note"
          :disabled="!!pending"
          @click="toggleTournament"
        >
          Switch to Tournament mode <ArrowRight :size="15" />
        </button>

        <MotionEditor v-model="drafts[profile]" :disabled="!!pending || !motion.enabled" />

        <footer>
          <button class="text-button" :disabled="!!pending" @click="openCalibration">
            <Crosshair :size="17" /> Calibrate
          </button>
          <div class="save-actions">
            <button v-if="dirty" class="text-button" :disabled="!!pending" @click="discard">
              Discard
            </button>
            <button
              class="primary"
              :disabled="!!pending || (motion.enabled && motion.activationKey === 0)"
              @click="save"
            >
              {{ pending === 'save' ? 'Saving…' : 'Save' }}
            </button>
          </div>
        </footer>
      </div>
      <div class="feedback">
        <p v-if="error" class="error">{{ error }}</p>
        <p v-else-if="pending === 'mode'">Switching mode…</p>
        <p v-else-if="notice"><Check :size="14" />{{ notice }}</p>
      </div>
    </template>

    <div v-else class="empty-state">
      <GyroMark :class="{ searching: pending === 'load' }" />
      <h1>{{ pending ? 'Reading controller…' : 'Connect your controller' }}</h1>
      <p>{{ error || 'Ultimate 3 for Xbox · USB or 2.4G' }}</p>
      <button v-if="!pending" class="primary" @click="reload">
        <RefreshCw :size="15" /> Refresh
      </button>
    </div>

    <dialog ref="calibrationDialog" class="calibration-dialog" @cancel="cancelCalibration">
      <Crosshair :size="30" />
      <h2>Calibrate gyroscope</h2>
      <p>
        {{
          pending === 'calibrate'
            ? 'Keep the controller still…'
            : 'Place the controller on a flat surface.'
        }}
      </p>
      <p v-if="error" class="error">{{ error }}</p>
      <div class="calibration-actions">
        <button class="text-button" :disabled="!!pending" @click="calibrationDialog?.close()">
          Cancel
        </button>
        <button class="primary" :disabled="!!pending" @click="calibrate">
          {{ pending === 'calibrate' ? 'Calibrating…' : 'Start calibration' }}
        </button>
      </div>
    </dialog>
  </main>
</template>

<style scoped>
main {
  height: 100vh;
  display: flex;
  flex-direction: column;
  padding: 24px 32px 12px;
}
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}
.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 23px;
  letter-spacing: -0.6px;
  svg {
    width: 35px;
    height: 35px;
    color: var(--accent);
  }
  b {
    font-weight: 400;
    color: var(--accent);
  }
}
.icon-button {
  display: flex;
  padding: 9px;
  border-radius: 7px;
  color: var(--muted);
  &:hover {
    background: var(--control);
    color: var(--text);
  }
}
.device-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 15px;
  gap: 20px;
  margin-bottom: 24px;
}
.subtle {
  color: var(--muted);
}
.tournament {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 14px;
  color: var(--muted);
}
.workspace {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--surface);
  border-radius: 14px;
  padding: 0 28px 22px;
}
.profiles {
  display: flex;
  gap: 27px;
  border-bottom: 1px solid var(--line);
  margin-bottom: 22px;
  button {
    font-size: 14px;
    color: var(--muted);
    padding: 19px 0 16px;
    border-bottom: 2px solid transparent;
  }
  .current {
    color: var(--accent);
    border-bottom-color: var(--accent);
  }
}
.motion-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 22px;
}
h1 {
  font-size: 27px;
  font-weight: 500;
  letter-spacing: -0.6px;
  margin: 0;
}
.mode-note {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--accent);
  margin: 0 0 22px;
  text-align: left;
}
footer {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 20px;
  margin-top: auto;
  padding-top: 22px;
}
.calibration-actions,
.save-actions {
  display: flex;
  align-items: center;
  gap: 14px;
}
.feedback {
  min-height: 48px;
  flex: none;
  padding: 10px 4px 0;
  p {
    display: flex;
    align-items: center;
    gap: 6px;
    margin: 0;
    font-size: 13px;
    color: var(--muted);
    line-height: 1.5;
  }
  .error {
    color: #e8a4a4;
  }
}
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  gap: 22px;
  > svg {
    color: var(--accent);
    width: 80px;
    height: 80px;
    margin-bottom: 10px;
  }
  p {
    font-size: 15px;
    color: var(--muted);
    line-height: 1.6;
    margin: 0;
    max-width: 400px;
  }
}
.spinning {
  animation: rotate 1s linear infinite;
}
.searching {
  animation: rotate 3s linear infinite;
}
@keyframes rotate {
  to {
    transform: rotate(360deg);
  }
}
.calibration-dialog {
  width: 400px;
  padding: 30px;
  background: #2d2a33;
  border: 1px solid #494350;
  border-radius: 14px;
  color: var(--text);
  &::backdrop {
    background: #101014b8;
  }
  > svg {
    color: var(--accent);
  }
  h2 {
    font-size: 22px;
    font-weight: 500;
    margin: 19px 0 12px;
  }
  p {
    font-size: 14px;
    color: var(--muted);
    line-height: 1.6;
    margin: 0 0 24px;
  }
  .error {
    color: #e8a4a4;
  }
  .calibration-actions {
    justify-content: space-between;
  }
}
</style>
