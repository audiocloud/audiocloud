import { z } from 'zod'

const LoginIDSchema = z
.string()
.min(1, 'Enter a domain admin ID.')
.max(24, 'Invalid domain admin ID.')

const LoginPasswordSchema = z
.string()
.min(1, 'Enter a password.')
.max(24, 'Invalid domain admin ID.')

export const LoginFormSchema = z.object({
  id: LoginIDSchema,
  password: LoginPasswordSchema
}).strict()

export const NewAudioEngineMaintenanceSchema = z.object({
  engine_id: z.string().min(1),
  title: z.string().min(1),
  description: z.string().min(1),
  startTime: z.string().min(1),
  endTime: z.string().min(1)
}).strict()

export const ExtendMaintenanceSchema = z.object({
  engine_id: z.string().min(1),
  endTime: z.string().min(1)
}).strict()

export const NewDeviceMaintenanceSchema = z.object({
  device_id: z.string().min(1),
  title: z.string().min(1),
  description: z.string().min(1),
  startTime: z.string().min(1),
  endTime: z.string().min(1)
}).strict()