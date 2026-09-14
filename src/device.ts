import { invoke } from '@tauri-apps/api/core'

export interface MotionSettings {
  enabled: boolean
  activationKey: number
  activationMode: number
  sensitivity: number
  deadzone: number
  mapping: number
}

export interface ControllerSettings {
  tournament: boolean
  reconnectRequired: boolean
  profiles: MotionSettings[]
}

export const activationButtons = [
  { label: 'LT', value: 0x4000 },
  { label: 'RT', value: 0x8000 },
  { label: 'LB', value: 0x400 },
  { label: 'RB', value: 0x800 },
  { label: 'A', value: 0x2000 },
  { label: 'B', value: 0x1000 },
  { label: 'X', value: 0x10 },
  { label: 'Y', value: 0x20 },
  { label: 'LS', value: 0x2 },
  { label: 'RS', value: 0x4 },
  { label: 'PL', value: 0x2000000 },
  { label: 'PR', value: 0x4000000 },
  { label: 'L4', value: 0x200000 },
  { label: 'R4', value: 0x40000000 },
  { label: 'D-pad ↑', value: 0x200 },
  { label: 'D-pad ↓', value: 0x100 },
  { label: 'D-pad ←', value: 0x80 },
  { label: 'D-pad →', value: 0x40 },
  { label: 'View', value: 0x8 },
  { label: 'Menu', value: 0x1 },
]

export const readSettings = () => invoke<ControllerSettings>('read_settings')
export const saveMotion = (profile: number, settings: MotionSettings) =>
  invoke<ControllerSettings>('save_motion', { profile, settings })
export const setTournament = (enabled: boolean) =>
  invoke<ControllerSettings>('set_tournament', { enabled })
export const calibrate = () => invoke<void>('calibrate')
