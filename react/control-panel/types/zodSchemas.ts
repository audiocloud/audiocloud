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