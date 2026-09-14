import { computed, onMounted, ref } from 'vue'
import {
  calibrate,
  readSettings,
  saveMotion,
  setTournament,
  type ControllerSettings,
  type MotionSettings,
} from './device'

export function useController() {
  const saved = ref<ControllerSettings | null>(null)
  const drafts = ref<MotionSettings[]>([])
  const profile = ref(0)
  const pending = ref<'load' | 'save' | 'mode' | 'calibrate' | null>(null)
  const error = ref('')
  const notice = ref('')
  const motion = computed(() => drafts.value[profile.value])
  const dirtyProfiles = computed(() =>
    drafts.value.map(
      (draft, index) => JSON.stringify(draft) !== JSON.stringify(saved.value?.profiles[index]),
    ),
  )
  const dirty = computed(() => dirtyProfiles.value[profile.value] ?? false)

  async function run(operation: NonNullable<typeof pending.value>, action: () => Promise<void>) {
    if (pending.value) return
    pending.value = operation
    error.value = ''
    notice.value = ''
    try {
      await action()
    } catch (reason) {
      error.value = String(reason)
    } finally {
      pending.value = null
    }
  }

  async function reload() {
    await run('load', async () => {
      const changed = dirtyProfiles.value
      const result = await readSettings()
      drafts.value = result.profiles.map((settings, index) =>
        changed[index] ? drafts.value[index] : { ...settings },
      )
      saved.value = result
    })
  }

  async function save() {
    const index = profile.value
    await run('save', async () => {
      saved.value = await saveMotion(index, { ...motion.value })
      drafts.value[index] = { ...saved.value.profiles[index] }
      notice.value = 'Saved to controller'
    })
  }

  async function toggleTournament() {
    if (!saved.value) return
    const next = !saved.value.tournament
    await run('mode', async () => {
      saved.value = await setTournament(next)
    })
  }

  async function startCalibration() {
    await run('calibrate', async () => {
      await calibrate()
      notice.value = 'Calibration complete'
    })
  }

  function discard() {
    if (saved.value) drafts.value[profile.value] = { ...saved.value.profiles[profile.value] }
    notice.value = ''
  }

  onMounted(reload)
  return {
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
  }
}
