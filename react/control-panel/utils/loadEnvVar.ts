export const loadEnvVar = (value: string | undefined, name: string): string => {	
  if (value === undefined) throw Error(`Missing environment variable ${name}`)	
  return value
}