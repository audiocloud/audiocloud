import { useEffect, useState } from "react"
import axios, { AxiosRequestConfig } from 'axios'

export const useDeviceConfig = (ip: string) => {

  const [config, setConfig] = useState<Record<string, any> | null>(null)

  const fetchConfig = async () => {
    const config: AxiosRequestConfig = {
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*'
      }
    }
    try {
      const res = await axios.get(
        `http://${ip}:3000/config`,
        config
      )
      setConfig(res.data)
    } catch (error) {
      setConfig({ error: error})
      console.log(error)
    }
  }

  // useEffects

  useEffect(() => {
    if (ip !== '') fetchConfig()
  }, [ip])

  return config
}